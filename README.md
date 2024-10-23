# cwmp, an implementation of the CWMP protocol in Rust
====================================================

![Build](https://github.com/jdalberg/cwmp/workflows/Build/badge.svg)

## Building and using
*not on crates.io yet

## Code coverage
The tarpaulin report should be generated on every commit really, at least before every release.

```bash
cargo tarpaulin -t 240 --out html
```

That will generate the tarpaulin-report.html file in the base project folder, which should be pushed to git.

## Parsing and Generating

```rust,no_run
use cwmp::{generate, parse};
use cwmp::protocol::*;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

fn main() {
  let input: Envelope = Envelope::new(
      Some(CwmpVersion::new(1,0)), 
      vec![HeaderElement::ID(ID::new(true,"1234"))], 
      vec![BodyElement::Inform(
              Inform::new(
                  DeviceId::new("MyManufacturer", "OUI", "MyProductClass", "S123456"),
                  vec![EventStruct::new("2 PERIODIC", "")],
                  1,
                  Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
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
  
              )
          )
      ]);
  match generate(&input) {
        Ok(xml) => match parse(&xml) {
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

