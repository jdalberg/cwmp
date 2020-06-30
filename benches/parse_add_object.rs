use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cwmp::parse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse add_object", |b| b.iter(|| parse(black_box(String::from(r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
    <SOAP-ENV:Header>
      <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
    </SOAP-ENV:Header>
    <SOAP-ENV:Body>
      <cwmp:AddObject>
        <ObjectName>Device.Test.</ObjectName>
        <ParameterKey>ParamKey</ParameterKey>
      </cwmp:AddObject>
    </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
