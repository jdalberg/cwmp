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
    use protocol::Envelope;
    extern crate quickcheck;

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
