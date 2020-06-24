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

// parse a CWMP XML envelope and convert it to a rust struct
pub fn parse(xml: String) -> Result<Envelope, Box<dyn Error>> {
    let config = ParserConfig::new().trim_whitespace(false).whitespace_to_characters(true);
    let parser = config.create_reader(xml.as_bytes());
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
        None =>  Ok(state.envelope),
        Some(b) => Err(b)
    }
   
}

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
    use super::*;
    use chrono::prelude::*;
    use chrono::{DateTime, Utc};
    use protocol::*;
    use std::str;
    extern crate quickcheck;

    #[test]
    fn parse_1() -> Result<(), String> {
        match parse(String::from("<xml></xml>")) {
            Ok(_e) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn add_object_response_1() {
        test(
            include_bytes!("xmlsamples/add_object_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::AddObjectResponse(
                protocol::AddObjectResponse::new(1, String::from("0")),
            )],
        )
    }
    #[test]
    fn add_object_1() {
        test(
            include_bytes!("xmlsamples/add_object_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::AddObject(protocol::AddObject::new(
                String::from("Device.Test."),
                String::from("ParamKey"),
            ))],
        )
    }
    #[test]
    // parsing invalid object names we must do, cause the handling of that particular
    // kind of error is up to the user of the module.
    fn add_object_2() {
        test(
            include_bytes!("xmlsamples/add_object_2.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::AddObject(protocol::AddObject::new(
                String::from("Device.Test"),
                String::from("ParamKey"),
            ))],
        )
    }

    #[test]
    fn autonomous_dustate_change_complete_response_1() {
        test(
            include_bytes!("xmlsamples/autonomous_dustate_change_complete_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::AutonomousDUStateChangeCompleteResponse(
                protocol::AutonomousDUStateChangeCompleteResponse {},
            )],
        )
    }

    #[test]
    fn autonomous_dustate_change_complete_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);

        let start_time: DateTime<Utc> = match "2015-01-19T23:45:12+00:00".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> =
            match "2015-01-19T23:55:12+00:00".parse::<DateTime<Utc>>() {
                Ok(dt) => dt,
                _ => bogus_utc_dt,
            };

        test(
            include_bytes!("xmlsamples/autonomous_dustate_change_complete_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::AutonomousDUStateChangeComplete(
                protocol::AutonomousDUStateChangeComplete::new(vec![protocol::AutonOpResult::new(
                    String::from("some-uuid"),
                    String::from("uref"),
                    String::from("v2.1"),
                    String::from("curState"),
                    String::from("1"),
                    String::from("a,b,c"),
                    start_time,
                    complete_time,
                    0,
                    String::from(""),
                    String::from("Install"),
                )]),
            )],
        )
    }

    #[test]
    fn autonomous_transfer_complete_response_1() {
        test(
            include_bytes!("xmlsamples/autonomous_transfer_complete_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("1")))],
            vec![BodyElement::AutonomousTransferCompleteResponse(
                protocol::AutonomousTransferCompleteResponse {},
            )],
        )
    }

    #[test]
    fn autonomous_transfer_complete_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);

        let start_time: DateTime<Utc> = match "2016-04-07T08:43:49Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2016-04-07T08:45:06Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        test(
            include_bytes!("xmlsamples/autonomous_transfer_complete_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("1")))],
            vec![BodyElement::AutonomousTransferComplete(
                protocol::AutonomousTransferComplete::new(
                    String::from("http://example.com/announce"),
                    String::from("http://example.com/transfer"),
                    1,
                    String::from("1 Firmware Upgrade Image"),
                    10000,
                    String::from("/bin/image"),
                    protocol::FaultStruct::new(0, String::from("")),
                    start_time,
                    complete_time,
                ),
            )],
        )
    }

    #[test]
    fn cancel_transfer_response_1() {
        test(
            include_bytes!("xmlsamples/cancel_transfer_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::CancelTransferResponse(
                protocol::CancelTransferResponse {},
            )],
        )
    }

    #[test]
    fn cancel_transfer_1() {
        test(
            include_bytes!("xmlsamples/cancel_transfer_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey"),
            ))],
        )
    }

    #[test]
    fn change_du_state_response_1() {
        test(
            include_bytes!("xmlsamples/change_du_state_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::ChangeDUStateResponse(
                protocol::ChangeDUStateResponse {},
            )],
        )
    }

    #[test]
    fn change_du_state_1() {
        test(
            include_bytes!("xmlsamples/change_du_state_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                String::from("cmdkey"),
                vec![protocol::InstallOp::new(
                    String::from("http://example.com/url"),
                    String::from("some-uuid"),
                    String::from("user"),
                    String::from("pass"),
                    String::from("env"),
                )],
                vec![],
                vec![],
            ))],
        )
    }

    #[test]
    fn change_du_state_2() {
        test(
            include_bytes!("xmlsamples/change_du_state_2.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                String::from("cmdkey"),
                vec![protocol::InstallOp::new(
                    String::from("http://example.com/url"),
                    String::from("some-uuid"),
                    String::from("user"),
                    String::from("pass"),
                    String::from("env"),
                )],
                vec![protocol::UninstallOp::new(
                    String::from("http://example.com/url2"),
                    String::from("some-uuid2"),
                    String::from("env2"),
                )],
                vec![],
            ))],
        )
    }

    #[test]
    fn change_du_state_3() {
        test(
            include_bytes!("xmlsamples/change_du_state_3.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                String::from("cmdkey"),
                vec![protocol::InstallOp::new(
                    String::from("http://example.com/url"),
                    String::from("some-uuid"),
                    String::from("user"),
                    String::from("pass"),
                    String::from("env"),
                )],
                vec![protocol::UninstallOp::new(
                    String::from("http://example.com/url2"),
                    String::from("some-uuid2"),
                    String::from("env2"),
                )],
                vec![protocol::UpdateOp::new(
                    String::from("http://example.com/url"),
                    String::from("some-uuid"),
                    String::from("user"),
                    String::from("pass"),
                    String::from("v2.0"),
                )],
            ))],
        )
    }

    #[test]
    fn delete_object_response_1() {
        test(
            include_bytes!("xmlsamples/delete_object_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::DeleteObjectResponse(
                protocol::DeleteObjectResponse::new(String::from("0")),
            )],
        )
    }

    #[test]
    fn delete_object_1() {
        test(
            include_bytes!("xmlsamples/delete_object_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::DeleteObject(protocol::DeleteObject::new(
                String::from("Device.Test."),
                String::from("ParamKey"),
            ))],
        )
    }

    #[test]
    fn download_response_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2015-01-19T23:08:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2015-01-19T23:09:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        test(
            include_bytes!("xmlsamples/download_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::DownloadResponse(
                protocol::DownloadResponse::new(String::from("1"), start_time, complete_time),
            )],
        )
    }

    #[test]
    fn download_1() {
        test(
            include_bytes!("xmlsamples/download_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::Download(protocol::Download::new(
                String::from("cmdkey"),
                String::from("1 Firmware Upgrade Image"),
                String::from("http://example.com/url"),
                String::from("user"),
                String::from("pass"),
                123456,
                String::from("image"),
                5,
                String::from("http://example.com/success"),
                String::from("http://example.com/failure"),
            ))],
        )
    }

    #[test]
    fn download_2() {
        test(
            include_bytes!("xmlsamples/download_2.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::Download(protocol::Download::new(
                String::from("cmdkey"),
                String::from("1 Firmware Upgrade Image"),
                String::from("http://example.com/url"),
                String::from(""),
                String::from(""),
                123456,
                String::from(""),
                0,
                String::from(""),
                String::from(""),
            ))],
        )
    }

    #[test]
    fn du_state_change_complete_response_1() {
        test(
            include_bytes!("xmlsamples/du_state_change_complete_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::DUStateChangeCompleteResponse(
                protocol::DUStateChangeCompleteResponse {},
            )],
        )
    }

    #[test]
    fn du_state_change_complete_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2015-01-19T23:45:12+00:00".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> =
            match "2015-01-19T23:55:12+00:00".parse::<DateTime<Utc>>() {
                Ok(dt) => dt,
                _ => bogus_utc_dt,
            };
        test(
            include_bytes!("xmlsamples/du_state_change_complete_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::DUStateChangeComplete(
                protocol::DUStateChangeComplete::new(
                    String::from("cmdkey"),
                    vec![protocol::OpResult::new(
                        String::from("some-uuid"),
                        String::from("uref"),
                        String::from("v2.1"),
                        String::from("curState"),
                        1,
                        String::from("a,b,c"),
                        start_time,
                        complete_time,
                        protocol::FaultStruct::new(0, String::from("")),
                    )],
                ),
            )],
        );
    }

    #[test]
    fn factory_reset_response_1() {
        test(
            include_bytes!("xmlsamples/factory_reset_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::FactoryResetResponse(
                protocol::FactoryResetResponse {},
            )],
        )
    }

    #[test]
    fn factory_reset_1() {
        test(
            include_bytes!("xmlsamples/factory_reset_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::FactoryReset(protocol::FactoryReset {})],
        )
    }

    #[test]
    fn fault_1() {
        test(
            include_bytes!("xmlsamples/fault_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::Fault(protocol::Fault::new(
                String::from("SOAP-ENV:Client"),
                String::from("CWMP fault"),
                9005,
                String::from("Invalid parameter name"),
            ))],
        )
    }

    #[test]
    fn get_all_queued_transfer_response_1() {
        test(
            include_bytes!("xmlsamples/get_all_queued_transfer_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_953323a9b674bb42b7cad250b2cf0607"),
            ))],
            vec![BodyElement::GetAllQueuedTransfersResponse(
                protocol::GetAllQueuedTransfersResponse::new(vec![
                    protocol::AllQueuedTransfers::new(
                        String::from("cmdkey"),
                        String::from("2"),
                        1,
                        String::from("1 Firmware Upgrade Image"),
                        123456,
                        String::from("image"),
                    ),
                    protocol::AllQueuedTransfers::new(
                        String::from("cmdkey2"),
                        String::from("3"),
                        0,
                        String::from("3 Vendor Configuration File"),
                        1234,
                        String::from(""),
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_all_queued_transfers_1() {
        test(
            include_bytes!("xmlsamples/get_all_queued_transfers_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::GetAllQueuedTransfers(
                protocol::GetAllQueuedTransfers {},
            )],
        )
    }

    #[test]
    fn get_options_response_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2015-01-10T23:45:12+00:00".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let expiration_time: DateTime<Utc> =
            match "2015-01-10T23:45:12+00:00".parse::<DateTime<Utc>>() {
                Ok(dt) => dt,
                _ => bogus_utc_dt,
            };
        test(
            include_bytes!("xmlsamples/get_options_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_69412286f02e475b44783c61972f0a91"),
            ))],
            vec![BodyElement::GetOptionsResponse(
                protocol::GetOptionsResponse::new(vec![
                    protocol::OptionStruct::new(
                        String::from("First Option"),
                        String::from("12345678"),
                        1,
                        String::from("1"),
                        start_time,
                        expiration_time,
                        1,
                    ),
                    protocol::OptionStruct::new(
                        String::from("Second Option"),
                        String::from("12345678"),
                        1,
                        String::from("1"),
                        start_time,
                        expiration_time,
                        1,
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_options_1() {
        test(
            include_bytes!("xmlsamples/get_options_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_69412286f02e475b44783c61972f0a91"),
            ))],
            vec![BodyElement::GetOptions(protocol::GetOptions::new(
                String::from("Some Option"),
            ))],
        )
    }

    #[test]
    fn get_parameter_attributes_response_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_attributes_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_953323a9b674bb42b7cad250b2cf0607"),
            ))],
            vec![BodyElement::GetParameterAttributesResponse(
                protocol::GetParameterAttributesResponse::new(vec![
                    ParameterAttribute::new(
                        String::from("InternetGatewayDevice.DeviceInfo.HardwareVersion"),
                        String::from("0"),
                        vec![String::from("Subscriber")],
                    ),
                    ParameterAttribute::new(
                        String::from("InternetGatewayDevice.DeviceInfo.SoftwareVersion"),
                        String::from("2"),
                        vec![String::from("Subscriber")],
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_parameter_attributes_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_attributes_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_953323a9b674bb42b7cad250b2cf0607"),
            ))],
            vec![BodyElement::GetParameterAttributes(
                protocol::GetParameterAttributes {
                    parameternames: vec![
                        "InternetGatewayDevice.DeviceInfo.HardwareVersion".to_string(),
                        "InternetGatewayDevice.DeviceInfo.SoftwareVersion".to_string(),
                    ],
                },
            )],
        )
    }

    #[test]
    fn get_parameter_names_response_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_names_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_28edd28d788a784422413db3914c34b0"),
            ))],
            vec![BodyElement::GetParameterNamesResponse(
                protocol::GetParameterNamesResponse::new(vec![
                    protocol::ParameterInfoStruct::new(String::from("InternetGatewayDevice.DeviceInfo."), 0),
                    protocol::ParameterInfoStruct::new(
                        String::from("InternetGatewayDevice.DeviceInfo.Manufacturer"),
                        1,
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_parameter_names_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_names_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_28edd28d788a784422413db3914c34b0"),
            ))],
            vec![BodyElement::GetParameterNames(
                protocol::GetParameterNames::new(String::from("InternetGatewayDevice.DeviceInfo."), 0),
            )],
        )
    }

    #[test]
    fn get_parameter_values_response_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_values_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::GetParameterValuesResponse(
                protocol::GetParameterValuesResponse::new(vec![
                    ParameterValue::new(
                        String::from("Device.IP.Interface.3.IPv4AddressNumberOfEntries"),
                        String::from("xsd:unsignedInt"),
                        String::from("1"),
                    ),
                    ParameterValue::new(
                        String::from("Device.IP.Interface.3.IPv6AddressNumberOfEntries"),
                        String::from("xsd:unsignedInt"),
                        String::from("2"),
                    ),
                ]),
            )],
        )
    }
    #[test]
    fn get_parameter_values_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_values_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::GetParameterValues(
                protocol::GetParameterValues::new(vec![
                    "Device.IP.Interface.3.IPv4AddressNumberOfEntries".to_string(),
                    "Device.IP.Interface.3.IPv6AddressNumberOfEntries".to_string(),
                ]),
            )],
        )
    }

    #[test]
    fn get_queued_transfers_response_1() {
        test(
            include_bytes!("xmlsamples/get_queued_transfers_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_28edd28d788a784422413db3914c34b0"),
            ))],
            vec![BodyElement::GetQueuedTransfersResponse(
                protocol::GetQueuedTransfersResponse::new(vec![
                    protocol::QueuedTransferStruct::new(Some(String::from("cmdkey")), Some(String::from("2"))),
                    protocol::QueuedTransferStruct::new(Some(String::from("cmdkey2")), Some(String::from("3"))),
                ]),
            )],
        )
    }

    #[test]
    fn get_queued_transfers_1() {
        test(
            include_bytes!("xmlsamples/get_queued_transfers_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_28edd28d788a784422413db3914c34b0"),
            ))],
            vec![BodyElement::GetQueuedTransfers(
                protocol::GetQueuedTransfers {},
            )],
        )
    }

    #[test]
    fn get_rpc_methods_response_1() {
        test(
            include_bytes!("xmlsamples/get_rpc_methods_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::GetRPCMethodsResponse(
                protocol::GetRPCMethodsResponse::new(vec![
                    String::from("GetRPCMethods"),
                    String::from("SetParameterValues"),
                    String::from("GetParameterValues"),
                    String::from("GetParameterNames"),
                    String::from("GetParameterAttributes"),
                    String::from("SetParameterAttributes"),
                    String::from("AddObject"),
                    String::from("DeleteObject"),
                    String::from("Reboot"),
                    String::from("Download"),
                    String::from("Upload"),
                    String::from("GetQueuedTransfers"),
                    String::from("ScheduleInform"),
                    String::from("FactoryReset"),
                ]),
            )],
        )
    }

    #[test]
    fn get_rpc_methods_1() {
        test(
            include_bytes!("xmlsamples/get_rpc_methods_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::GetRPCMethods(protocol::GetRPCMethods {})],
        )
    }

    #[test]
    fn inform_response_1() {
        test(
            include_bytes!("xmlsamples/inform_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![
                HeaderElement::ID(ID::new(true, String::from("100"))),
                HeaderElement::NoMoreRequests(protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![BodyElement::InformResponse(protocol::InformResponse::new(1))],
        )
    }

    #[test]
    fn inform_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);

        let current_time: DateTime<Utc> = match "2015-01-19T23:08:24+00:00".parse::<DateTime<Utc>>()
        {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        test(
            include_bytes!("xmlsamples/inform_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![               
                HeaderElement::ID(ID::new(true, String::from("100"))),
                HeaderElement::NoMoreRequests(protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![BodyElement::Inform(protocol::Inform::new(
                protocol::DeviceId::new(String::from("The Company"), String::from("AA1234"), String::from("IAD_001"), String::from("S99998888")),
                vec![protocol::EventStruct::new(String::from("2 PERIODIC"), String::from(""))],
                1,
                current_time,
                0,
                vec![
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceSummary"), String::from("xsd:string"), String::from("InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SpecVersion"), String::from("xsd:string"), String::from("1.0")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.HardwareVersion"), String::from("xsd:string"), String::from("HW1.0")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SoftwareVersion"), String::from("xsd:string"), String::from("V1.00(beta)")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.ProvisioningCode"), String::from("xsd:string"), String::from("")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ConnectionRequestURL"), String::from("xsd:string"), String::from("http://2.2.2.2:7676/CWMP/ConnectionRequest")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ParameterKey"), String::from("xsd:string"), String::from("")),
                protocol::ParameterValue::new(String::from("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress"), String::from("xsd:string"), String::from("2.2.2.2")),
                ],
            ))],
        )
    }

    #[test]
    fn kicked_response_1() {
        test(
            include_bytes!("xmlsamples/kicked_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::KickedResponse(protocol::KickedResponse::new(
                String::from("http://example.com/next"),
            ))],
        )
    }

    #[test]
    fn kicked_1() {
        test(
            include_bytes!("xmlsamples/kicked_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::Kicked(protocol::Kicked::new(
                String::from("cmd"),
                String::from("some_host"),
                String::from("success"),
                String::from("http://example.com/next"),
            ))],
        )
    }

    #[test]
    fn reboot_response_1() {
        test(
            include_bytes!("xmlsamples/reboot_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_953323a9b674bb42b7cad250b2cf0607")))],
            vec![BodyElement::RebootResponse(protocol::RebootResponse {})],
        )
    }

    #[test]
    fn reboot_1() {
        test(
            include_bytes!("xmlsamples/reboot_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_953323a9b674bb42b7cad250b2cf0607")))],
            vec![BodyElement::Reboot(protocol::Reboot::new(String::from("cmdkey")))],
        )
    }

    #[test]
    fn request_download_response_1() {
        test(
            include_bytes!("xmlsamples/request_download_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::RequestDownloadResponse(
                protocol::RequestDownloadResponse {},
            )],
        )
    }

    #[test]
    fn request_download_1() {
        test(
            include_bytes!("xmlsamples/request_download_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::RequestDownload(
                protocol::RequestDownload::new(
                    String::from("2 Web Content"),
                    vec![protocol::ArgStruct::new(String::from("Version"), String::from("v2.0"))],
                ),
            )],
        )
    }

    #[test]
    fn schedule_download_response_1() {
        test(
            include_bytes!("xmlsamples/schedule_download_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::ScheduleDownloadResponse(
                protocol::ScheduleDownloadResponse {},
            )],
        )
    }

    #[test]
    fn schedule_download_1() {
        test(
            include_bytes!("xmlsamples/schedule_download_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::ScheduleDownload(
                protocol::ScheduleDownload::new(
                    String::from("cmdkey"),
                    String::from("1 Firmware Upgrade Image"),
                    String::from("http://example.com/url"),
                    String::from("user"),
                    String::from("pass"),
                    123456,
                    String::from("image"),
                    vec![protocol::TimeWindow::new(
                        5,
                        45,
                        String::from("1 At Any Time"),
                        String::from("A message"),
                        -1,
                    )],
                ),
            )],
        )
    }

    #[test]
    fn schedule_inform_response_1() {
        test(
            include_bytes!("xmlsamples/schedule_inform_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_28edd28d788a784422413db3914c34b0")))],
            vec![BodyElement::ScheduleInformResponse(
                protocol::ScheduleInformResponse {},
            )],
        )
    }

    #[test]
    fn schedule_inform_1() {
        test(
            include_bytes!("xmlsamples/schedule_inform_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_28edd28d788a784422413db3914c34b0")))],
            vec![BodyElement::ScheduleInform(protocol::ScheduleInform::new(
                5, String::from("cmdkey"),
            ))],
        )
    }

    #[test]
    fn set_parameter_attributes_response_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_attributes_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::SetParameterAttributesResponse(
                protocol::SetParameterAttributesResponse {},
            )],
        )
    }

    #[test]
    fn set_parameter_attributes_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_attributes_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_7bfc27c1f4f0a2c1d775f8aa1840439e")))],
            vec![BodyElement::SetParameterAttributes(
                protocol::SetParameterAttributes::new(vec![
                    protocol::SetParameterAttributesStruct::new(
                        String::from("Device.Test"),
                        0,
                        2,
                        1,
                        vec![String::from("Subscriber")],
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn set_parameter_values_response_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::SetParameterValuesResponse(
                protocol::SetParameterValuesResponse::new(0),
            )],
        )
    }

    #[test]
    fn set_parameter_values_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::SetParameterValues(
                protocol::SetParameterValues::new(
                    None,
                    vec![
                        protocol::ParameterValue::new(String::from("Device.Test"), String::from("xsi:string"), String::from("Foo")),
                        protocol::ParameterValue::new(String::from("Device.Test.Whatever"), String::from("xsi:int"), String::from("1")),
                    ],
                ),
            )],
        )
    }

    #[test]
    fn set_parameter_values_2() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_2.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("50")))],
            vec![BodyElement::SetParameterValues(
                protocol::SetParameterValues::new(
                    Some("foo".to_string()),
                    vec![
                        protocol::ParameterValue::new(String::from("Device.Test"), String::from("xsi:string"), String::from("Foo")),
                        protocol::ParameterValue::new(String::from("Device.Test.Whatever"), String::from("xsi:int"), String::from("1")),
                    ],
                ),
            )],
        )
    }

    #[test]
    fn set_vouchers_response_1() {
        test(
            include_bytes!("xmlsamples/set_vouchers_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![
                HeaderElement::ID(ID::new(true, String::from("100"))),
                HeaderElement::NoMoreRequests(protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![BodyElement::SetVouchersResponse(
                protocol::SetVouchersResponse {},
            )],
        )
    }

    #[test]
    fn set_vouchers_1() {
        test(
            include_bytes!("xmlsamples/set_vouchers_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_69412286f02e475b44783c61972f0a91")))],
            vec![BodyElement::SetVouchers(protocol::SetVouchers::new(vec![
                String::from("Rm9vTW9vQmFy"),
            ]))],
        )
    }

    #[test]
    fn transfer_complete_response_1() {
        test(
            include_bytes!("xmlsamples/transfer_complete_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("1")))],
            vec![BodyElement::TransferCompleteResponse(
                protocol::TransferCompleteResponse {},
            )],
        )
    }

    #[test]
    fn transfer_complete_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2016-04-07T08:43:49Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2016-04-07T08:45:06Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        test(
            include_bytes!("xmlsamples/transfer_complete_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("1")))],
            vec![BodyElement::TransferComplete(
                protocol::TransferComplete::new(
                    String::from("AutoconfFirmwareUpgrade"),
                    protocol::FaultStruct::new(0, String::from("")),
                    Some(start_time),
                    Some(complete_time),
                ),
            )],
        )
    }

    #[test]
    fn upload_response_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2015-01-19T23:08:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2015-01-19T23:09:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        test(
            include_bytes!("xmlsamples/upload_response_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::UploadResponse(protocol::UploadResponse::new(
                1,
                Some(start_time),
                Some(complete_time),
            ))],
        )
    }

    #[test]
    fn upload_1() {
        test(
            include_bytes!("xmlsamples/upload_1.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::Upload(protocol::Upload::new(
                String::from("cmdkey"),
                String::from("1 Firmware Upgrade Image"),
                String::from("http://example.com/url"),
                String::from("user"),
                String::from("pass"),
                5,
            ))],
        )
    }

    #[test]
    fn upload_2() {
        test(
            include_bytes!("xmlsamples/upload_2.xml"),
            protocol::CwmpVersion::new(1,0),
            vec![HeaderElement::ID(ID::new(true, String::from("API_aa0642e34b23820801e7642ad7cb536c")))],
            vec![BodyElement::Upload(protocol::Upload::new(
                String::from("cmdkey"),
                String::from("1 Firmware Upgrade Image"),
                String::from("http://example.com/url"),
                String::from(""),
                String::from(""),
                0,
            ))],
        )
    }


    // Generate and Parse tests...
    #[test]
    fn add_object_gap_1() {
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
          vec![BodyElement::AddObject(protocol::AddObject::new(
                String::from("Device.Test."),
                String::from("ParamKey"),
            ))]);

        test_gap(&e);
    }

    #[test]
    fn add_object_response_gap_1() {
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
          vec![BodyElement::AddObjectResponse(protocol::AddObjectResponse::new(
               1,String::from("0"),
            ))]);

        test_gap(&e);
    }

    #[test]
    fn autonomous_du_state_change_complete_gap_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2020-05-07T23:08:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2020-05-08T23:09:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
      
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
          vec![BodyElement::AutonomousDUStateChangeComplete(  protocol::AutonomousDUStateChangeComplete::new(vec![protocol::AutonOpResult::new(
            String::from("some-uuid"),
            String::from("uref"),
            String::from("v2.1"),
            String::from("curState"),
            String::from("1"),
            String::from("a,b,c"),
            start_time,
            complete_time,
            0,
            String::from(""),
            String::from("Install"),
        )]),)]);

        test_gap(&e);
    }

    #[test]
    fn autonomous_du_state_change_complete_response_gap_1() {
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
          vec![BodyElement::AutonomousDUStateChangeCompleteResponse(protocol::AutonomousDUStateChangeCompleteResponse {}
             )]);

        test_gap(&e);
    }

    #[test]
    fn autonomous_transfer_complete_gap_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2020-05-07T23:08:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2020-05-08T23:09:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
      
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
            vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
            vec![BodyElement::AutonomousTransferComplete(  protocol::AutonomousTransferComplete::new(
                String::from("http://example.com/announce"),
                String::from("http://example.com/transfer"),
                1,
                String::from("1 Firmware Upgrade Image"),
                10000,
                String::from("/bin/image"),
                protocol::FaultStruct::new(0, String::from("")),
                start_time,
                complete_time,
            ))]
        );

        test_gap(&e);
    }

    #[test]
    fn autonomous_transfer_complete_response_gap_1() {
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
            vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
            vec![BodyElement::AutonomousTransferCompleteResponse(protocol::AutonomousTransferCompleteResponse {})]
        );

        test_gap(&e);
    }

    #[test]
    fn cancel_transfer_gap_1() {
    
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
          vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey"),
            ))],
        );

        test_gap(&e);
    }

    
    #[test]
    fn upload_gap_1() {
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
        vec![BodyElement::Upload(protocol::Upload::new(
            String::from("cmdkey"),
            String::from("1 Firmware Upgrade Image"),
            String::from("http://example.com/url"),
            String::from(""),
            String::from(""),
            0,
        ))]);

        test_gap(&e);
    }

    #[test]
    fn upload_response_gap_1() {
        let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_utc_dt = bogus_dt.with_timezone(&Utc);
        let start_time: DateTime<Utc> = match "2020-05-07T23:08:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
        let complete_time: DateTime<Utc> = match "2020-05-08T23:09:24Z".parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            _ => bogus_utc_dt,
        };
      
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
        vec![BodyElement::UploadResponse(protocol::UploadResponse::new(
            1,
            Some(start_time),
            Some(complete_time),
        ))]);

        test_gap(&e);
    }

    #[test]
    fn upload_response_gap_2() {
        let start_time: DateTime<Utc> = Utc::now();
        let complete_time: DateTime<Utc> = Utc::now();
      
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
        vec![HeaderElement::ID(ID::new(true, String::from("12345678")))],
        vec![BodyElement::UploadResponse(protocol::UploadResponse::new(
            1,
            Some(start_time),
            Some(complete_time),
        ))]);

        test_gap(&e);
    }

    fn test(input: &[u8], cwmp: CwmpVersion, header: Vec<HeaderElement>, body: Vec<BodyElement>) {
        let should_be = Envelope::new(Some(cwmp), header, body);
        let envelope: protocol::Envelope =
            parse(str::from_utf8(input).unwrap().to_string()).unwrap();
        assert_eq!(envelope, should_be);
    }

    fn test_gap(envelope: &Envelope) {
        // Generate xml from the envelope
        let xml: String = generate(envelope).unwrap();

        println!("XML: [{}]", xml);

        // Parse the generated xml
        let parsed: protocol::Envelope =     
                parse(xml).unwrap();

        // compare parser outout to passed envelope
        assert_eq!(*envelope, parsed)
    }

    #[test]
    fn cancel_transfer_multiple_bodies_gap_1() {
    
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
          vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            ))],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey"),
            )),BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey2"),
            ))],
        );

        test_gap(&e);
    }


    #[test]
    fn cancel_transfer_multiple_headers_same_type_gap_1() {
    
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
          vec![HeaderElement::ID(ID::new(
                true,
                String::from("API_aa0642e34b23820801e7642ad7cb536c"),
            )),
            HeaderElement::ID(ID::new(
                true,
                String::from("API_1234"),
            ))],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey"),
            ))],
        );

        test_gap(&e);
    }

    #[test]
    fn cancel_transfer_multiple_headers_same_type_gap_2() {
    
        let e = protocol::Envelope::new(Some(protocol::CwmpVersion::new(1,0)),
          vec![HeaderElement::SessionTimeout(SessionTimeout::new(
                true,
                45
                )),
            HeaderElement::SessionTimeout(SessionTimeout::new(
                true,
                60,
            ))],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                String::from("cmdkey"),
            ))],
        );

        test_gap(&e);
    }

    #[test]
    fn get_options_response_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetOptionsResponse(GetOptionsResponse::new(vec![]) )]);
        test_gap(&e);
    }

    #[test]
    fn set_parameter_values_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::SetParameterValues(SetParameterValues::new(None, vec![]) )]);
        test_gap(&e);
    }

    #[test]
    fn get_parameter_values_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetParameterValues(GetParameterValues::new(vec!["".to_string()]) )]);
        test_gap(&e);
    }

    #[test]
    fn get_queued_transfers_response_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetQueuedTransfersResponse(GetQueuedTransfersResponse::new(vec![QueuedTransferStruct::new(None, Some("".to_string()))]) )]);
        test_gap(&e);
    }

    #[test]
    fn autonomous_du_state_change_complete_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::AutonomousDUStateChangeComplete(AutonomousDUStateChangeComplete::new(vec![]) )]);
        test_gap(&e);
    }

    #[test]
    fn request_download_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::RequestDownload(RequestDownload::new("".to_string(), vec![ArgStruct::new("".to_string(), "\t".to_string())]) )]);
        test_gap(&e);
    }

    #[test]
    fn get_options_response_qcfail_gap_2() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetOptionsResponse(GetOptionsResponse::new(vec![OptionStruct::new("?>".to_string(), "".to_string(), 0, "".to_string(), Utc::now(), Utc::now(), 0)]) )]);
        test_gap(&e);
    }

    #[test]
    fn set_parameter_values_qcfail_gap_2() {
        let e = Envelope::new(None, vec![], vec![BodyElement::SetParameterValues(SetParameterValues::new(Some("".to_string()), vec![]) )]);
        test_gap(&e);
    }

    #[test]
    fn set_parameter_values_qcfail_gap_3() {
        let e = Envelope::new(None, vec![], vec![BodyElement::SetParameterValues(SetParameterValues::new(None, vec![ParameterValue::new("".to_string(), "".to_string(), "".to_string())]) )]);
        test_gap(&e);
    }

    #[test]
    fn get_all_queued_transfers_response_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetAllQueuedTransfersResponse(GetAllQueuedTransfersResponse::new(vec![AllQueuedTransfers::new(String::from(""), String::from(""), 1, "".to_string(), 0, "".to_string())]) )]);
        test_gap(&e);
    }

    #[test]
    fn get_parameter_attributes_response_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::GetParameterAttributesResponse(GetParameterAttributesResponse::new(vec![ParameterAttribute::new(String::from(""), String::from(""), vec![String::from("")])]) )]);
        test_gap(&e);
    }

    #[test]
    fn du_state_change_complete_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::DUStateChangeComplete(DUStateChangeComplete::new("".to_string(), vec![]) )]);
        test_gap(&e);
    }

    #[test]
    fn set_parameter_attributes_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::SetParameterAttributes(SetParameterAttributes::new(vec![SetParameterAttributesStruct::new("".to_string(), 0, 0, 0, vec!["".to_string()])]) )]);
        test_gap(&e);
    }

    #[test]
    fn change_dustate_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::ChangeDUState(ChangeDUState::new("".to_string(), vec![], vec![], vec![UpdateOp::new("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())]))]);
        test_gap(&e);
    }

    #[test]
    fn use_cwmp_version_header_qcfail_gap_1() {
        let e = Envelope::new(None, vec![HeaderElement::UseCWMPVersion(UseCWMPVersion::new(false, "".to_string()))], vec![]);
        test_gap(&e);
    }

    #[test]
    fn blank_envelope_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![]);
        test_gap(&e);
    }

    #[test]
    fn add_object_response_qcfail_gap_1() {
        let e = Envelope::new(None, vec![], vec![BodyElement::AddObjectResponse(AddObjectResponse::new(0, "".to_string()))]);
        test_gap(&e);
    }

    #[quickcheck]
    fn gen_and_parse(e: Envelope) -> bool {
        match generate(&e) {
            Ok(xml) => match parse(xml) {
                Ok(r) => r == e,
                Err(e) => {
                    println!("ERROR DURING PARSE: {:?}", e);
                    false
                }
            },
            Err(e) => {
                println!("ERROR DURING GENERATE: {:?}", e);
                false 
            }

        }
    }
   

}
