use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Upload {
    pub command_key: XmlSafeString,
    pub file_type: XmlSafeString,
    pub url: XmlSafeString,
    pub username: XmlSafeString,
    pub password: XmlSafeString,
    pub delay_seconds: u32,
}

impl Upload {
    #[must_use]
    pub fn new(
        command_key: &str,
        file_type: &str,
        url: &str,
        username: &str,
        password: &str,
        delay_seconds: u32,
    ) -> Self {
        Upload {
            command_key: command_key.into(),
            file_type: file_type.into(),
            url: url.into(),
            username: username.into(),
            password: password.into(),
            delay_seconds,
        }
    }

    /// Generate XML for `Upload`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Upload")[..],
        ))?;
        writer.write(XmlEvent::start_element("CommandKey"))?;
        writer.write(self.command_key.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("FileType"))?;
        writer.write(self.file_type.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("URL"))?;
        writer.write(self.url.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Username"))?;
        writer.write(self.username.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Password"))?;
        writer.write(self.password.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("DelaySeconds"))?;
        let s: String = self.delay_seconds.to_string();
        writer.write(&s[..])?;
        writer.write(XmlEvent::end_element())?;

        let e: XmlEvent = XmlEvent::end_element().into();
        writer.write(e)?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["Upload", "CommandKey"] => self.command_key = characters.into(),
            ["Upload", "FileType"] => self.file_type = characters.into(),
            ["Upload", "URL"] => self.url = characters.into(),
            ["Upload", "Username"] => self.username = characters.into(),
            ["Upload", "Password"] => self.password = characters.into(),
            ["Upload", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Upload {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            file_type: XmlSafeString::arbitrary(g),
            url: XmlSafeString::arbitrary(g),
            username: XmlSafeString::arbitrary(g),
            password: XmlSafeString::arbitrary(g),
            delay_seconds: u32::arbitrary(g),
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
                self.delay_seconds,
            )
                .shrink()
                .map(|(c, f, u, un, pw, ds)| Upload {
                    command_key: c,
                    file_type: f,
                    url: u,
                    username: un,
                    password: pw,
                    delay_seconds: ds,
                }),
        )
    }
}
