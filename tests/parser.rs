#![forbid(unsafe_code)]

extern crate cwmp;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use std::str;

#[test]
fn parse_1() {
    let bogus_dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let bogus_utc_dt = bogus_dt.with_timezone(&Utc);

    let current_time: DateTime<Utc> = match "2015-01-19T23:08:24+00:00".parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        _ => bogus_utc_dt,
    };
    test(
        include_bytes!("samples/inform_1.xml"),
        cwmp::protocol::Envelope::new("urn:dslforum-org:cwmp-1-0", 
            vec![
                cwmp::protocol::HeaderElement::ID(cwmp::protocol::ID::new(true, "100")),
                cwmp::protocol::HeaderElement::NoMoreRequests(cwmp::protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![cwmp::protocol::BodyElement::Inform(cwmp::protocol::Inform::new(
                cwmp::protocol::DeviceId::new("The Company", "AA1234", "IAD_001", "S99998888"),
                vec![cwmp::protocol::EventStruct::new("2 PERIODIC", "")],
                1,
                current_time,
                0,
                vec![
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.DeviceSummary","xsd:string","InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)"),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.SpecVersion","xsd:string","1.0"),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.HardwareVersion","xsd:string","HW1.0"),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.SoftwareVersion","xsd:string","V1.00(beta)"),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.DeviceInfo.ProvisioningCode","xsd:string",""),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.ManagementServer.ConnectionRequestURL","xsd:string","http://2.2.2.2:7676/CWMP/ConnectionRequest"),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.ManagementServer.ParameterKey","xsd:string",""),
                    cwmp::protocol::ParameterValue::new("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress","xsd:string","2.2.2.2"),
                ],
            ))],
       ),
    )
}

fn test(input: &[u8], expected: cwmp::protocol::Envelope) {
    match str::from_utf8(input) {
        Ok(s) => match cwmp::parse(s.to_string()) {
            Ok(envelope) => {
                if envelope != expected {
                    println!("{:?}", envelope);
                    println!("{:?}", expected);
                    panic!("No match")
                }
            }
            Err(e) => panic!("cwmp::parse did not return an Ok result: {:?}", e),
        },
        Err(e) => panic!("Could not parse utf8 string: {:?}", e),
    }
}
