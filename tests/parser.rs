#![forbid(unsafe_code)]

extern crate cwmp;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use std::str;

fn gen_utc_date(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, mon, day)
        .unwrap_or(NaiveDate::default())
        .and_hms_opt(hour, min, sec)
        .unwrap_or(NaiveDateTime::default())
        .and_utc()
}

#[test]
fn parse_1() {
    let bogus_dt = gen_utc_date(2014, 11, 28, 12, 0, 9);
    let bogus_utc_dt = bogus_dt.with_timezone(&Utc);

    let current_time: DateTime<Utc> = match "2015-01-19T23:08:24+00:00".parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        _ => bogus_utc_dt,
    };
    test(
        include_bytes!("samples/inform_1.xml"),
        cwmp::protocol::Envelope::new(Some(cwmp::protocol::CwmpVersion::new(1,0)), 
            vec![
                cwmp::protocol::HeaderElement::ID(cwmp::protocol::ID::new(true, String::from("100"))),
                cwmp::protocol::HeaderElement::NoMoreRequests(cwmp::protocol::NoMoreRequests::new(true, 1)),
            ],
            vec![cwmp::protocol::BodyElement::Inform(cwmp::protocol::Inform::new(
                cwmp::protocol::DeviceId::new(String::from("The Company"), String::from("AA1234"), String::from("IAD_001"), String::from("S99998888")),
                vec![cwmp::protocol::EventStruct::new(String::from("2 PERIODIC"), String::from(""))],
                1,
                current_time,
                0,
                vec![
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceSummary"),String::from("xsd:string"),String::from("InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SpecVersion"),String::from("xsd:string"),String::from("1.0")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.HardwareVersion"),String::from("xsd:string"),String::from("HW1.0")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.SoftwareVersion"),String::from("xsd:string"),String::from("V1.00(beta)")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.DeviceInfo.ProvisioningCode"),String::from("xsd:string"),String::from("")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ConnectionRequestURL"),String::from("xsd:string"),String::from("http://2.2.2.2:7676/CWMP/ConnectionRequest")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.ManagementServer.ParameterKey"),String::from("xsd:string"),String::from("")),
                    cwmp::protocol::ParameterValue::new(String::from("InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress"),String::from("xsd:string"),String::from("2.2.2.2")),
                ],
            ))],
       ),
    )
}

fn test(input: &[u8], expected: cwmp::protocol::Envelope) {
    match str::from_utf8(input) {
        Ok(s) => match cwmp::parse(s) {
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
