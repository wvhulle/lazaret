CREATE TABLE trusted_contents (
    content_sha256 TEXT PRIMARY KEY REFERENCES file_contents (content_sha256),
    trusted_at_unix INTEGER NOT NULL
);
CREATE TABLE ignored_signatures (
    signature_name TEXT PRIMARY KEY
        CHECK (length(signature_name) BETWEEN 3 AND 200
            AND signature_name NOT GLOB '*[^A-Za-z0-9._-]*'
            AND signature_name NOT GLOB 'Heuristic.*'
            AND signature_name NOT GLOB 'Heuristics.*'
            AND signature_name NOT GLOB '*.UNOFFICIAL'),
    ignored_at_unix INTEGER NOT NULL
);
