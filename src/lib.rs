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

    #[test]
    fn parse_1() -> Result<(), String> {
        match parse(String::from("<xml></xml>")) {
            Ok(_e) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn add_object_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:AddObjectResponse>
            <InstanceNumber>1</InstanceNumber>
            <Status>0</Status>
          </cwmp:AddObjectResponse>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::AddObjectResponse(
                protocol::AddObjectResponse::new(1, "0"),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }
    #[test]
    fn add_object_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:AddObject>
            <ObjectName>Device.Test.</ObjectName>
            <ParameterKey>ParamKey</ParameterKey>
          </cwmp:AddObject>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::AddObject(protocol::AddObject::new(
                "Device.Test.",
                "ParamKey",
            ))],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }
    #[test]
    // parsing invalid object names we must do, cause the handling of that particular
    // kind of error is up to the user of the module.
    fn add_object_2() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:AddObject>
            <ObjectName>Device.Test</ObjectName>
            <ParameterKey>ParamKey</ParameterKey>
          </cwmp:AddObject>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::AddObject(protocol::AddObject::new(
                "Device.Test",
                "ParamKey",
            ))],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }
    #[test]
    fn autonomous_dustate_change_complete_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:AutonomousDUStateChangeCompleteResponse/>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::AutonomousDUStateChangeCompleteResponse(
                protocol::AutonomousDUStateChangeCompleteResponse {},
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }
    #[test]
    fn autonomous_dustate_change_complete_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:AutonomousDUStateChangeComplete>
                  <Results SOAP-ENC:arrayType="cwmp:AutonOpResultStruct[1]">
                    <AutonOpResultStruct>
                      <UUID>some-uuid</UUID>
                      <DeploymentUnitRef>uref</DeploymentUnitRef>
                      <Version>v2.1</Version>
                      <CurrentState>curState</CurrentState>
                      <Resolved>1</Resolved>
                      <ExecutionUnitRefList>a,b,c</ExecutionUnitRefList>
                      <StartTime>2015-01-19T23:45:12+00:00</StartTime>
                      <CompleteTime>2015-01-19T23:55:12+00:00</CompleteTime>
                      <Fault>
                        <FaultStruct>
                          <FaultCode>0</FaultCode>
                          <FaultString></FaultString>
                        </FaultStruct>
                      </Fault>
                      <OperationPerformed>Install</OperationPerformed>
                    </AutonOpResultStruct>
                  </Results>
                </cwmp:AutonomousDUStateChangeComplete>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

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
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],

            body: vec![BodyElement::AutonomousDUStateChangeComplete(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn autonomous_transfer_complete_response_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">1</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:AutonomousTransferCompleteResponse/>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
            body: vec![BodyElement::AutonomousTransferCompleteResponse(
                protocol::AutonomousTransferCompleteResponse {},
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn autonomous_transfer_complete_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">1</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:AutonomousTransferComplete>
                        <AnnounceURL>http://example.com/announce</AnnounceURL>
                        <TransferURL>http://example.com/transfer</TransferURL>
                        <IsDownload>1</IsDownload>
                        <FileType>1 Firmware Upgrade Image</FileType>
                        <FileSize>10000</FileSize>
                        <TargetFileName>/bin/image</TargetFileName>
                        <FaultStruct>
                                <FaultCode>0</FaultCode>
                                <FaultString></FaultString>
                        </FaultStruct>
                        <StartTime>2016-04-07T08:43:49Z</StartTime>
                        <CompleteTime>2016-04-07T08:45:06Z</CompleteTime>
                </cwmp:AutonomousTransferComplete>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
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

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "1".to_string(),
            })],
            body: vec![BodyElement::AutonomousTransferComplete(
                protocol::AutonomousTransferComplete::new(
                    "http://example.com/announce",
                    "http://example.com/transfer",
                    1,
                    "1 Firmware Upgrade Image",
                    10000,
                    "/bin/image",
                    protocol::Fault::new(0, ""),
                    start_time,
                    complete_time,
                ),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn cancel_transfer_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:CancelTransferResponse/>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::CancelTransferResponse(
                protocol::CancelTransferResponse {},
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn cancel_transfer_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:CancelTransfer>
            <CommandKey>cmdkey</CommandKey>
          </cwmp:CancelTransfer>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::CancelTransfer(protocol::CancelTransfer::new(
                "cmdkey",
            ))],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn change_du_state_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:ChangeDUStateResponse/>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::ChangeDUStateResponse(
                protocol::ChangeDUStateResponse {},
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn change_du_state_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:ChangeDUState>
                        <CommandKey>cmdkey</CommandKey>
                        <Operations>
                                <InstallOpStruct>
                                        <URL>http://example.com/url</URL>
                                        <UUID>some-uuid</UUID>
                                        <Username>user</Username>
                                        <Password>pass</Password>
                                        <ExecutionEnvRef>env</ExecutionEnvRef>
                                </InstallOpStruct>
                        </Operations>
                </cwmp:ChangeDUState>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            body: vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn change_du_state_2() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:ChangeDUState>
                        <CommandKey>cmdkey</CommandKey>
                        <Operations>
                                <InstallOpStruct>
                                        <URL>http://example.com/url</URL>
                                        <UUID>some-uuid</UUID>
                                        <Username>user</Username>
                                        <Password>pass</Password>
                                        <ExecutionEnvRef>env</ExecutionEnvRef>
                                </InstallOpStruct>
                                <UninstallOpStruct>
                                        <URL>http://example.com/url2</URL>
                                        <UUID>some-uuid2</UUID>
                                        <ExecutionEnvRef>env2</ExecutionEnvRef>
                                </UninstallOpStruct>
                        </Operations>
                </cwmp:ChangeDUState>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            body: vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn change_du_state_3() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:ChangeDUState>
                        <CommandKey>cmdkey</CommandKey>
                        <Operations>
                                <InstallOpStruct>
                                        <URL>http://example.com/url</URL>
                                        <UUID>some-uuid</UUID>
                                        <Username>user</Username>
                                        <Password>pass</Password>
                                        <ExecutionEnvRef>env</ExecutionEnvRef>
                                </InstallOpStruct>
                                <UpdateOpStruct>
                                        <URL>http://example.com/url</URL>
                                        <UUID>some-uuid</UUID>
                                        <Username>user</Username>
                                        <Password>pass</Password>
                                        <Version>v2.0</Version>
                                </UpdateOpStruct>
                                <UninstallOpStruct>
                                        <URL>http://example.com/url2</URL>
                                        <UUID>some-uuid2</UUID>
                                        <ExecutionEnvRef>env2</ExecutionEnvRef>
                                </UninstallOpStruct>
                        </Operations>
                </cwmp:ChangeDUState>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            body: vec![BodyElement::ChangeDUState(protocol::ChangeDUState::new(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn delete_object_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:DeleteObjectResponse>
            <Status>0</Status>
          </cwmp:DeleteObjectResponse>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::DeleteObjectResponse(
                protocol::DeleteObjectResponse::new("0"),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn delete_object_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:DeleteObject>
            <ObjectName>Device.Test.</ObjectName>
            <ParameterKey>ParamKey</ParameterKey>
          </cwmp:DeleteObject>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::DeleteObject(protocol::DeleteObject::new(
                "Device.Test.",
                "ParamKey",
            ))],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn download_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:DownloadResponse>
            <Status>1</Status>
            <StartTime>2015-01-19T23:08:24Z</StartTime>
            <CompleteTime>2015-01-19T23:09:24Z</CompleteTime>
          </cwmp:DownloadResponse>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

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
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::DownloadResponse(
                protocol::DownloadResponse::new("1", start_time, complete_time),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn download_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:Download>
            <CommandKey>cmdkey</CommandKey>
            <FileType>1 Firmware Upgrade Image</FileType>
            <URL>http://example.com/url</URL>
            <Username>user</Username>
            <Password>pass</Password>
            <FileSize>123456</FileSize>
            <TargetFileName>image</TargetFileName>
            <DelaySeconds>5</DelaySeconds>
            <SuccessURL>http://example.com/success</SuccessURL>
            <FailureURL>http://example.com/failure</FailureURL>
          </cwmp:Download>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::Download(protocol::Download::new(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn download_2() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:Download>
            <CommandKey>cmdkey</CommandKey>
            <FileType>1 Firmware Upgrade Image</FileType>
            <URL>http://example.com/url</URL>
            <FileSize>123456</FileSize>
          </cwmp:Download>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::Download(protocol::Download::new(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn du_state_change_complete_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:DUStateChangeCompleteResponse/>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_aa0642e34b23820801e7642ad7cb536c".to_string(),
            })],
            body: vec![BodyElement::DUStateChangeCompleteResponse(
                protocol::DUStateChangeCompleteResponse {},
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn du_state_change_complete_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:DUStateChangeComplete>
                  <CommandKey>cmdkey</CommandKey>
                  <Results SOAP-ENC:arrayType="cwmp:OpResultStruct[1]">
                    <OpResultStruct>
                      <UUID>some-uuid</UUID>
                      <DeploymentUnitRef>uref</DeploymentUnitRef>
                      <Version>v2.1</Version>
                      <CurrentState>curState</CurrentState>
                      <Resolved>1</Resolved>
                      <ExecutionUnitRefList>a,b,c</ExecutionUnitRefList>
                      <StartTime>2015-01-19T23:45:12+00:00</StartTime>
                      <CompleteTime>2015-01-19T23:55:12+00:00</CompleteTime>
                      <Fault>
                        <FaultStruct>
                          <FaultCode>0</FaultCode>
                          <FaultString></FaultString>
                        </FaultStruct>
                      </Fault>
                    </OpResultStruct>
                  </Results>
                </cwmp:DUStateChangeComplete>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
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
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],

            body: vec![BodyElement::DUStateChangeComplete(
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
                        protocol::Fault::new(0, ""),
                    )],
                ),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn get_parameter_attributes_1() -> Result<(), String> {
        let xml: String = String::from(
            "<SOAP-ENV:Envelope
        xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\"
        xmlns:SOAP-ENC=\"http://schemas.xmlsoap.org/soap/encoding/\"
        xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"
        xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"
        xmlns:cwmp=\"urn:dslforum-org:cwmp-1-0\">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand=\"1\">API_953323a9b674bb42b7cad250b2cf0607</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:GetParameterAttributes>
            <ParameterNames>
              <string>InternetGatewayDevice.DeviceInfo.HardwareVersion</string>
              <string>InternetGatewayDevice.DeviceInfo.SoftwareVersion</string>
            </ParameterNames>
          </cwmp:GetParameterAttributes>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>",
        );
        match parse(xml) {
            Ok(_parsed) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn get_parameter_attributes_2() {
        let src = r#"<Envelope cwmp="urn:dslforum-org:cwmp-1-0">
                <Header>
                    <ID mustUnderstand="1">API_953323a9b674bb42b7cad250b2cf0607</ID>
                </Header>
                <Body>
                    <GetParameterAttributes>
                        <ParameterNames>
                            <string>InternetGatewayDevice.DeviceInfo.HardwareVersion</string>
                            <string>InternetGatewayDevice.DeviceInfo.SoftwareVersion</string>
                        </ParameterNames>
                    </GetParameterAttributes>
                </Body>
            </Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            body: vec![BodyElement::GetParameterAttributes(
                protocol::GetParameterAttributes {
                    parameternames: vec![
                        "InternetGatewayDevice.DeviceInfo.HardwareVersion".to_string(),
                        "InternetGatewayDevice.DeviceInfo.SoftwareVersion".to_string(),
                    ],
                },
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn get_parameter_attributes_response_1() {
        let src = r#"<SOAP-ENV:Envelope
        xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_953323a9b674bb42b7cad250b2cf0607</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:GetParameterAttributesResponse>
            <ParameterList SOAP-ENC:arrayType="cwmp:ParameterAttributeStruct[0002]">
              <ParameterAttributeStruct>
                <Name>InternetGatewayDevice.DeviceInfo.HardwareVersion</Name>
                <Notification>0</Notification>
                <AccessList SOAP-ENC:arrayType="xsd:string[1]">
                  <string>Subscriber</string>
                </AccessList>
              </ParameterAttributeStruct>
              <ParameterAttributeStruct>
                <Name>InternetGatewayDevice.DeviceInfo.SoftwareVersion</Name>
                <Notification>2</Notification>
                <AccessList SOAP-ENC:arrayType="xsd:string[1]">
                  <string>Subscriber</string>
                </AccessList>
              </ParameterAttributeStruct>
            </ParameterList>
          </cwmp:GetParameterAttributesResponse>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "API_953323a9b674bb42b7cad250b2cf0607".to_string(),
            })],
            body: vec![BodyElement::GetParameterAttributesResponse(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn get_parameter_values_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:GetParameterValues>
                        <ParameterNames>
                                <string>Device.IP.Interface.3.IPv4AddressNumberOfEntries</string>
                                <string>Device.IP.Interface.3.IPv6AddressNumberOfEntries</string>
                        </ParameterNames>
                </cwmp:GetParameterValues>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            body: vec![BodyElement::GetParameterValues(
                protocol::GetParameterValues::new(vec![
                    "Device.IP.Interface.3.IPv4AddressNumberOfEntries".to_string(),
                    "Device.IP.Interface.3.IPv6AddressNumberOfEntries".to_string(),
                ]),
            )],
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }

    #[test]
    fn get_parameter_values_response_1() {
        let src = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
                <cwmp:ID SOAP-ENV:mustUnderstand="1">50</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
                <cwmp:GetParameterValuesResponse>
                        <ParameterList xsi:type="SOAP-ENC:Array" SOAP-ENC:arrayType="cwmp:ParameterValueStruct[64]">
                                <ParameterValueStruct>
                                        <Name>Device.IP.Interface.3.IPv4AddressNumberOfEntries</Name>
                                        <Value xsi:type="xsd:unsignedInt">1</Value>
                                </ParameterValueStruct>
                                <ParameterValueStruct>
                                        <Name>Device.IP.Interface.3.IPv6AddressNumberOfEntries</Name>
                                        <Value xsi:type="xsd:unsignedInt">2</Value>
                                </ParameterValueStruct>
                        </ParameterList>
                </cwmp:GetParameterValuesResponse>
        </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;
        let should_be = Envelope {
            cwmp: "urn:dslforum-org:cwmp-1-0".to_string(),
            header: vec![HeaderElement::ID(ID {
                must_understand: true,
                id: "50".to_string(),
            })],
            body: vec![BodyElement::GetParameterValuesResponse(
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
        };
        let envelope: protocol::Envelope = parse(String::from(src)).unwrap();
        assert_eq!(envelope, should_be);
    }
}
