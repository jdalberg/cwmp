use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Upload {
    pub command_key: String,
    pub file_type: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub delay_seconds: u32,
}

impl Upload {
    #[must_use]
    pub fn new(
        command_key: String,
        file_type: String,
        url: String,
        username: String,
        password: String,
        delay_seconds: u32,
    ) -> Self {
        Upload {
            command_key,
            file_type,
            url,
            username,
            password,
            delay_seconds,
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Upload")[..],
        ))?;
        writer.write(XmlEvent::start_element("CommandKey"))?;
        writer.write(&self.command_key[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("FileType"))?;
        writer.write(&self.file_type[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("URL"))?;
        writer.write(&self.url[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Username"))?;
        writer.write(&self.username[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Password"))?;
        writer.write(&self.password[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("DelaySeconds"))?;
        let s: String = self.delay_seconds.to_string();
        writer.write(&s[..])?;
        writer.write(XmlEvent::end_element())?;

        let e: XmlEvent = XmlEvent::end_element().into();
        writer.write(e)?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Upload", "CommandKey"] => self.command_key = characters.to_string(),
            ["Upload", "FileType"] => self.file_type = characters.to_string(),
            ["Upload", "URL"] => self.url = characters.to_string(),
            ["Upload", "Username"] => self.username = characters.to_string(),
            ["Upload", "Password"] => self.password = characters.to_string(),
            ["Upload", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Upload {
    fn arbitrary(g: &mut Gen) -> Self {
        Upload::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.file_type.clone(),
                self.url.clone(),
                self.username.clone(),
                self.password.clone(),
                self.delay_seconds.clone(),
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
