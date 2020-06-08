cwmp, an implementation of the CWMP protocol in Rust
====================================================

![CI](https://github.com/jdalberg/cwmp/workflows/CI/badge.svg)

Building and using
------------------
*not on crates.io yet

Parsing
-------

```rust,no_run
extern crate cwmp;

use cwmp::parse;

fn main() {
   let s = r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
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

    match cwmp::parse(s.to_string()) {
        Ok(envelope) => println!("{:?}", envelope),
        Err(e) => println!("Error [{:?}] occured", e)
    }
}
```