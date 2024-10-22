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

/// Parses some XML supposedly being a CWMP Envelope and convert it to a rust struct
/// 
/// # Errors
/// 
/// Returns an error if the envelope cannot be parsed from the XML
pub fn parse(xml: &str) -> Result<Envelope, Box<dyn Error>> {
    parse_bytes(xml.as_bytes())
}

/// parse a CWMP XML envelope as a slice and convert it to a rust struct
/// 
/// # Errors
/// 
/// Returns a `core::Error` if the envelope cannot be parsed from the XML
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

///
/// Generate CWMP XML from a rust Evenlope struct
/// 
/// # Errors
/// 
/// Returns a `protocol::GenerateError` if the envelope cannot be converted to XML
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

    #[quickcheck]
    fn gen_and_parse(e: Envelope) -> bool {
        match generate(&e) {
            Ok(xml) => {
                match parse(&xml) {
                Ok(r) => if r == e {
                    true
                } else {
                    println!("gen_and_parse NOT EQUAL: {r:?} != {e:?}");
                    false
                },
                Err(e) => {
                    println!("gen_and_parse ERROR DURING PARSE: {e:?}");
                    false
                }}
            },
            Err(e) => {
                println!("gen_and_parse ERROR DURING GENERATE: {e:?}");
                false
            }
        }
    }

    #[test]
    fn bytes() {
        let e: Envelope = Envelope::new(
            Some(CwmpVersion::new(1,0)), 
            vec![HeaderElement::ID(ID::new(true,"1234"))], 
            vec![BodyElement::Inform(
                    Inform::new(
                        DeviceId::new("MyManufacturer", "OUI", "MyProductClass", "S123456"),
                        vec![EventStruct::new("2 PERIODIC", "")],
                        1,
                        gen_utc_date(2014, 11, 28, 12, 0, 9),
                        0,
                        vec![
                            ParameterValue::new("InternetGatewayDevice.DeviceSummary","xsd:string","InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)"),
                            ParameterValue::new("InternetGatewayDevice.DeviceInfo.SpecVersion","xsd:string","1.0"),
                            ParameterValue::new("InternetGatewayDevice.DeviceInfo.HardwareVersion","xsd:string","HW1.0"),
                            ParameterValue::new("InternetGatewayDevice.DeviceInfo.SoftwareVersion","xsd:string","V1.00(beta)"),
                            ParameterValue::new("InternetGatewayDevice.DeviceInfo.ProvisioningCode","xsd:string",""),
                            ParameterValue::new("InternetGatewayDevice.ManagementServer.ConnectionRequestURL","xsd:string","http://2.2.2.2:7676/CWMP/ConnectionRequest"),
                            ParameterValue::new("InternetGatewayDevice.ManagementServer.ParameterKey","xsd:string",""),
                            ParameterValue::new("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress","xsd:string","2.2.2.2"),
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
