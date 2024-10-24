use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cwmp::parse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse add_object", |b| b.iter(|| parse(black_box(r#"<SOAP-ENV:Envelope
    SOAP-ENV:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/"
    xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"
    xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
    xmlns:cwmp="urn:dslforum-org:cwmp-1-0"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <SOAP-ENV:Header>
      <cwmp:ID SOAP-ENV:mustUnderstand="1">100</cwmp:ID>
      <cwmp:NoMoreRequests SOAP-ENV:mustUnderstand="1">1</cwmp:NoMoreRequests>
    </SOAP-ENV:Header>
    <SOAP-ENV:Body>
      <cwmp:Inform>
        <DeviceId>
          <Manufacturer>The Company</Manufacturer>
          <OUI>AA1234</OUI>
          <ProductClass>IAD_001</ProductClass>
          <SerialNumber>S99998888</SerialNumber>
        </DeviceId>
        <Event SOAP-ENC:arrayType="cwmp:EventStruct[1]">
          <EventStruct>
            <EventCode>2 PERIODIC</EventCode>
            <CommandKey></CommandKey>
          </EventStruct>
        </Event>
        <MaxEnvelopes>1</MaxEnvelopes>
        <CurrentTime>2015-01-19T23:08:24+00:00</CurrentTime>
        <RetryCount>0</RetryCount>
        <ParameterList SOAP-ENC:arrayType="cwmp:ParameterValueStruct[8]">
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.DeviceSummary</Name>
            <Value xsi:type="xsd:string">InternetGatewayDevice:1.4[](Baseline:1, EthernetLAN:1, WiFiLAN:1, EthernetWAN:1, ADSLWAN:1, IPPing:1, DSLDiagnostics:1, Time:1), VoiceService:1.0[1](Endpoint:1, SIPEndpoint:1)</Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.DeviceInfo.SpecVersion</Name>
            <Value xsi:type="xsd:string">1.0</Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.DeviceInfo.HardwareVersion</Name>
            <Value xsi:type="xsd:string">HW1.0</Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.DeviceInfo.SoftwareVersion</Name>
            <Value xsi:type="xsd:string">V1.00(beta)</Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.DeviceInfo.ProvisioningCode</Name>
            <Value xsi:type="xsd:string"></Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.ManagementServer.ConnectionRequestURL</Name>
            <Value xsi:type="xsd:string">http://2.2.2.2:7676/CWMP/ConnectionRequest</Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.ManagementServer.ParameterKey</Name>
            <Value xsi:type="xsd:string"></Value>
          </ParameterValueStruct>
          <ParameterValueStruct>
            <Name>InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress</Name>
            <Value xsi:type="xsd:string">2.2.2.2</Value>
          </ParameterValueStruct>
        </ParameterList>
      </cwmp:Inform>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
