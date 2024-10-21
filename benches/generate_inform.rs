use chrono::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cwmp::generate;
use cwmp::protocol::{
    BodyElement, CwmpVersion, DeviceId, Envelope, EventStruct, HeaderElement, Inform,
    ParameterValue, ID,
};

fn gen_utc_date(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, mon, day)
        .unwrap_or_default()
        .and_hms_opt(hour, min, sec)
        .unwrap_or_default()
        .and_utc()
}
fn criterion_benchmark(c: &mut Criterion) {
    let e: Envelope = Envelope::new(Some(CwmpVersion::new(1,0)), vec![HeaderElement::ID(ID::new(true, "1234"))],vec![BodyElement::Inform(Inform::new(
        DeviceId::new("The Company", "AA1234", "IAD_001", "S99998888"),
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
    ))]);
    c.bench_function("generate add_object", |b| {
        b.iter(|| generate(black_box(&e)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
