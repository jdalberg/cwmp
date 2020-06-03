extern crate xml;

use std::error::Error;
use xml::reader::{ParserConfig, XmlEvent};

// import the protocol defs into global scope
use protocol::{Envelope, State};
mod protocol;

trait TrimInPlace {
    fn trim_in_place(self: &'_ mut Self);
}
impl TrimInPlace for String {
    fn trim_in_place(self: &'_ mut Self) {
        let (start, len): (*const u8, usize) = {
            let self_trimmed: &str = self.trim();
            (self_trimmed.as_ptr(), self_trimmed.len())
        };
        unsafe {
            core::ptr::copy(
                start,
                self.as_bytes_mut().as_mut_ptr(), // no str::as_mut_ptr() in std ...
                len,
            );
        }
        self.truncate(len); // no String::set_len() in std ...
    }
}
// using xml-rs and serde did not seem viable due to the chaotic nature of
// vendors
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct

// parse a CWMP XML envelope and convert it to a rust struct
pub fn parse(xml: String) -> Result<Envelope, Box<dyn Error>> {
    let config = ParserConfig::new().trim_whitespace(true);
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
                println!("Error: {}", e);
                break;
            }
            event => {
                println!("{:?}", event);
            }
        }
    }
    Ok(state.envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::{DateTime, Utc};
    use protocol::{BodyElement, Envelope, HeaderElement, ParameterAttribute, ParameterValue, ID};
    use std::str;
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::AddObjectResponse(
                protocol::AddObjectResponse::new(1, "0"),
            )],
        )
    }
    #[test]
    fn add_object_1() {
        test(
            include_bytes!("xmlsamples/add_object_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::AddObject(protocol::AddObject::new(
                "Device.Test.",
                "ParamKey",
            ))],
        )
    }
    #[test]
    // parsing invalid object names we must do, cause the handling of that particular
    // kind of error is up to the user of the module.
    fn add_object_2() {
        test(
            include_bytes!("xmlsamples/add_object_2.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::AddObject(protocol::AddObject::new(
                "Device.Test",
                "ParamKey",
            ))],
        )
    }

    #[test]
    fn autonomous_dustate_change_complete_response_1() {
        test(
            include_bytes!("xmlsamples/autonomous_dustate_change_complete_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::AutonomousDUStateChangeComplete(
                protocol::AutonomousDUStateChangeComplete::new(vec![protocol::AutoOpResult::new(
                    "some-uuid",
                    "uref",
                    "v2.1",
                    "curState",
                    "1",
                    "a,b,c",
                    start_time,
                    complete_time,
                    0,
                    "",
                    "Install",
                )]),
            )],
        )
    }

    #[test]
    fn autonomous_transfer_complete_response_1() {
        test(
            include_bytes!("xmlsamples/autonomous_transfer_complete_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
            vec![BodyElement::AutonomousTransferComplete(
                protocol::AutonomousTransferComplete::new(
                    "http://example.com/announce",
                    "http://example.com/transfer",
                    1,
                    "1 Firmware Upgrade Image",
                    10000,
                    "/bin/image",
                    protocol::FaultStruct::new(0, ""),
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::CancelTransferResponse(
                protocol::CancelTransferResponse {},
            )],
        )
    }

    #[test]
    fn cancel_transfer_1() {
        test(
            include_bytes!("xmlsamples/cancel_transfer_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                "cmdkey",
            ))],
        )
    }

    #[test]
    fn change_du_state_response_1() {
        test(
            include_bytes!("xmlsamples/change_du_state_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::ChangeDUStateResponse(
                protocol::ChangeDUStateResponse {},
            )],
        )
    }

    #[test]
    fn change_du_state_1() {
        test(
            include_bytes!("xmlsamples/change_du_state_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                "cmdkey",
                vec![protocol::InstallOp::new(
                    "http://example.com/url",
                    "some-uuid",
                    "user",
                    "pass",
                    "env",
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                "cmdkey",
                vec![protocol::InstallOp::new(
                    "http://example.com/url",
                    "some-uuid",
                    "user",
                    "pass",
                    "env",
                )],
                vec![protocol::UninstallOp::new(
                    "http://example.com/url2",
                    "some-uuid2",
                    "env2",
                )],
                vec![],
            ))],
        )
    }

    #[test]
    fn change_du_state_3() {
        test(
            include_bytes!("xmlsamples/change_du_state_3.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
                "cmdkey",
                vec![protocol::InstallOp::new(
                    "http://example.com/url",
                    "some-uuid",
                    "user",
                    "pass",
                    "env",
                )],
                vec![protocol::UninstallOp::new(
                    "http://example.com/url2",
                    "some-uuid2",
                    "env2",
                )],
                vec![protocol::UpdateOp::new(
                    "http://example.com/url",
                    "some-uuid",
                    "user",
                    "pass",
                    "v2.0",
                )],
            ))],
        )
    }

    #[test]
    fn delete_object_response_1() {
        test(
            include_bytes!("xmlsamples/delete_object_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::DeleteObjectResponse(
                protocol::DeleteObjectResponse::new("0"),
            )],
        )
    }

    #[test]
    fn delete_object_1() {
        test(
            include_bytes!("xmlsamples/delete_object_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::DeleteObject(protocol::DeleteObject::new(
                "Device.Test.",
                "ParamKey",
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::DownloadResponse(
                protocol::DownloadResponse::new("1", start_time, complete_time),
            )],
        )
    }

    #[test]
    fn download_1() {
        test(
            include_bytes!("xmlsamples/download_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::Download(protocol::Download::new(
                "cmdkey",
                "1 Firmware Upgrade Image",
                "http://example.com/url",
                "user",
                "pass",
                123456,
                "image",
                5,
                "http://example.com/success",
                "http://example.com/failure",
            ))],
        )
    }

    #[test]
    fn download_2() {
        test(
            include_bytes!("xmlsamples/download_2.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::Download(protocol::Download::new(
                "cmdkey",
                "1 Firmware Upgrade Image",
                "http://example.com/url",
                "",
                "",
                123456,
                "",
                0,
                "",
                "",
            ))],
        )
    }

    #[test]
    fn du_state_change_complete_response_1() {
        test(
            include_bytes!("xmlsamples/du_state_change_complete_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::DUStateChangeComplete(
                protocol::DUStateChangeComplete::new(
                    "cmdkey",
                    vec![protocol::OpResult::new(
                        "some-uuid",
                        "uref",
                        "v2.1",
                        "curState",
                        1,
                        "a,b,c",
                        start_time,
                        complete_time,
                        protocol::FaultStruct::new(0, ""),
                    )],
                ),
            )],
        );
    }

    #[test]
    fn factory_reset_response_1() {
        test(
            include_bytes!("xmlsamples/factory_reset_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::FactoryResetResponse(
                protocol::FactoryResetResponse {},
            )],
        )
    }

    #[test]
    fn factory_reset_1() {
        test(
            include_bytes!("xmlsamples/factory_reset_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::FactoryReset(protocol::FactoryReset {})],
        )
    }

    #[test]
    fn fault_1() {
        test(
            include_bytes!("xmlsamples/fault_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::Fault(protocol::Fault::new(
                "SOAP-ENV:Client",
                "CWMP fault",
                9005,
                "Invalid parameter name",
            ))],
        )
    }

    #[test]
    fn get_all_queued_transfer_response_1() {
        test(
            include_bytes!("xmlsamples/get_all_queued_transfer_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            vec![BodyElement::GetAllQueuedTransfersResponse(
                protocol::GetAllQueuedTransfersResponse::new(vec![
                    protocol::AllQueuedTransfers::new(
                        "cmdkey",
                        "2",
                        1,
                        "1 Firmware Upgrade Image",
                        123456,
                        "image",
                    ),
                    protocol::AllQueuedTransfers::new(
                        "cmdkey2",
                        "3",
                        0,
                        "3 Vendor Configuration File",
                        1234,
                        "",
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_all_queued_transfers_1() {
        test(
            include_bytes!("xmlsamples/get_all_queued_transfers_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_69412286f02e475b44783c61972f0a91".to_string(),
            })],
            vec![BodyElement::GetOptionsResponse(
                protocol::GetOptionsResponse::new(vec![
                    protocol::OptionStruct::new(
                        "First Option",
                        "12345678",
                        1,
                        "1",
                        start_time,
                        expiration_time,
                        1,
                    ),
                    protocol::OptionStruct::new(
                        "Second Option",
                        "12345678",
                        1,
                        "1",
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_69412286f02e475b44783c61972f0a91".to_string(),
            })],
            vec![BodyElement::GetOptions(protocol::GetOptions::new(
                "Some Option",
            ))],
        )
    }

    #[test]
    fn get_parameter_attributes_response_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_attributes_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            vec![BodyElement::GetParameterAttributesResponse(
                protocol::GetParameterAttributesResponse::new(vec![
                    ParameterAttribute::new(
                        "InternetGatewayDevice.DeviceInfo.HardwareVersion",
                        "0",
                        vec!["Subscriber"],
                    ),
                    ParameterAttribute::new(
                        "InternetGatewayDevice.DeviceInfo.SoftwareVersion",
                        "2",
                        vec!["Subscriber"],
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn get_parameter_attributes_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_attributes_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::GetParameterNamesResponse(
                protocol::GetParameterNamesResponse::new(vec![
                    protocol::ParameterInfoStruct::new("InternetGatewayDevice.DeviceInfo.", 0),
                    protocol::ParameterInfoStruct::new(
                        "InternetGatewayDevice.DeviceInfo.Manufacturer",
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::GetParameterNames(
                protocol::GetParameterNames::new("InternetGatewayDevice.DeviceInfo.", 0),
            )],
        )
    }

    #[test]
    fn get_parameter_values_response_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_values_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::GetParameterValuesResponse(
                protocol::GetParameterValuesResponse::new(vec![
                    ParameterValue::new(
                        "Device.IP.Interface.3.IPv4AddressNumberOfEntries",
                        "xsd:unsignedInt",
                        "1",
                    ),
                    ParameterValue::new(
                        "Device.IP.Interface.3.IPv6AddressNumberOfEntries",
                        "xsd:unsignedInt",
                        "2",
                    ),
                ]),
            )],
        )
    }
    #[test]
    fn get_parameter_values_1() {
        test(
            include_bytes!("xmlsamples/get_parameter_values_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::GetQueuedTransfersResponse(
                protocol::GetQueuedTransfersResponse::new(vec![
                    protocol::QueuedTransferStruct::new("cmdkey", "2"),
                    protocol::QueuedTransferStruct::new("cmdkey2", "3"),
                ]),
            )],
        )
    }

    #[test]
    fn get_queued_transfers_1() {
        test(
            include_bytes!("xmlsamples/get_queued_transfers_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::GetQueuedTransfers(
                protocol::GetQueuedTransfers {},
            )],
        )
    }

    #[test]
    fn get_rpc_methods_response_1() {
        test(
            include_bytes!("xmlsamples/get_rpc_methods_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::GetRPCMethodsResponse(
                protocol::GetRPCMethodsResponse::new(vec![
                    "GetRPCMethods",
                    "SetParameterValues",
                    "GetParameterValues",
                    "GetParameterNames",
                    "GetParameterAttributes",
                    "SetParameterAttributes",
                    "AddObject",
                    "DeleteObject",
                    "Reboot",
                    "Download",
                    "Upload",
                    "GetQueuedTransfers",
                    "ScheduleInform",
                    "FactoryReset",
                ]),
            )],
        )
    }

    #[test]
    fn get_rpc_methods_1() {
        test(
            include_bytes!("xmlsamples/get_rpc_methods_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::GetRPCMethods(protocol::GetRPCMethods {})],
        )
    }

    #[test]
    fn inform_response_1() {
        test(
            include_bytes!("xmlsamples/inform_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![
                HeaderElement::ID(ID {
                    must_understand: true,
                    id: "100".to_string(),
                }),
                HeaderElement::NoMoreRequests(protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![BodyElement::InformResponse(protocol::InformResponse {})],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "100".to_string(),
            }),
            HeaderElement::NoMoreRequests(protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![BodyElement::Inform(protocol::Inform::new(
                protocol::DeviceId::new("The Company", "AA1234", "IAD_001", "S99998888"),
                vec![protocol::EventStruct::new("2 PERIODIC", "")],
                1,
                current_time,
                0,
                vec![
                protocol::ParameterValue::new("InternetGatewayDevice.DeviceSummary","xsd:string","InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)"),
                protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.SpecVersion","xsd:string","1.0"),
                protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.HardwareVersion","xsd:string","HW1.0"),
                protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.SoftwareVersion","xsd:string","V1.00(beta)"),
                protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.ProvisioningCode","xsd:string",""),
                protocol::ParameterValue::new("InternetGatewayDevice.ManagementServer.ConnectionRequestURL","xsd:string","http://2.2.2.2:7676/CWMP/ConnectionRequest"),
                protocol::ParameterValue::new("InternetGatewayDevice.ManagementServer.ParameterKey","xsd:string",""),
                protocol::ParameterValue::new("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress","xsd:string","2.2.2.2"),
                ],
            ))],
        )
    }

    #[test]
    fn kicked_response_1() {
        test(
            include_bytes!("xmlsamples/kicked_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::KickedResponse(protocol::KickedResponse::new(
                "http://example.com/next",
            ))],
        )
    }

    #[test]
    fn kicked_1() {
        test(
            include_bytes!("xmlsamples/kicked_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::Kicked(protocol::Kicked::new(
                "cmd",
                "some_host",
                "success",
                "http://example.com/next",
            ))],
        )
    }

    #[test]
    fn reboot_response_1() {
        test(
            include_bytes!("xmlsamples/reboot_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            vec![BodyElement::RebootResponse(protocol::RebootResponse {})],
        )
    }

    #[test]
    fn reboot_1() {
        test(
            include_bytes!("xmlsamples/reboot_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            vec![BodyElement::Reboot(protocol::Reboot::new("cmdkey"))],
        )
    }

    #[test]
    fn request_download_response_1() {
        test(
            include_bytes!("xmlsamples/request_download_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::RequestDownloadResponse(
                protocol::RequestDownloadResponse {},
            )],
        )
    }

    #[test]
    fn request_download_1() {
        test(
            include_bytes!("xmlsamples/request_download_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::RequestDownload(
                protocol::RequestDownload::new(
                    "2 Web Content",
                    vec![protocol::ArgStruct::new("Version", "v2.0")],
                ),
            )],
        )
    }

    #[test]
    fn schedule_download_response_1() {
        test(
            include_bytes!("xmlsamples/schedule_download_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::ScheduleDownloadResponse(
                protocol::ScheduleDownloadResponse {},
            )],
        )
    }

    #[test]
    fn schedule_download_1() {
        test(
            include_bytes!("xmlsamples/schedule_download_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::ScheduleDownload(
                protocol::ScheduleDownload::new(
                    "cmdkey",
                    "1 Firmware Upgrade Image",
                    "http://example.com/url",
                    "user",
                    "pass",
                    123456,
                    "image",
                    vec![protocol::TimeWindow::new(
                        5,
                        45,
                        "1 At Any Time",
                        "A message",
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::ScheduleInformResponse(
                protocol::ScheduleInformResponse {},
            )],
        )
    }

    #[test]
    fn schedule_inform_1() {
        test(
            include_bytes!("xmlsamples/schedule_inform_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_28edd28d788a784422413db3914c34b0".to_string(),
            })],
            vec![BodyElement::ScheduleInform(protocol::ScheduleInform::new(
                5, "cmdkey",
            ))],
        )
    }

    #[test]
    fn set_parameter_attributes_response_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_attributes_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::SetParameterAttributesResponse(
                protocol::SetParameterAttributesResponse {},
            )],
        )
    }

    #[test]
    fn set_parameter_attributes_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_attributes_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_7bfc27c1f4f0a2c1d775f8aa1840439e".to_string(),
            })],
            vec![BodyElement::SetParameterAttributes(
                protocol::SetParameterAttributes::new(vec![
                    protocol::SetParameterAttributesStruct::new(
                        "Device.Test",
                        0,
                        2,
                        1,
                        vec!["Subscriber"],
                    ),
                ]),
            )],
        )
    }

    #[test]
    fn set_parameter_values_response_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::SetParameterValuesResponse(
                protocol::SetParameterValuesResponse::new(0),
            )],
        )
    }

    #[test]
    fn set_parameter_values_1() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::SetParameterValues(
                protocol::SetParameterValues::new(
                    None,
                    vec![
                        protocol::ParameterValue::new("Device.Test", "xsi:string", "Foo"),
                        protocol::ParameterValue::new("Device.Test.Whatever", "xsi:int", "1"),
                    ],
                ),
            )],
        )
    }

    #[test]
    fn set_parameter_values_2() {
        test(
            include_bytes!("xmlsamples/set_parameter_values_2.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            vec![BodyElement::SetParameterValues(
                protocol::SetParameterValues::new(
                    Some("foo".to_string()),
                    vec![
                        protocol::ParameterValue::new("Device.Test", "xsi:string", "Foo"),
                        protocol::ParameterValue::new("Device.Test.Whatever", "xsi:int", "1"),
                    ],
                ),
            )],
        )
    }

    #[test]
    fn set_vouchers_response_1() {
        test(
            include_bytes!("xmlsamples/set_vouchers_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![
                HeaderElement::ID(ID {
                    must_understand: true,
                    id: "100".to_string(),
                }),
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_69412286f02e475b44783c61972f0a91".to_string(),
            })],
            vec![BodyElement::SetVouchers(protocol::SetVouchers::new(vec![
                "Rm9vTW9vQmFy",
            ]))],
        )
    }

    #[test]
    fn transfer_complete_response_1() {
        test(
            include_bytes!("xmlsamples/transfer_complete_response_1.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
            vec![BodyElement::TransferComplete(
                protocol::TransferComplete::new(
                    "AutoconfFirmwareUpgrade",
                    protocol::FaultStruct::new(0, ""),
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
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
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::Upload(protocol::Upload::new(
                "cmdkey",
                "1 Firmware Upgrade Image",
                "http://example.com/url",
                "user",
                "pass",
                5,
            ))],
        )
    }

    #[test]
    fn upload_2() {
        test(
            include_bytes!("xmlsamples/upload_2.xml"),
            "urn:dslforum-org:cwmp-1-0",
            vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            vec![BodyElement::Upload(protocol::Upload::new(
                "cmdkey",
                "1 Firmware Upgrade Image",
                "http://example.com/url",
                "",
                "",
                0,
            ))],
        )
    }

    fn test(input: &[u8], cwmp: &str, header: Vec<HeaderElement>, body: Vec<BodyElement>) {
        let should_be = Envelope {
            cwmp: Some(cwmp.to_string()),
            header: header,
            body: body,
        };
        let envelope: protocol::Envelope =
            parse(str::from_utf8(input).unwrap().to_string()).unwrap();
        assert_eq!(envelope, should_be);
    }
}
