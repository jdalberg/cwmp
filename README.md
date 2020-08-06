cwmp, an implementation of the CWMP protocol in Rust
====================================================

![Build](https://github.com/jdalberg/cwmp/workflows/Build/badge.svg) [![codecov](https://codecov.io/gh/jdalberg/cwmp/branch/master/graph/badge.svg)](https://codecov.io/gh/jdalberg/cwmp)

Building and using
------------------
*not on crates.io yet

Parsing and Generating
----------------------

```rust,no_run
use cwmp::{generate, parse};
use cwmp::protocol::*;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

fn main() {
  let input: Envelope = Envelope::new(
      Some(CwmpVersion::new(1,0)), 
      vec![HeaderElement::ID(ID::new(true,String::from("1234")))], 
      vec![BodyElement::Inform(
              Inform::new(
                  DeviceId::new(String::from("MyManufacturer"), String::from("OUI"), String::from("MyProductClass"), String::from("S123456")),
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
  
              )
          )
      ]);
  match generate(&input) {
        Ok(xml) => match parse(xml) {
            Ok(envelope) => if envelope != input {
                println!("Things aren't really doing what we want");
            }
            else {
                println!("Things are generating and parsing perfectly!");
            },
            Err(e) => println!("An error occured during parse: {:?}", e)
        },
        Err(e) => 
            println!("An error occurred during generate: {:?}", e)
    } 

}
```

