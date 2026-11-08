use std::ops::RangeInclusive;

const SHA256_HEX_LENGTH: usize = 64;
const SIGNATURE_NAME_LENGTHS: RangeInclusive<usize> = 3..=200;
const UNOFFICIAL_SUFFIX: &str = ".UNOFFICIAL";
const ENGINE_HEURISTIC_PREFIXES: [&str; 2] = ["Heuristics.", "Heuristic."];

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum TrustEntryRefusal {
    #[error("the SHA-256 hash must be 64 hexadecimal characters")]
    HashMalformed,
    #[error("the signature name may only hold letters, digits, dots, dashes and underscores")]
    SignatureNameMalformed,
    #[error(
        "{0} is an engine heuristic that matches many unrelated files; trust the file by its hash \
         instead"
    )]
    HeuristicSignature(String),
}

pub fn validated_sha256(hash_text: &str) -> Result<String, TrustEntryRefusal> {
    let hash_text = hash_text.trim();
    if hash_text.len() == SHA256_HEX_LENGTH
        && hash_text
            .bytes()
            .all(|hash_byte| hash_byte.is_ascii_hexdigit())
    {
        Ok(hash_text.to_ascii_lowercase())
    } else {
        Err(TrustEntryRefusal::HashMalformed)
    }
}

pub fn signature_without_unofficial_suffix(detected_signature_name: &str) -> &str {
    let trimmed_name = detected_signature_name.trim();
    trimmed_name
        .strip_suffix(UNOFFICIAL_SUFFIX)
        .unwrap_or(trimmed_name)
}

pub fn is_engine_heuristic(signature_name: &str) -> bool {
    ENGINE_HEURISTIC_PREFIXES
        .iter()
        .any(|heuristic_prefix| signature_name.starts_with(heuristic_prefix))
}

pub fn validated_signature_name(name_text: &str) -> Result<String, TrustEntryRefusal> {
    let signature_name = signature_without_unofficial_suffix(name_text);
    let well_formed = SIGNATURE_NAME_LENGTHS.contains(&signature_name.len())
        && signature_name
            .bytes()
            .all(|name_byte| name_byte.is_ascii_alphanumeric() || b"._-".contains(&name_byte))
        && signature_name
            .bytes()
            .next()
            .is_some_and(|first_byte| first_byte.is_ascii_alphanumeric())
        && !signature_name.contains("..");
    if !well_formed {
        return Err(TrustEntryRefusal::SignatureNameMalformed);
    }
    if is_engine_heuristic(signature_name) {
        return Err(TrustEntryRefusal::HeuristicSignature(
            signature_name.to_owned(),
        ));
    }
    Ok(signature_name.to_owned())
}

#[cfg(test)]
mod trust_entry_names_tests {
    use super::*;

    #[test]
    fn hashes_are_normalised_to_lowercase_and_must_be_complete() {
        assert_eq!(validated_sha256(&"AB".repeat(32)), Ok("ab".repeat(32)));
        assert_eq!(
            validated_sha256(&"ab".repeat(31)),
            Err(TrustEntryRefusal::HashMalformed)
        );
        assert_eq!(
            validated_sha256(&format!("{}zz", "ab".repeat(31))),
            Err(TrustEntryRefusal::HashMalformed)
        );
    }

    #[test]
    fn signature_names_lose_the_unofficial_suffix_and_refuse_heuristics() {
        assert_eq!(
            validated_signature_name("Sanesecurity.Foo.UNOFFICIAL"),
            Ok("Sanesecurity.Foo".to_owned())
        );
        assert_eq!(
            validated_signature_name("Heuristics.Encrypted.Zip"),
            Err(TrustEntryRefusal::HeuristicSignature(
                "Heuristics.Encrypted.Zip".to_owned()
            ))
        );
        assert_eq!(
            validated_signature_name("Win.Test\nEICAR"),
            Err(TrustEntryRefusal::SignatureNameMalformed)
        );
        assert_eq!(
            validated_signature_name("Win..Test"),
            Err(TrustEntryRefusal::SignatureNameMalformed)
        );
    }
}
