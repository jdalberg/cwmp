#![feature(test)]

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
    let config = ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(true);
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
        None => Ok(state.envelope),
        Some(b) => Err(b),
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
    use chrono::Utc;
    use protocol::*;
    extern crate quickcheck;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_generate_add_object(b: &mut Bencher) {
        let e: Envelope = Envelope::new(
            Some(CwmpVersion::new(1, 0)),
            vec![HeaderElement::ID(ID::new(true, "1234".to_string()))],
            vec![BodyElement::AddObject(AddObject::new(
                "foo".to_string(),
                "key".to_string(),
            ))],
        );
        b.iter(|| generate(&e));
    }

    #[bench]
    fn bench_generate_inform(b: &mut Bencher) {
        let e: Envelope = Envelope::new(Some(CwmpVersion::new(1,0)), vec![HeaderElement::ID(ID::new(true, "1234".to_string()))],vec![BodyElement::Inform(Inform::new(
            DeviceId::new(String::from("The Company"), String::from("AA1234"), String::from("IAD_001"), String::from("S99998888")),
            vec![EventStruct::new(String::from("2 PERIODIC"), String::from(""))],
            1,
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
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
        b.iter(|| generate(&e));
    }

    #[bench]
    fn bench_parse_add_object(b: &mut Bencher) {
        b.iter(|| parse(String::from(r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
        <SOAP-ENV:Header>
          <cwmp:ID SOAP-ENV:mustUnderstand="1">API_aa0642e34b23820801e7642ad7cb536c</cwmp:ID>
        </SOAP-ENV:Header>
        <SOAP-ENV:Body>
          <cwmp:AddObject>
            <ObjectName>Device.Test.</ObjectName>
            <ParameterKey>ParamKey</ParameterKey>
          </cwmp:AddObject>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#)));
    }

    #[bench]
    fn bench_parse_inform(b: &mut Bencher) {
        b.iter(|| parse(String::from(r#"<SOAP-ENV:Envelope
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
      </SOAP-ENV:Envelope>"#)));
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
