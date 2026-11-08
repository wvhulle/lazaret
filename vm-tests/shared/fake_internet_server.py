import argparse
import hashlib
import json
import ssl
import threading
from email.parser import BytesParser
from email.policy import HTTP
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlsplit

VIRUSTOTAL_ANALYSIS_PREFIX = "fixture-analysis/"
CLAMAV_SESSION_COOKIE = "_clamav-net_session=fixture-session-cookie"
CLAMAV_AUTHENTICITY_TOKEN = "fixture-authenticity-token"


class FakeInternetState:
    def __init__(self, state_directory, signature_directory):
        self.state_directory = Path(state_directory)
        self.signature_directory = Path(signature_directory)
        self.request_log_path = self.state_directory / "requests.jsonl"
        self.scenario_path = self.state_directory / "scenario.json"
        self.state_lock = threading.Lock()
        self.bazaar_uploaded_hashes = set()
        self.virustotal_uploaded_hashes = set()
        self.virustotal_analysis_polls = {}

    def scenario_overrides(self):
        try:
            return json.loads(self.scenario_path.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            return {}

    def append_request_record(self, request_record):
        with self.state_lock, self.request_log_path.open("a") as request_log:
            request_log.write(json.dumps(request_record) + "\n")


def decoded_request_body(request_handler):
    transfer_encoding = request_handler.headers.get("Transfer-Encoding", "").lower()
    if "chunked" in transfer_encoding:
        collected_chunks = bytearray()
        while True:
            chunk_size_line = request_handler.rfile.readline().strip()
            chunk_size = int(chunk_size_line.split(b";")[0], 16)
            if chunk_size == 0:
                while request_handler.rfile.readline().strip():
                    pass
                return bytes(collected_chunks)
            collected_chunks += request_handler.rfile.read(chunk_size)
            request_handler.rfile.readline()
    declared_length = int(request_handler.headers.get("Content-Length", "0") or "0")
    return request_handler.rfile.read(declared_length) if declared_length else b""


def summarised_form_fields(content_type, body_bytes):
    if content_type.startswith("multipart/form-data"):
        multipart_message = BytesParser(policy=HTTP).parsebytes(
            f"Content-Type: {content_type}\r\n\r\n".encode() + body_bytes
        )
        form_fields = {}
        for form_part in multipart_message.iter_parts():
            field_name = form_part.get_param("name", header="content-disposition")
            part_bytes = form_part.get_payload(decode=True) or b""
            uploaded_name = form_part.get_filename()
            if uploaded_name is None:
                form_fields[field_name] = part_bytes.decode("utf-8", "replace")
            else:
                form_fields[field_name] = {
                    "uploaded_name": uploaded_name,
                    "content_sha256": hashlib.sha256(part_bytes).hexdigest(),
                    "byte_size": len(part_bytes),
                }
        return form_fields
    if content_type.startswith("application/x-www-form-urlencoded"):
        return {
            field_name: field_values[0]
            for field_name, field_values in parse_qs(body_bytes.decode("utf-8", "replace")).items()
        }
    return {}


class FakeInternetHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    fake_internet_state = None

    def log_message(self, message_format, *message_arguments):
        print(f"{self.headers.get('Host', '?')} {message_format % message_arguments}", flush=True)

    def do_GET(self):
        self.answer_request(b"")

    def do_POST(self):
        self.answer_request(decoded_request_body(self))

    def answer_request(self, body_bytes):
        requested_host = self.headers.get("Host", "").split(":")[0]
        requested_url = urlsplit(self.path)
        content_type = self.headers.get("Content-Type", "")
        form_fields = summarised_form_fields(content_type, body_bytes)
        route_name, answer_status, answer_headers, answer_body = self.routed_answer(
            requested_host, requested_url, form_fields
        )
        scenario_override = self.fake_internet_state.scenario_overrides().get(route_name)
        if scenario_override is not None and self.override_applies(scenario_override, requested_url, form_fields):
            answer_status = scenario_override.get("status", answer_status)
            answer_body = scenario_override.get("body", answer_body)
            answer_headers = {"Content-Type": scenario_override.get("content_type", "application/json")}
        if isinstance(answer_body, (dict, list)):
            answer_body = json.dumps(answer_body)
        if isinstance(answer_body, str):
            answer_body = answer_body.encode()
        self.fake_internet_state.append_request_record(
            {
                "host": requested_host,
                "method": self.command,
                "path": requested_url.path,
                "query": requested_url.query,
                "headers": {header_name.lower(): header_value for header_name, header_value in self.headers.items()},
                "form_fields": form_fields,
                "body_byte_size": len(body_bytes),
                "route_name": route_name,
                "answer_status": answer_status,
            }
        )
        self.send_response(answer_status)
        for header_name, header_value in answer_headers.items():
            self.send_header(header_name, header_value)
        self.send_header("Content-Length", str(len(answer_body)))
        self.end_headers()
        self.wfile.write(answer_body)

    @staticmethod
    def override_applies(scenario_override, requested_url, form_fields):
        only_hash = scenario_override.get("only_hash")
        if only_hash is None:
            return True
        return only_hash in requested_url.path or only_hash == form_fields.get("hash")

    def routed_answer(self, requested_host, requested_url, form_fields):
        json_headers = {"Content-Type": "application/json"}
        if requested_host == "mb-api.abuse.ch":
            return self.bazaar_answer(form_fields, json_headers)
        if requested_host == "hashlookup.circl.lu":
            return "circl_lookup", 404, json_headers, {"message": "Non existing SHA-256"}
        if requested_host == "www.virustotal.com":
            return self.virustotal_answer(requested_url, form_fields, json_headers)
        if requested_host == "www.clamav.net":
            return self.clamav_answer(requested_url)
        if requested_host == "clamav-site.s3.amazonaws.com":
            return "clamav_sample_upload", 204, {}, b""
        if requested_host == "signatures.fixture.test":
            return self.signature_answer(requested_url)
        if requested_host == "urlhaus-api.abuse.ch":
            return self.urlhaus_answer(requested_url)
        return "unknown_host", 404, {"Content-Type": "text/plain"}, "unknown host"

    def bazaar_answer(self, form_fields, json_headers):
        bazaar_state = self.fake_internet_state
        if "file" in form_fields:
            uploaded_hash = form_fields["file"]["content_sha256"]
            with bazaar_state.state_lock:
                bazaar_state.bazaar_uploaded_hashes.add(uploaded_hash)
            return "bazaar_upload", 200, json_headers, {"query_status": "inserted"}
        requested_query = form_fields.get("query")
        if requested_query == "get_clamavinfo":
            return (
                "bazaar_get_clamavinfo",
                200,
                json_headers,
                {
                    "query_status": "ok",
                    "data": [
                        {
                            "sha256_hash": "0" * 64,
                            "signature": "FixtureLoader",
                            "first_seen": "2026-01-02 03:04:05",
                            "tags": ["fixture", "loader"],
                        }
                    ],
                },
            )
        if requested_query == "get_info":
            requested_hash = form_fields.get("hash", "")
            if requested_hash in bazaar_state.bazaar_uploaded_hashes:
                return (
                    "bazaar_get_info",
                    200,
                    json_headers,
                    {
                        "query_status": "ok",
                        "data": [{"sha256_hash": requested_hash, "signature": "FixtureLoader", "tags": []}],
                    },
                )
            return "bazaar_get_info", 200, json_headers, {"query_status": "hash_not_found"}
        return "bazaar_unknown_query", 200, json_headers, {"query_status": "unknown_query"}

    def virustotal_answer(self, requested_url, form_fields, json_headers):
        virustotal_state = self.fake_internet_state
        request_path = requested_url.path
        if request_path == "/api/v3/files" and "file" in form_fields:
            uploaded_hash = form_fields["file"]["content_sha256"]
            with virustotal_state.state_lock:
                virustotal_state.virustotal_uploaded_hashes.add(uploaded_hash)
            return (
                "virustotal_upload",
                200,
                json_headers,
                {"data": {"type": "analysis", "id": f"{VIRUSTOTAL_ANALYSIS_PREFIX}{uploaded_hash}=="}},
            )
        if request_path.startswith("/api/v3/files/"):
            return (
                "virustotal_file_report",
                404,
                json_headers,
                {"error": {"code": "NotFoundError", "message": "File not found"}},
            )
        if request_path.startswith("/api/v3/analyses/"):
            analysis_id = request_path.removeprefix("/api/v3/analyses/")
            with virustotal_state.state_lock:
                poll_count = virustotal_state.virustotal_analysis_polls.get(analysis_id, 0) + 1
                virustotal_state.virustotal_analysis_polls[analysis_id] = poll_count
            analysis_status = "queued" if poll_count == 1 else "completed"
            return (
                "virustotal_analysis",
                200,
                json_headers,
                {
                    "data": {
                        "id": analysis_id,
                        "attributes": {
                            "status": analysis_status,
                            "stats": {"malicious": 41, "suspicious": 2, "undetected": 20, "harmless": 0},
                        },
                    }
                },
            )
        return "virustotal_unknown_path", 404, json_headers, {"error": {"code": "NotFoundError"}}

    def clamav_answer(self, requested_url):
        request_path = requested_url.path
        if request_path in ("/reports/fp", "/reports/malware"):
            return (
                "clamav_report_form",
                200,
                {"Content-Type": "text/html", "Set-Cookie": f"{CLAMAV_SESSION_COOKIE}; path=/; HttpOnly"},
                f'<form><input type="hidden" name="authenticity_token" value="{CLAMAV_AUTHENTICITY_TOKEN}" /></form>',
            )
        if request_path == "/presigned":
            return (
                "clamav_presigned",
                200,
                {"Content-Type": "application/json"},
                {
                    "key": "uploads/fixturesubmission-0001",
                    "acl": "private",
                    "policy": "fixture-policy",
                    "x-amz-meta-original-filename": "clamsubmit",
                    "x-amz-credential": "fixture-credential",
                    "x-amz-algorithm": "AWS4-HMAC-SHA256",
                    "x-amz-date": "20260101T000000Z",
                    "x-amz-signature": "fixture-signature",
                },
            )
        if request_path == "/reports/submit":
            return "clamav_submit", 302, {"Location": "https://www.clamav.net/reports/success"}, b""
        return "clamav_unknown_path", 404, {"Content-Type": "text/plain"}, "not found"

    def signature_answer(self, requested_url):
        release_name = self.fake_internet_state.scenario_overrides().get("signature_release", "v1")
        signature_path = self.fake_internet_state.signature_directory / release_name / Path(requested_url.path).name
        if not signature_path.is_file():
            return "signature_download", 404, {"Content-Type": "text/plain"}, "not found"
        return "signature_download", 200, {"Content-Type": "application/octet-stream"}, signature_path.read_bytes()

    def urlhaus_answer(self, requested_url):
        file_name = Path(requested_url.path).name
        signature_path = self.fake_internet_state.signature_directory / "urlhaus" / file_name
        if not signature_path.is_file():
            return "urlhaus_download", 404, {"Content-Type": "text/plain"}, "not found"
        return "urlhaus_download", 200, {"Content-Type": "application/octet-stream"}, signature_path.read_bytes()


def serve_fake_internet():
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument("--certificate", required=True)
    argument_parser.add_argument("--private-key", required=True)
    argument_parser.add_argument("--state-directory", required=True)
    argument_parser.add_argument("--signature-directory", required=True)
    argument_parser.add_argument("--port", type=int, default=443)
    parsed_arguments = argument_parser.parse_args()

    FakeInternetHandler.fake_internet_state = FakeInternetState(
        parsed_arguments.state_directory, parsed_arguments.signature_directory
    )
    tls_context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    tls_context.load_cert_chain(parsed_arguments.certificate, parsed_arguments.private_key)
    fake_internet_server = ThreadingHTTPServer(("127.0.0.1", parsed_arguments.port), FakeInternetHandler)
    fake_internet_server.socket = tls_context.wrap_socket(fake_internet_server.socket, server_side=True)
    fake_internet_server.serve_forever()


if __name__ == "__main__":
    serve_fake_internet()
