use thiserror::Error;

use crate::protocol::GenerateError;

#[derive(Debug, Error)]
pub enum Cwmp {
    #[error("Parse error: {0}")]
    ParseError(#[from] xml::reader::Error),
    #[error("Generate error: {0}")]
    GenerateError(GenerateError),
}
