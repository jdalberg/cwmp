use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{parse_to_int, write_simple, GenerateError, OptionStruct};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetOptionsResponse {
    pub option_list: Vec<OptionStruct>,
}

impl GetOptionsResponse {
    #[must_use]
    pub fn new(option_list: Vec<OptionStruct>) -> Self {
        GetOptionsResponse { option_list }
    }
    /// Generate XML for `GetOptionsResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetOptionsResponse"))?;
            format!("cwmp:OptionStruct[{}]", self.option_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetOptionsResponse"))?;
            format!("OptionStruct[{}]", self.option_list.len())
        };

        writer.write(XmlEvent::start_element("OptionList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for o in &self.option_list {
            writer.write(XmlEvent::start_element("OptionStruct"))?;
            write_simple(writer, "OptionName", &o.option_name)?;
            write_simple(writer, "VoucherSN", &o.voucher_sn)?;
            write_simple(writer, "State", &o.state.to_string())?;
            write_simple(writer, "Mode", &o.mode)?;
            if let Some(dt) = o.start_date {
                write_simple(writer, "StartDate", &dt.to_rfc3339())?;
            }
            if let Some(dt) = o.expiration_date {
                write_simple(writer, "ExpirationDate", &dt.to_rfc3339())?;
            }
            write_simple(writer, "IsTransferable", &o.is_transferable.to_string())?;

            writer.write(XmlEvent::end_element())?;
        }

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
        if let ["GetOptionsResponse", "OptionList", "OptionStruct"] = &path_pattern[..] {
            self.option_list.push(OptionStruct::default());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["GetOptionsResponse", "OptionList", "OptionStruct", key] = *path {
            if let Some(last) = self.option_list.last_mut() {
                match key {
                    "OptionName" => last.option_name = characters.to_string(),
                    "VoucherSN" => last.voucher_sn = characters.to_string(),
                    "State" => last.state = parse_to_int(characters, 0),
                    "Mode" => last.mode = characters.to_string(),
                    "StartDate" => {
                        if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                            last.start_date = Some(dt);
                        }
                    }
                    "ExpirationDate" => {
                        if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                            last.expiration_date = Some(dt);
                        }
                    }
                    "IsTransferable" => last.is_transferable = parse_to_int(characters, 0),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetOptionsResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        GetOptionsResponse::new(Vec::<OptionStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.option_list
                .clone()
                .shrink()
                .map(|ol| GetOptionsResponse { option_list: ol }),
        )
    }
}
