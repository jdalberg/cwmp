use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[cfg(test)]
use super::gen_utc_date;

use super::{
    cwmp_prefix, extract_attribute, parse_to_int, write_simple, DeviceId, EventStruct,
    GenerateError, ParameterValue,
};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Inform {
    pub device_id: DeviceId,
    pub event: Vec<EventStruct>,
    pub max_envelopes: u32,
    pub current_time: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub parameter_list: Vec<ParameterValue>,
}

impl Inform {
    #[must_use]
    pub fn new(
        device_id: DeviceId,
        event: Vec<EventStruct>,
        max_envelopes: u32,
        current_time: DateTime<Utc>,
        retry_count: u32,
        parameter_list: Vec<ParameterValue>,
    ) -> Self {
        Inform {
            device_id,
            event,
            max_envelopes,
            current_time: Some(current_time),
            retry_count,
            parameter_list,
        }
    }

    /// Generate XML for `Inform`
    ///     
    /// # Errors
    /// Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Inform")[..],
        ))?;
        writer.write(XmlEvent::start_element("DeviceId"))?;
        write_simple(
            writer,
            "Manufacturer",
            self.device_id.manufacturer.0.as_ref(),
        )?;
        write_simple(writer, "OUI", self.device_id.oui.0.as_ref())?;
        write_simple(
            writer,
            "ProductClass",
            self.device_id.product_class.0.as_ref(),
        )?;
        write_simple(
            writer,
            "SerialNumber",
            self.device_id.serial_number.0.as_ref(),
        )?;
        writer.write(XmlEvent::end_element())?;

        let ss = format!("cwmp:EventStruct[{}]", self.event.len());

        writer.write(XmlEvent::start_element("Event").attr("SOAP-ENC:arrayType", &ss))?;

        for e in &self.event {
            writer.write(XmlEvent::start_element("EventStruct"))?;
            write_simple(writer, "EventCode", e.event_code.0.as_ref())?;
            write_simple(writer, "CommandKey", e.command_key.0.as_ref())?;
            writer.write(XmlEvent::end_element())?;
        }
        // Event
        writer.write(XmlEvent::end_element())?;

        write_simple(writer, "MaxEnvelopes", &self.max_envelopes.to_string())?;
        if let Some(dt) = self.current_time {
            write_simple(writer, "CurrentTime", &dt.to_rfc3339())?;
        }
        write_simple(writer, "RetryCount", &self.retry_count.to_string())?;

        let pls = format!("cwmp:ParameterValueStruct[{}]", self.parameter_list.len());
        writer.write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pls))?;

        for p in &self.parameter_list {
            writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
            write_simple(writer, "Name", p.name.0.as_ref())?;
            writer.write(XmlEvent::start_element("Value").attr("xsi:type", p.r#type.0.as_ref()))?;
            writer.write(p.value.0.as_ref())?;
            writer.write(XmlEvent::end_element())?; // Value
            writer.write(XmlEvent::end_element())?; // ParameterValueStruct
        }

        // ParameterList
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Inform", "Event", "EventStruct"] => self.event.push(EventStruct::default()),
            ["Inform", "ParameterList", "ParameterValueStruct"] => {
                self.parameter_list.push(ParameterValue::default());
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                if let Some(e) = self.parameter_list.last_mut() {
                    e.r#type = extract_attribute(attributes, "type");
                }
            }
            _ => {}
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["Inform", "DeviceId", "Manufacturer"] => {
                self.device_id.manufacturer = characters.into();
            }
            ["Inform", "DeviceId", "OUI"] => self.device_id.oui = characters.into(),
            ["Inform", "DeviceId", "ProductClass"] => {
                self.device_id.product_class = characters.into();
            }
            ["Inform", "DeviceId", "SerialNumber"] => {
                self.device_id.serial_number = characters.into();
            }
            ["Inform", "Event", "EventStruct", key] => {
                if let Some(e) = self.event.last_mut() {
                    match key {
                        "EventCode" => e.event_code = characters.into(),
                        "CommandKey" => e.command_key = characters.into(),
                        _ => {}
                    }
                }
            }
            ["Inform", "MaxEnvelopes"] => self.max_envelopes = parse_to_int(characters, 0),
            ["Inform", "RetryCount"] => self.retry_count = parse_to_int(characters, 0),
            ["Inform", "CurrentTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.current_time = Some(dt);
                }
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Name"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.name = characters.into();
                }
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.value = characters.into();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Inform {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            device_id: DeviceId::arbitrary(g),
            event: Vec::<EventStruct>::arbitrary(g),
            max_envelopes: u32::arbitrary(g),
            current_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            retry_count: u32::arbitrary(g),
            parameter_list: Vec::<ParameterValue>::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.device_id.clone(),
                self.event.clone(),
                self.max_envelopes,
                self.retry_count,
                self.parameter_list.clone(),
            )
                .shrink()
                .map(|(d, e, m, r, p)| Inform {
                    device_id: d,
                    event: e,
                    max_envelopes: m,
                    current_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    retry_count: r,
                    parameter_list: p,
                }),
        )
    }
}
