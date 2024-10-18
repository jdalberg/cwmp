use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Download {
    command_key: String,
    file_type: String,
    url: String,
    username: String,
    password: String,
    file_size: u32,
    target_filename: String,
    delay_seconds: u32,
    success_url: String,
    failure_url: String,
}

impl Download {
    pub fn new(
        command_key: String,
        file_type: String,
        url: String,
        username: String,
        password: String,
        file_size: u32,
        target_filename: String,
        delay_seconds: u32,
        success_url: String,
        failure_url: String,
    ) -> Self {
        Download {
            command_key,
            file_type,
            url,
            username,
            password,
            file_size,
            target_filename,
            delay_seconds,
            success_url,
            failure_url,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Download")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        write_simple(writer, "FileType", &self.file_type)?;
        write_simple(writer, "URL", &self.url)?;
        write_simple(writer, "Username", &self.username)?;
        write_simple(writer, "Password", &self.password)?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", &self.target_filename)?;
        write_simple(writer, "DelaySeconds", &self.delay_seconds.to_string())?;
        write_simple(writer, "SuccessURL", &self.success_url)?;
        write_simple(writer, "FailureURL", &self.failure_url)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Download", "CommandKey"] => self.command_key = characters.to_string(),
            ["Download", "FileType"] => self.file_type = characters.to_string(),
            ["Download", "URL"] => self.url = characters.to_string(),
            ["Download", "Username"] => self.username = characters.to_string(),
            ["Download", "Password"] => self.password = characters.to_string(),
            ["Download", "FileSize"] => self.file_size = parse_to_int(characters, 0),
            ["Download", "TargetFileName"] => self.target_filename = characters.to_string(),
            ["Download", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            ["Download", "SuccessURL"] => self.success_url = characters.to_string(),
            ["Download", "FailureURL"] => self.failure_url = characters.to_string(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Download {
    fn arbitrary(g: &mut Gen) -> Self {
        Download::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::from(""),
            String::from(""),
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
                self.file_size.clone(),
                self.target_filename.clone(),
                self.delay_seconds.clone(),
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
                    success_url: String::from(""),
                    failure_url: String::from(""),
                }),
        )
    }
}
