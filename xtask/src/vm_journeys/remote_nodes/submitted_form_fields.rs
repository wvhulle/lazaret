use std::{collections::BTreeMap, convert::Infallible};

use axum::body::Bytes;
use sha2::{Digest as _, Sha256};

use crate::vm_journeys::remote_nodes::fake_internet_protocol::{FormField, UploadedFile};

async fn multipart_fields(
    body_bytes: Bytes,
    boundary: String,
) -> Result<BTreeMap<String, FormField>, multer::Error> {
    let body_stream = futures_util::stream::once(async move { Ok::<_, Infallible>(body_bytes) });
    let mut multipart_body = multer::Multipart::new(body_stream, boundary);
    let mut form_fields = BTreeMap::new();
    while let Some(multipart_field) = multipart_body.next_field().await? {
        let field_name = multipart_field.name().unwrap_or_default().to_owned();
        let uploaded_name = multipart_field.file_name().map(str::to_owned);
        let part_bytes = multipart_field.bytes().await?;
        let form_field = match uploaded_name {
            Some(uploaded_name) => FormField::UploadedFileField(UploadedFile {
                uploaded_name,
                content_sha256: hex::encode(Sha256::digest(&part_bytes)),
                byte_size: part_bytes.len() as u64,
            }),
            None => FormField::TextField(String::from_utf8_lossy(&part_bytes).into_owned()),
        };
        form_fields.entry(field_name).or_insert(form_field);
    }
    Ok(form_fields)
}

fn url_encoded_fields(body_bytes: &[u8]) -> BTreeMap<String, FormField> {
    form_urlencoded::parse(body_bytes).fold(
        BTreeMap::new(),
        |mut collected_fields, (field_name, field_value)| {
            collected_fields
                .entry(field_name.into_owned())
                .or_insert_with(|| FormField::TextField(field_value.into_owned()));
            collected_fields
        },
    )
}

pub async fn summarised_form_fields(
    content_type: &str,
    body_bytes: Bytes,
) -> BTreeMap<String, FormField> {
    if content_type.starts_with("application/x-www-form-urlencoded") {
        return url_encoded_fields(&body_bytes);
    }
    let Ok(boundary) = multer::parse_boundary(content_type) else {
        return BTreeMap::new();
    };
    multipart_fields(body_bytes, boundary)
        .await
        .unwrap_or_else(|multipart_failure| {
            eprintln!("an unreadable multipart body: {multipart_failure}");
            BTreeMap::new()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocking_form_fields(
        content_type: &str,
        body_bytes: &'static [u8],
    ) -> BTreeMap<String, FormField> {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("a test runtime")
            .block_on(summarised_form_fields(
                content_type,
                Bytes::from_static(body_bytes),
            ))
    }

    #[test]
    fn multipart_upload_keeps_text_fields_and_hashes_files() {
        let form_fields = blocking_form_fields(
            "multipart/form-data; boundary=edge",
            b"--edge\r\nContent-Disposition: form-data; name=\"json_data\"\r\n\r\n{\"anonymous\":1}\r\n--edge\r\nContent-Disposition: form-data; name=\"file\"; filename=\"abc\"\r\nContent-Type: application/octet-stream\r\n\r\nEICAR\r\n--edge--\r\n",
        );
        assert!(matches!(
            form_fields.get("json_data"),
            Some(FormField::TextField(json_text)) if json_text == "{\"anonymous\":1}"
        ));
        assert!(matches!(
            form_fields.get("file"),
            Some(FormField::UploadedFileField(uploaded_file))
                if uploaded_file.uploaded_name == "abc"
                    && uploaded_file.byte_size == 5
                    && uploaded_file.content_sha256 == hex::encode(Sha256::digest(b"EICAR"))
        ));
    }

    #[test]
    fn url_encoded_form_keeps_the_first_value() {
        let form_fields = blocking_form_fields(
            "application/x-www-form-urlencoded",
            b"query=get_clamavinfo&clamav=Win.Fixture&query=later",
        );
        assert!(matches!(
            form_fields.get("query"),
            Some(FormField::TextField(query_text)) if query_text == "get_clamavinfo"
        ));
    }
}
