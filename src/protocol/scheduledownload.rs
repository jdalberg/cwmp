use std::io::Write;

use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError, TimeWindow, XmlSafeString};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleDownload {
    pub command_key: XmlSafeString,
    pub file_type: XmlSafeString,
    pub url: XmlSafeString,
    pub username: XmlSafeString,
    pub password: XmlSafeString,
    pub file_size: u32,
    pub target_filename: XmlSafeString,
    pub timewindow_list: Vec<TimeWindow>,
}

impl ScheduleDownload {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        command_key: &str,
        file_type: &str,
        url: &str,
        username: &str,
        password: &str,
        file_size: u32,
        target_filename: &str,
        timewindow_list: Vec<TimeWindow>,
    ) -> Self {
        Self {
            command_key: command_key.into(),
            file_type: file_type.into(),
            url: url.into(),
            username: username.into(),
            password: password.into(),
            file_size,
            target_filename: target_filename.into(),
            timewindow_list,
        }
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["ScheduleDownload", "TimeWindowList", "TimeWindowStruct"] = &path_pattern[..] {
            self.timewindow_list.push(TimeWindow::default());
        }
    }

    /// Generate XML for `ScheduleDownload`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ScheduleDownload")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        write_simple(writer, "FileType", self.file_type.0.as_ref())?;
        write_simple(writer, "URL", self.url.0.as_ref())?;
        write_simple(writer, "Username", self.username.0.as_ref())?;
        write_simple(writer, "Password", self.password.0.as_ref())?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", self.target_filename.0.as_ref())?;
        let ts = format!("cwmp:TimeWindowStruct[{}]", self.timewindow_list.len());
        writer
            .write(XmlEvent::start_element("TimeWindowList").attr("SOAP-ENC:arrayType", &ts[..]))?;

        for t in &self.timewindow_list {
            writer.write(XmlEvent::start_element("TimeWindowStruct"))?;
            write_simple(writer, "WindowStart", &t.window_start.to_string())?;
            write_simple(writer, "WindowEnd", &t.window_end.to_string())?;
            write_simple(writer, "WindowMode", t.window_mode.0.as_ref())?;
            write_simple(writer, "UserMessage", t.user_message.0.as_ref())?;
            write_simple(writer, "MaxRetries", &t.max_retries.to_string())?;
            writer.write(XmlEvent::end_element())?;
        }

        // TimeWindownList
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["ScheduleDownload", "CommandKey"] => {
                self.command_key = characters.into();
            }
            ["ScheduleDownload", "FileType"] => {
                self.file_type = characters.into();
            }
            ["ScheduleDownload", "URL"] => {
                self.url = characters.into();
            }
            ["ScheduleDownload", "Username"] => {
                self.username = characters.into();
            }
            ["ScheduleDownload", "Password"] => {
                self.password = characters.into();
            }
            ["ScheduleDownload", "FileSize"] => {
                self.file_size = parse_to_int(characters, 0);
            }
            ["ScheduleDownload", "TargetFileName"] => {
                self.target_filename = characters.into();
            }
            ["ScheduleDownload", "TimeWindowList", "TimeWindowStruct", key] => {
                if let Some(e) = self.timewindow_list.last_mut() {
                    match key {
                        "WindowStart" => e.window_start = parse_to_int(characters, 0),
                        "WindowEnd" => e.window_end = parse_to_int(characters, 0),
                        "WindowMode" => e.window_mode = characters.into(),
                        "UserMessage" => e.user_message = characters.into(),
                        "MaxRetries" => e.max_retries = parse_to_int(characters, 0),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ScheduleDownload {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            file_type: XmlSafeString::arbitrary(g),
            url: XmlSafeString::arbitrary(g),
            username: XmlSafeString::arbitrary(g),
            password: XmlSafeString::arbitrary(g),
            file_size: u32::arbitrary(g),
            target_filename: XmlSafeString::arbitrary(g),
            timewindow_list: Vec::<TimeWindow>::arbitrary(g),
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
                self.file_size.clone(),
                self.target_filename.clone(),
                self.timewindow_list.clone(),
            )
                .shrink()
                .map(|(c, f, u, un, pw, fs, tf, tl)| ScheduleDownload {
                    command_key: c,
                    file_type: f,
                    url: u,
                    username: un,
                    password: pw,
                    file_size: fs,
                    target_filename: tf,
                    timewindow_list: tl,
                }),
        )
    }
}
