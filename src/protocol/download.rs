use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Download {
    command_key: XmlSafeString,
    file_type: XmlSafeString,
    url: XmlSafeString,
    username: XmlSafeString,
    password: XmlSafeString,
    file_size: u32,
    target_filename: XmlSafeString,
    delay_seconds: u32,
    success_url: XmlSafeString,
    failure_url: XmlSafeString,
}

impl Download {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        command_key: &str,
        file_type: &str,
        url: &str,
        username: &str,
        password: &str,
        file_size: u32,
        target_filename: &str,
        delay_seconds: u32,
        success_url: &str,
        failure_url: &str,
    ) -> Self {
        Download {
            command_key: command_key.into(),
            file_type: file_type.into(),
            url: url.into(),
            username: username.into(),
            password: password.into(),
            file_size,
            target_filename: target_filename.into(),
            delay_seconds,
            success_url: success_url.into(),
            failure_url: failure_url.into(),
        }
    }
    /// Generate XML for `Download`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Download")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        write_simple(writer, "FileType", self.file_type.0.as_ref())?;
        write_simple(writer, "URL", self.url.0.as_ref())?;
        write_simple(writer, "Username", self.username.0.as_ref())?;
        write_simple(writer, "Password", self.password.0.as_ref())?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", self.target_filename.0.as_ref())?;
        write_simple(writer, "DelaySeconds", &self.delay_seconds.to_string())?;
        write_simple(writer, "SuccessURL", self.success_url.0.as_ref())?;
        write_simple(writer, "FailureURL", self.failure_url.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["Download", "CommandKey"] => self.command_key = characters.into(),
            ["Download", "FileType"] => self.file_type = characters.into(),
            ["Download", "URL"] => self.url = characters.into(),
            ["Download", "Username"] => self.username = characters.into(),
            ["Download", "Password"] => self.password = characters.into(),
            ["Download", "FileSize"] => self.file_size = parse_to_int(characters, 0),
            ["Download", "TargetFileName"] => self.target_filename = characters.into(),
            ["Download", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            ["Download", "SuccessURL"] => self.success_url = characters.into(),
            ["Download", "FailureURL"] => self.failure_url = characters.into(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Download {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            file_type: XmlSafeString::arbitrary(g),
            url: XmlSafeString::arbitrary(g),
            username: XmlSafeString::arbitrary(g),
            password: XmlSafeString::arbitrary(g),
            file_size: u32::arbitrary(g),
            target_filename: XmlSafeString::arbitrary(g),
            delay_seconds: u32::arbitrary(g),
            success_url: XmlSafeString::arbitrary(g),
            failure_url: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.file_type.clone(),
                self.url.clone(),
                self.username.clone(),
                self.password.clone(),
                self.file_size,
                self.target_filename.clone(),
                self.delay_seconds,
            )
                .shrink()
                .map(|(c, ft, u, un, pw, fs, tf, ds)| Download {
                    command_key: c,
                    file_type: ft,
                    url: u,
                    username: un,
                    password: pw,
                    file_size: fs,
                    target_filename: tf,
                    delay_seconds: ds,
                    success_url: XmlSafeString::new(),
                    failure_url: XmlSafeString::new(),
                }),
        )
    }
}
