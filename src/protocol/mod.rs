use std::error::Error;
use std::fmt;
use std::io::Write;
use xml::writer::XmlEvent;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
use chrono::NaiveDate;
#[cfg(test)]
use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

mod addobject;
mod addobjectresponse;
mod allqueuedtransfers;
mod argstruct;
mod autonomousdustatechangecomplete;
mod autonomousdustatechangecompleteresponse;
mod autonomoustransfercomplete;
mod autonomoustransfercompleteresponse;
mod autonopresult;
mod bodyelement;
mod canceltransfer;
mod canceltransferresponse;
mod changedustate;
mod changedustateresponse;
mod cwmpversion;
mod deleteobject;
mod deleteobjectresponse;
mod deviceid;
mod download;
mod downloadresponse;
mod dustatechangecomplete;
mod dustatechangecompleteresponse;
mod envelope;
mod eventstruct;
mod factoryreset;
mod factoryresetresponse;
mod fault;
mod getallqueuedtransfers;
mod getallqueuedtransfersresponse;
mod getoptions;
mod getoptionsresponse;
mod getparameterattributes;
mod getparameterattributesresponse;
mod getparameternames;
mod getparameternamesresponse;
mod getparametervalues;
mod getparametervaluesresponse;
mod getqueuedtransfers;
mod getqueuedtransfersresponse;
mod getrpcmethods;
mod getrpcmethodsresponse;
mod headerelement;
mod holdrequests;
mod id;
mod inform;
mod informresponse;
mod installop;
mod kicked;
mod kickedresponse;
mod nomorerequests;
mod opresult;
mod optionstruct;
mod parameterattribute;
mod parameterinfostruct;
mod parametervalue;
mod queuedtransferstruct;
mod reboot;
mod rebootresponse;
mod requestdownload;
mod requestdownloadresponse;
mod scheduledownload;
mod scheduledownloadresponse;
mod scheduleinform;
mod scheduleinformresponse;
mod sessiontimeout;
mod setparameterattributes;
mod setparameterattributesresponse;
mod setparameterattributesstruct;
mod setparametervalues;
mod setparametervaluesresponse;
mod setvouchers;
mod setvouchersresponse;
mod supportedcwmpversions;
mod timewindow;
mod transfercomplete;
mod transfercompleteresponse;
mod uninstallop;
mod updateop;
mod upload;
mod uploadresponse;
mod usecwmpversion;

pub use addobject::AddObject;
pub use addobjectresponse::AddObjectResponse;
pub use allqueuedtransfers::AllQueuedTransfers;
pub use argstruct::ArgStruct;
pub use autonomousdustatechangecomplete::AutonomousDUStateChangeComplete;
pub use autonomousdustatechangecompleteresponse::AutonomousDUStateChangeCompleteResponse;
pub use autonomoustransfercomplete::AutonomousTransferComplete;
pub use autonomoustransfercompleteresponse::AutonomousTransferCompleteResponse;
pub use autonopresult::AutonOpResult;
pub use bodyelement::BodyElement;
pub use canceltransfer::CancelTransfer;
pub use canceltransferresponse::CancelTransferResponse;
pub use changedustate::ChangeDUState;
pub use changedustateresponse::ChangeDUStateResponse;
pub use cwmpversion::CwmpVersion;
pub use deleteobject::DeleteObject;
pub use deleteobjectresponse::DeleteObjectResponse;
pub use deviceid::DeviceId;
pub use download::Download;
pub use downloadresponse::DownloadResponse;
pub use dustatechangecomplete::DUStateChangeComplete;
pub use dustatechangecompleteresponse::DUStateChangeCompleteResponse;
pub use envelope::Envelope;
pub use eventstruct::EventStruct;
pub use factoryreset::FactoryReset;
pub use factoryresetresponse::FactoryResetResponse;
pub use fault::{Fault, FaultDetail, FaultStruct};
pub use getallqueuedtransfers::GetAllQueuedTransfers;
pub use getallqueuedtransfersresponse::GetAllQueuedTransfersResponse;
pub use getoptions::GetOptions;
pub use getoptionsresponse::GetOptionsResponse;
pub use getparameterattributes::GetParameterAttributes;
pub use getparameterattributesresponse::GetParameterAttributesResponse;
pub use getparameternames::GetParameterNames;
pub use getparameternamesresponse::GetParameterNamesResponse;
pub use getparametervalues::GetParameterValues;
pub use getparametervaluesresponse::GetParameterValuesResponse;
pub use getqueuedtransfers::GetQueuedTransfers;
pub use getqueuedtransfersresponse::GetQueuedTransfersResponse;
pub use getrpcmethods::GetRPCMethods;
pub use getrpcmethodsresponse::GetRPCMethodsResponse;
pub use headerelement::HeaderElement;
pub use holdrequests::HoldRequests;
pub use id::ID;
pub use inform::Inform;
pub use informresponse::InformResponse;
pub use installop::InstallOp;
pub use kicked::Kicked;
pub use kickedresponse::KickedResponse;
pub use nomorerequests::NoMoreRequests;
pub use opresult::OpResult;
pub use optionstruct::OptionStruct;
pub use parameterattribute::ParameterAttribute;
pub use parameterinfostruct::ParameterInfoStruct;
pub use parametervalue::ParameterValue;
pub use queuedtransferstruct::QueuedTransferStruct;
pub use reboot::Reboot;
pub use rebootresponse::RebootResponse;
pub use requestdownload::RequestDownload;
pub use requestdownloadresponse::RequestDownloadResponse;
pub use scheduledownload::ScheduleDownload;
pub use scheduledownloadresponse::ScheduleDownloadResponse;
pub use scheduleinform::ScheduleInform;
pub use scheduleinformresponse::ScheduleInformResponse;
pub use sessiontimeout::SessionTimeout;
pub use setparameterattributes::SetParameterAttributes;
pub use setparameterattributesresponse::SetParameterAttributesResponse;
pub use setparametervalues::SetParameterValues;
pub use setparametervaluesresponse::SetParameterValuesResponse;
pub use setvouchers::SetVouchers;
pub use setvouchersresponse::SetVouchersResponse;
pub use supportedcwmpversions::SupportedCWMPVersions;
pub use timewindow::TimeWindow;
pub use transfercomplete::TransferComplete;
pub use transfercompleteresponse::TransferCompleteResponse;
pub use uninstallop::UninstallOp;
pub use updateop::UpdateOp;
pub use upload::Upload;
pub use uploadresponse::UploadResponse;
pub use usecwmpversion::UseCWMPVersion;

#[cfg(test)]
const VALID_CHARS: &[u8] =
    b" !#$%&()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ^_abcdefghijklmnopqrstuvwxyz{|}~";

fn bool2str(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
fn str2bool(s: &str) -> bool {
    s != "0"
}

fn write_simple<W: Write>(
    writer: &mut xml::EventWriter<W>,
    name: &str,
    value: &str,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(value)?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_empty_tag<W: Write>(
    writer: &mut xml::EventWriter<W>,
    name: &str,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_fault_struct<W: Write>(
    writer: &mut xml::EventWriter<W>,
    fault: &FaultStruct,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element("FaultStruct"))?;
    write_simple(writer, "FaultCode", &fault.code.to_string())?;
    write_simple(writer, "FaultString", fault.string.0.as_ref())?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_fault<W: Write>(
    writer: &mut xml::EventWriter<W>,
    fault: &FaultStruct,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element("Fault"))?;
    write_fault_struct(writer, fault)?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

#[cfg(test)]
#[must_use]
pub fn gen_utc_date(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, mon, day)
        .unwrap_or_default()
        .and_hms_opt(hour, min, sec)
        .unwrap_or_default()
        .and_utc()
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct XmlSafeString(pub String);

impl From<&str> for XmlSafeString {
    fn from(s: &str) -> Self {
        XmlSafeString(s.to_string())
    }
}

impl XmlSafeString {
    #[must_use]
    pub fn new() -> XmlSafeString {
        XmlSafeString(String::new())
    }
}

#[must_use]
pub fn convert_to_xml_safe_strings(input: &[&str]) -> Vec<XmlSafeString> {
    input
        .iter()
        .map(|&s| XmlSafeString(s.to_string())) // Convert &str to XmlSafeString
        .collect() // Collect into Vec<XmlSafeString>
}

#[cfg(test)]
impl Arbitrary for XmlSafeString {
    fn arbitrary(g: &mut Gen) -> XmlSafeString {
        // Generate a random string of valid XML-safe characters
        let s: String = (0..g.size())
            .map(|_| {
                *g.choose(VALID_CHARS).unwrap_or(&b' ') as char // safely choose a valid char
            })
            .collect();

        XmlSafeString(s)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = XmlSafeString>> {
        let shrunk = self.0.shrink().map(XmlSafeString);
        Box::new(shrunk)
    }
}

pub enum GenerateError {
    FromUtf8Error(std::string::FromUtf8Error),
    XmlError(xml::writer::Error),
}

impl From<xml::writer::Error> for GenerateError {
    fn from(e: xml::writer::Error) -> GenerateError {
        GenerateError::XmlError(e)
    }
}
impl From<std::string::FromUtf8Error> for GenerateError {
    fn from(e: std::string::FromUtf8Error) -> GenerateError {
        GenerateError::FromUtf8Error(e)
    }
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateError::FromUtf8Error(ref err) => std::fmt::Display::fmt(&err, f),
            GenerateError::XmlError(ref err) => std::fmt::Display::fmt(&err, f),
        }
    }
}
impl fmt::Debug for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateError::FromUtf8Error(ref err) => std::fmt::Debug::fmt(&err, f),
            GenerateError::XmlError(ref err) => std::fmt::Debug::fmt(&err, f),
        }
    }
}

// private functions
fn extract_attribute(
    attributes: &[xml::attribute::OwnedAttribute],
    attrib_name: &str,
) -> XmlSafeString {
    let f = attributes
        .iter()
        .find(|&x| x.name.local_name == attrib_name);
    match f {
        Some(e) => XmlSafeString::from(e.value.as_ref()),
        None => XmlSafeString::new(),
    }
}

fn cwmp_prefix(envelope_has_cwmp_version: bool, postfix: &str) -> String {
    if envelope_has_cwmp_version {
        format!("cwmp:{postfix}")
    } else {
        postfix.to_string()
    }
}

// parses urns like "urn:dslforum-org:cwmp-1-0" into
// CwmpVersion, i.e. (1,0) in this example
fn cwmp_urn_to_version(urn: &str) -> CwmpVersion {
    let mut version_string: Vec<&str> = urn.split('-').collect();
    let mi = if let Some(mi_s) = version_string.pop() {
        parse_to_int(mi_s, 0)
    } else {
        0
    };
    let ma = if let Some(ma_s) = version_string.pop() {
        parse_to_int(ma_s, 0)
    } else {
        0
    };

    CwmpVersion::new(ma, mi)
}

pub trait Parseable {}
impl Parseable for u32 {}
impl Parseable for u16 {}
impl Parseable for i32 {}
impl Parseable for u8 {}

fn parse_to_int<T: Parseable + std::str::FromStr>(chars: &str, default: T) -> T {
    match chars.parse::<T>() {
        Ok(parsed) => parsed,
        _ => default,
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        State {
            last_text: String::new(),
            envelope: Envelope::default(),
            path: vec![],
            error: None,
        }
    }
    pub fn start_handler(
        &mut self,
        name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
        namespace: &xml::namespace::Namespace,
    ) {
        // push a copy of the name into the current path
        self.path.push(name.local_name.to_string());

        self.envelope
            .start_handler(&self.path, name, attributes, namespace);
    }

    pub fn end_handler(&mut self, name: &xml::name::OwnedName) {
        // pop the name from the current path
        self.path.pop();
        self.envelope.end_handler(&self.path, name);
    }
    pub fn characters(&mut self, characters: &String) {
        self.last_text = String::from(characters);
        self.envelope.characters(&self.path, characters);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub struct State {
    pub path: Vec<String>,
    pub last_text: String,
    pub envelope: Envelope,
    pub error: Option<Box<dyn Error>>,
}
