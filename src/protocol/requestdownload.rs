use std::io::Write;

use super::{cwmp_prefix, write_simple, ArgStruct, GenerateError};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RequestDownload {
    pub file_type: String,
    pub file_type_arg: Vec<ArgStruct>,
}

impl RequestDownload {
    #[must_use]
    pub fn new(file_type: String, file_type_arg: Vec<ArgStruct>) -> Self {
        RequestDownload {
            file_type,
            file_type_arg,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "RequestDownload")[..],
        ))?;
        write_simple(writer, "FileType", &self.file_type)?;
        let argss = format!("cwmp:ArgStruct[{}]", self.file_type_arg.len());
        writer
            .write(XmlEvent::start_element("FileTypeArg").attr("SOAP-ENC:arrayType", &argss[..]))?;

        for a in &self.file_type_arg {
            writer.write(XmlEvent::start_element("ArgStruct"))?;
            write_simple(writer, "Name", &a.name)?;
            write_simple(writer, "Value", &a.value)?;
            writer.write(XmlEvent::end_element())?;
        }

        // FileTypeArg
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["RequestDownload", "FileTypeArg", "ArgStruct"] = &path_pattern[..] {
            self.file_type_arg.push(ArgStruct::default());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["RequestDownload", "FileType"] => {
                self.file_type = characters.to_string();
            }
            ["RequestDownload", "FileTypeArg", "ArgStruct", key] => {
                if let Some(e) = self.file_type_arg.last_mut() {
                    match key {
                        "Name" => e.name = characters.to_string(),
                        "Value" => e.value = characters.to_string(),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for RequestDownload {
    fn arbitrary(g: &mut Gen) -> Self {
        RequestDownload::new(String::arbitrary(g), Vec::<ArgStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.file_type.clone(), self.file_type_arg.clone())
                .shrink()
                .map(|(ft, fta)| RequestDownload {
                    file_type: ft,
                    file_type_arg: fta,
                }),
        )
    }
}
