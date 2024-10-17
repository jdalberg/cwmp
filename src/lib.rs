extern crate xml;

use std::error::Error;
use xml::reader::{ParserConfig, XmlEvent};

// import the protocol defs into global scope
use protocol::{Envelope, State};
pub mod protocol;

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md");

// using xml-rs and serde did not seem viable due to the chaotic nature of
// vendors
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct

/// parse a CWMP XML envelope and convert it to a rust struct
/// 
/// # Errors
///    Returns a `core::Error` if the envelope cannot be parsed from the XML
pub fn parse(xml: &str) -> Result<Envelope, Box<dyn Error>> {
    parse_bytes(xml.as_bytes())
}

/// # Errors
///    Returns a `core::Error` if the envelope cannot be parsed from the XML
pub fn parse_bytes(xml: &[u8]) -> Result<Envelope, Box<dyn Error>> {
     let config = ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(true);
    let parser = config.create_reader(xml);
    let mut state: State = State::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                ref name,
                ref attributes,
                ref namespace,
            }) => {
                // the cwmp version is part of the namespaces
                // call the start handler for the element name
                state.start_handler(name, attributes, namespace);
            }
            Ok(XmlEvent::EndElement { ref name }) => {
                state.end_handler(name);
            }
            Ok(XmlEvent::Characters(ref s)) => {
                // store the last seen text in the state.
                state.characters(s);
            }
            Err(e) => {
                state.error = Some(Box::new(e));
                break;
            }
            _ => {}
        }
    }
    match state.error {
        None => Ok(state.envelope),
        Some(b) => Err(b),
    }
}

/// # Errors
///    Returns a `protocol::GenerateError` if the envelope cannot be converted to XML
pub fn generate(envelope: &Envelope) -> Result<String, protocol::GenerateError> {
    envelope.generate()
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
mod tests {
    use crate::protocol::*;

    use super::*;
    use protocol::Envelope;
    extern crate quickcheck;

    fn contains_control_characters(s: &str) -> bool {
        s.chars().any(|c| c.is_control())
    }
    fn sanitize_xml(s: &str) -> String {
        s.chars().filter(|&c| !c.is_control() && c != '\u{FFFF}' && c != '\u{FFFE}' && c != '\0').collect()
    }

    #[quickcheck]
    fn gen_and_parse(e: Envelope) -> bool {
        match generate(&e) {
            
            Ok(mut xml) => {
                // Sanitize the XML before parsing
                xml = sanitize_xml(&xml);
                if contains_control_characters(&xml) {
                    println!("Generated XML contains control characters: {:?}", xml);
                    return false;
                }
                // We generate the XML and then parse it back into a struct
                match parse(&xml) {
                    Ok(r) => if r == e {
                        true
                    } else {
                        println!("NOT EQUAL: {:?} != {:?}", r, e);
                        false
                    },
                    Err(e) => {
                        println!("ERROR DURING PARSE: {:?}", e);
                        false
                    }
                }
            },
            Err(e) => {
                println!("ERROR DURING GENERATE: {:?}", e);
                false
            }
        }
    }

    #[test]
    fn bytes() {
        let e: Envelope = Envelope::new(
            Some(CwmpVersion::new(1,0)), 
            vec![HeaderElement::ID(ID::new(true,String::from("1234")))], 
            vec![BodyElement::Inform(
                    Inform::new(
                        DeviceId::new(String::from("MyManufacturer"), String::from("OUI"), String::from("MyProductClass"), String::from("S123456")),
                        vec![EventStruct::new(String::from("2 PERIODIC"), String::from(""))],
                        1,
                        gen_utc_date(2014, 11, 28, 12, 0, 9),
                        0,
                        vec![
                            ParameterValue::new(String::from("InternetGatewayDevice.DeviceSummary"),String::from("xsd:string"),String::from("InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)")),
                            ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SpecVersion"),String::from("xsd:string"),String::from("1.0")),
                            ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.HardwareVersion"),String::from("xsd:string"),String::from("HW1.0")),
                            ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SoftwareVersion"),String::from("xsd:string"),String::from("V1.00(beta)")),
                            ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.ProvisioningCode"),String::from("xsd:string"),String::from("")),
                            ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ConnectionRequestURL"),String::from("xsd:string"),String::from("http://2.2.2.2:7676/CWMP/ConnectionRequest")),
                            ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ParameterKey"),String::from("xsd:string"),String::from("")),
                            ParameterValue::new(String::from("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress"),String::from("xsd:string"),String::from("2.2.2.2")),
                        ],
        
                    )
                )
            ]);
        match generate(&e) {
            Ok(xml) => match parse_bytes(xml.as_bytes()) {
                Ok(r) => assert_eq!(r , e),
                Err(e) => {
                    panic!("ERROR DURING PARSE: {:?}", e);
                }
            },
            Err(e) => {
                panic!("ERROR DURING GENERATE: {:?}", e);
            }
        }
    }
}
