use super::{parse_to_int, write_simple, GenerateError, ParameterInfoStruct};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterNamesResponse {
    parameter_list: Vec<ParameterInfoStruct>,
}

impl GetParameterNamesResponse {
    #[must_use]
    pub fn new(parameter_list: Vec<ParameterInfoStruct>) -> Self {
        GetParameterNamesResponse { parameter_list }
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct"] =
            &path_pattern[..]
        {
            self.parameter_list.push(ParameterInfoStruct::default());
        }
    }
    /// Generate XML for `GetParameterNamesResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetParameterNamesResponse"))?;
            format!("cwmp:ParameterInfoStruct[{}]", self.parameter_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetParameterNamesResponse"))?;
            format!("ParameterInfoStruct[{}]", self.parameter_list.len())
        };

        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in &self.parameter_list {
            writer.write(XmlEvent::start_element("ParameterInfoStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            write_simple(writer, "Writable", &p.writable.to_string())?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Name"] => {
                if let Some(e) = self.parameter_list.last_mut() {
                    e.name = characters.to_string();
                }
            }
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Writable"] => {
                if let Some(e) = self.parameter_list.last_mut() {
                    e.writable = parse_to_int(characters, 0);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterNamesResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        GetParameterNamesResponse::new(Vec::<ParameterInfoStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameter_list
                .clone()
                .shrink()
                .map(|pl| GetParameterNamesResponse { parameter_list: pl }),
        )
    }
}
