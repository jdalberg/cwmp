use std::io::Write;

use super::{cwmp_prefix, write_empty_tag, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ChangeDUStateResponse;

impl ChangeDUStateResponse {
    /// Generate XML for `ChangeDUStateResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to writer will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "ChangeDUStateResponse")[..])?;
        Ok(())
    }
}
