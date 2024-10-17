use chrono::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cwmp::generate;
use cwmp::protocol::*;

fn gen_utc_date(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, mon, day)
        .unwrap_or(NaiveDate::default())
        .and_hms_opt(hour, min, sec)
        .unwrap_or(NaiveDateTime::default())
        .and_utc()
}
fn criterion_benchmark(c: &mut Criterion) {
    let e: Envelope = Envelope::new(Some(CwmpVersion::new(1,0)), vec![HeaderElement::ID(ID::new(true, "1234".to_string()))],vec![BodyElement::Inform(Inform::new(
        DeviceId::new(String::from("The Company"), String::from("AA1234"), String::from("IAD_001"), String::from("S99998888")),
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
    ))]);
    c.bench_function("generate add_object", |b| {
        b.iter(|| generate(black_box(&e)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
