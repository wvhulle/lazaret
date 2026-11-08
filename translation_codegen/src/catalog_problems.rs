use std::{fmt, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogProblem {
    UnreadablePath {
        path: PathBuf,
        reason: String,
    },
    InvalidLanguageDirectory {
        path: PathBuf,
    },
    MessageIdWithoutModule {
        path: PathBuf,
        line_number: usize,
        message_id: String,
    },
    MissingSourceLanguage {
        source_language_tag: String,
    },
    SyntaxError {
        path: PathBuf,
        line_number: usize,
        description: String,
    },
    DuplicateMessage {
        path: PathBuf,
        line_number: usize,
        message_id: String,
    },
    SourceMessageWithoutValue {
        path: PathBuf,
        line_number: usize,
        message_id: String,
    },
    SourceMessageWithAttributes {
        path: PathBuf,
        line_number: usize,
        message_id: String,
    },
    UnusableRustName {
        path: PathBuf,
        line_number: usize,
        fluent_name: String,
    },
    CollidingFunctionName {
        path: PathBuf,
        line_number: usize,
        function_name: String,
    },
    StaleTranslation {
        path: PathBuf,
        line_number: usize,
        message_id: String,
    },
    UnknownVariable {
        path: PathBuf,
        line_number: usize,
        message_id: String,
        variable_name: String,
    },
    SelectorOnTextVariable {
        path: PathBuf,
        line_number: usize,
        message_id: String,
        variable_name: String,
    },
    UnresolvedMessageReference {
        path: PathBuf,
        line_number: usize,
        message_id: String,
        referenced_id: String,
    },
    UnresolvedTermReference {
        path: PathBuf,
        line_number: usize,
        message_id: String,
        referenced_id: String,
    },
}

impl fmt::Display for CatalogProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnreadablePath { path, reason } => {
                write!(formatter, "{}: cannot be read: {reason}", path.display())
            }
            Self::InvalidLanguageDirectory { path } => write!(
                formatter,
                "{}: directory name is not a canonical BCP 47 language tag such as `pt-BR`",
                path.display()
            ),
            Self::MessageIdWithoutModule {
                path,
                line_number,
                message_id,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` needs a module prefix and a name, such as \
                 `scans-target-count`",
                path.display()
            ),
            Self::MissingSourceLanguage {
                source_language_tag,
            } => write!(
                formatter,
                "the source language directory `{source_language_tag}` is missing"
            ),
            Self::SyntaxError {
                path,
                line_number,
                description,
            } => write!(formatter, "{}:{line_number}: {description}", path.display()),
            Self::DuplicateMessage {
                path,
                line_number,
                message_id,
            } => write!(
                formatter,
                "{}:{line_number}: message `{message_id}` is defined twice",
                path.display()
            ),
            Self::SourceMessageWithoutValue {
                path,
                line_number,
                message_id,
            } => write!(
                formatter,
                "{}:{line_number}: source message `{message_id}` has no value",
                path.display()
            ),
            Self::SourceMessageWithAttributes {
                path,
                line_number,
                message_id,
            } => write!(
                formatter,
                "{}:{line_number}: source message `{message_id}` has attributes, give each text \
                 its own message instead",
                path.display()
            ),
            Self::UnusableRustName {
                path,
                line_number,
                fluent_name,
            } => write!(
                formatter,
                "{}:{line_number}: `{fluent_name}` does not convert to a usable Rust name",
                path.display()
            ),
            Self::CollidingFunctionName {
                path,
                line_number,
                function_name,
            } => write!(
                formatter,
                "{}:{line_number}: two messages both become `{function_name}`",
                path.display()
            ),
            Self::StaleTranslation {
                path,
                line_number,
                message_id,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` no longer exists in the source language",
                path.display()
            ),
            Self::UnknownVariable {
                path,
                line_number,
                message_id,
                variable_name,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` uses `${variable_name}`, which the source \
                 message does not provide",
                path.display()
            ),
            Self::SelectorOnTextVariable {
                path,
                line_number,
                message_id,
                variable_name,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` selects on `${variable_name}`, which the source \
                 message passes as text; select on it in the source message first",
                path.display()
            ),
            Self::UnresolvedMessageReference {
                path,
                line_number,
                message_id,
                referenced_id,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` references the unknown message `{referenced_id}`",
                path.display()
            ),
            Self::UnresolvedTermReference {
                path,
                line_number,
                message_id,
                referenced_id,
            } => write!(
                formatter,
                "{}:{line_number}: `{message_id}` references the unknown term `-{referenced_id}`",
                path.display()
            ),
        }
    }
}
