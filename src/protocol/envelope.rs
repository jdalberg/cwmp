use log::warn;
use xml::{writer::XmlEvent, EmitterConfig};

use super::{
    cwmp_urn_to_version, parse_to_int, str2bool, AddObject, AddObjectResponse,
    AutonomousDUStateChangeComplete, AutonomousDUStateChangeCompleteResponse,
    AutonomousTransferComplete, AutonomousTransferCompleteResponse, BodyElement, CancelTransfer,
    CancelTransferResponse, ChangeDUState, ChangeDUStateResponse, CwmpVersion,
    DUStateChangeComplete, DUStateChangeCompleteResponse, DeleteObject, DeleteObjectResponse,
    Download, DownloadResponse, FactoryReset, FactoryResetResponse, Fault, GenerateError,
    GetAllQueuedTransfers, GetAllQueuedTransfersResponse, GetOptions, GetOptionsResponse,
    GetParameterAttributes, GetParameterAttributesResponse, GetParameterNames,
    GetParameterNamesResponse, GetParameterValues, GetParameterValuesResponse, GetQueuedTransfers,
    GetQueuedTransfersResponse, GetRPCMethods, GetRPCMethodsResponse, HeaderElement, HoldRequests,
    Inform, InformResponse, Kicked, KickedResponse, NoMoreRequests, Reboot, RebootResponse,
    RequestDownload, RequestDownloadResponse, ScheduleDownload, ScheduleDownloadResponse,
    ScheduleInform, ScheduleInformResponse, SessionTimeout, SetParameterAttributes,
    SetParameterAttributesResponse, SetParameterValues, SetParameterValuesResponse, SetVouchers,
    SetVouchersResponse, SupportedCWMPVersions, TransferComplete, TransferCompleteResponse, Upload,
    UploadResponse, UseCWMPVersion, ID,
};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Envelope {
    pub cwmp_version: Option<CwmpVersion>,
    pub header: Vec<HeaderElement>,
    pub body: Vec<BodyElement>,
}
impl Envelope {
    #[must_use]
    pub fn new(
        cwmp_version: Option<CwmpVersion>,
        header: Vec<HeaderElement>,
        body: Vec<BodyElement>,
    ) -> Self {
        Envelope {
            cwmp_version,
            header,
            body,
        }
    }
    #[must_use]
    pub fn cwmp_version(self) -> Option<CwmpVersion> {
        self.cwmp_version
    }
    #[must_use]
    pub fn header(self) -> Vec<HeaderElement> {
        self.header
    }
    #[must_use]
    pub fn body(self) -> Vec<BodyElement> {
        self.body
    }

    #[must_use]
    pub fn is_inform(self) -> bool {
        self.body
            .iter()
            .any(|v| matches!(v, BodyElement::Inform(_)))
    }

    /// Generate XML for `Envelope`
    ///     
    /// # Errors
    /// Any errors encountered while writing to `writer` will be returned.
    #[allow(clippy::too_many_lines)]
    pub fn generate(&self) -> Result<String, GenerateError> {
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(Vec::new());

        let mut start_event = XmlEvent::start_element("Envelope")
            .ns("SOAP-ENV", "http://schemas.xmlsoap.org/soap/envelope/")
            .ns("SOAP-ENC", "http://schemas.xmlsoap.org/soap/encoding/")
            .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
            .ns("xsd", "http://www.w3.org/2001/XMLSchema");

        if let Some(cwmp) = &self.cwmp_version {
            start_event = start_event.ns(
                "cwmp",
                format!("urn:dslforum-org:cwmp-{}-{}", cwmp.major, cwmp.minor),
            );
        }
        start_event = start_event.attr(
            "SOAP-ENV:encodingStyle",
            "http://schemas.xmlsoap.org/soap/encoding/",
        );

        writer.write(start_event)?;

        // now generate the header elements
        let start_header = XmlEvent::start_element("SOAP-ENV:Header");
        writer.write(start_header)?;

        for he in &self.header {
            match he {
                HeaderElement::ID(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                HeaderElement::HoldRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::NoMoreRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::SessionTimeout(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::SupportedCWMPVersions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::UseCWMPVersion(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
            };
        }

        let end_header: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_header)?;

        // now generate the body elemenets
        let body_start = XmlEvent::start_element("SOAP-ENV:Body");
        writer.write(body_start)?;

        for be in &self.body {
            match be {
                BodyElement::AddObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AddObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousDUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousDUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousTransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousTransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::CancelTransferResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::CancelTransfer(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ChangeDUStateResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ChangeDUState(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DeleteObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DeleteObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Download(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::DUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::FactoryResetResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::FactoryReset(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Fault(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::GetAllQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetAllQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetOptionsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetOptions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterNamesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterNames(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetRPCMethodsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetRPCMethods(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::InformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Inform(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::KickedResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Kicked(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RebootResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Reboot(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RequestDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::RequestDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleInformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleInform(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetVouchersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetVouchers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::TransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::TransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Upload(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::UploadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
            }
        }

        let body_end: XmlEvent = XmlEvent::end_element().into();
        writer.write(body_end)?;

        // End the Envelope
        let end_event: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_event)?;

        // Excape invalid XML characters and return the XML
        Ok(String::from_utf8(writer.into_inner())?)
    }

    #[allow(clippy::too_many_lines)]
    pub fn start_handler(
        &mut self,
        path: &[String],
        name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
        namespace: &xml::namespace::Namespace,
    ) {
        // match out all the elements in path. If the path goes into body,
        // call the start_handler for each element in the Body vector
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope"] => {
                // search through the namespaces to find a cwmp value
                match namespace.get("cwmp") {
                    Some(ns) => self.cwmp_version = Some(cwmp_urn_to_version(ns)),
                    None => self.cwmp_version = None,
                }
            }
            ["Envelope", "Header", header_element] => {
                // check if there is a mustUnderstand attribute, and if so, check
                // if we actually understand the header_element given

                // the mustUnderstand attributes is used in more than one header element
                let must_understand_filter = attributes
                    .iter()
                    .find(|&x| x.name.local_name == "mustUnderstand");

                let must_understand: bool = if let Some(mua) = must_understand_filter {
                    str2bool(&mua.value.to_string())
                } else {
                    true
                };
                match *header_element {
                    "ID" => self
                        .header
                        .push(HeaderElement::ID(ID::new(must_understand, ""))),
                    "NoMoreRequests" => {
                        self.header
                            .push(HeaderElement::NoMoreRequests(NoMoreRequests::new(
                                must_understand,
                                0,
                            )));
                    }
                    "HoldRequests" => {
                        self.header
                            .push(HeaderElement::HoldRequests(HoldRequests::new(
                                must_understand,
                                false,
                            )));
                    }
                    "SessionTimeout" => {
                        self.header
                            .push(HeaderElement::SessionTimeout(SessionTimeout::new(
                                must_understand,
                                0,
                            )));
                    }
                    "SupportedCWMPVersions" => {
                        self.header.push(HeaderElement::SupportedCWMPVersions(
                            SupportedCWMPVersions::new(must_understand, ""),
                        ));
                    }
                    "UseCWMPVersion" => {
                        self.header
                            .push(HeaderElement::UseCWMPVersion(UseCWMPVersion::new(
                                must_understand,
                                String::new(),
                            )));
                    }
                    _ => {}
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                // Start of a new body element, create an instance of the correct
                // type, push the empty instance on to the stack and call the start
                // handler on the type
                if path_pattern.len() == 3 {
                    // an actual new Body element
                    match *body_element {
                        "AddObject" => self.body.push(BodyElement::AddObject(AddObject::default())),
                        "AddObjectResponse" => self
                            .body
                            .push(BodyElement::AddObjectResponse(AddObjectResponse::default())),
                        "AutonomousDUStateChangeCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousDUStateChangeCompleteResponse(
                                    AutonomousDUStateChangeCompleteResponse {},
                                ));
                        }
                        "AutonomousDUStateChangeComplete" => {
                            self.body.push(BodyElement::AutonomousDUStateChangeComplete(
                                AutonomousDUStateChangeComplete::default(),
                            ));
                        }
                        "AutonomousTransferCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousTransferCompleteResponse(
                                    AutonomousTransferCompleteResponse {},
                                ));
                        }
                        "AutonomousTransferComplete" => {
                            self.body.push(BodyElement::AutonomousTransferComplete(
                                AutonomousTransferComplete::default(),
                            ));
                        }
                        "CancelTransferResponse" => self.body.push(
                            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
                        ),
                        "CancelTransfer" => self
                            .body
                            .push(BodyElement::CancelTransfer(CancelTransfer::default())),
                        "ChangeDUStateResponse" => self
                            .body
                            .push(BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {})),
                        "ChangeDUState" => self
                            .body
                            .push(BodyElement::ChangeDUState(ChangeDUState::default())),
                        "DeleteObjectResponse" => self.body.push(
                            BodyElement::DeleteObjectResponse(DeleteObjectResponse::default()),
                        ),
                        "DeleteObject" => self
                            .body
                            .push(BodyElement::DeleteObject(DeleteObject::default())),
                        "DownloadResponse" => self
                            .body
                            .push(BodyElement::DownloadResponse(DownloadResponse::default())),
                        "Download" => self.body.push(BodyElement::Download(Download::default())),
                        "DUStateChangeCompleteResponse" => {
                            self.body.push(BodyElement::DUStateChangeCompleteResponse(
                                DUStateChangeCompleteResponse {},
                            ));
                        }
                        "DUStateChangeComplete" => self.body.push(
                            BodyElement::DUStateChangeComplete(DUStateChangeComplete::default()),
                        ),
                        "FactoryResetResponse" => self
                            .body
                            .push(BodyElement::FactoryResetResponse(FactoryResetResponse {})),
                        "FactoryReset" => {
                            self.body.push(BodyElement::FactoryReset(FactoryReset {}));
                        }
                        "Fault" => self.body.push(BodyElement::Fault(Fault::default())),
                        "GetAllQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetAllQueuedTransfersResponse(
                                GetAllQueuedTransfersResponse::default(),
                            ));
                        }
                        "GetAllQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {})),
                        "GetOptionsResponse" => self.body.push(BodyElement::GetOptionsResponse(
                            GetOptionsResponse::default(),
                        )),
                        "GetOptions" => self
                            .body
                            .push(BodyElement::GetOptions(GetOptions::default())),
                        "GetParameterAttributes" => self.body.push(
                            BodyElement::GetParameterAttributes(GetParameterAttributes::default()),
                        ),
                        "GetParameterAttributesResponse" => {
                            self.body.push(BodyElement::GetParameterAttributesResponse(
                                GetParameterAttributesResponse::default(),
                            ));
                        }
                        "GetParameterNamesResponse" => {
                            self.body.push(BodyElement::GetParameterNamesResponse(
                                GetParameterNamesResponse::default(),
                            ));
                        }
                        "GetParameterNames" => self
                            .body
                            .push(BodyElement::GetParameterNames(GetParameterNames::default())),
                        "GetParameterValues" => self.body.push(BodyElement::GetParameterValues(
                            GetParameterValues::default(),
                        )),
                        "GetParameterValuesResponse" => {
                            self.body.push(BodyElement::GetParameterValuesResponse(
                                GetParameterValuesResponse::default(),
                            ));
                        }
                        "GetQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetQueuedTransfersResponse(
                                GetQueuedTransfersResponse::default(),
                            ));
                        }
                        "GetQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetQueuedTransfers(GetQueuedTransfers {})),
                        "GetRPCMethodsResponse" => self.body.push(
                            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::default()),
                        ),
                        "GetRPCMethods" => {
                            self.body.push(BodyElement::GetRPCMethods(GetRPCMethods {}));
                        }
                        "InformResponse" => self
                            .body
                            .push(BodyElement::InformResponse(InformResponse::new(1))),
                        "Inform" => self.body.push(BodyElement::Inform(Inform::default())),
                        "KickedResponse" => self
                            .body
                            .push(BodyElement::KickedResponse(KickedResponse::default())),
                        "Kicked" => self.body.push(BodyElement::Kicked(Kicked::default())),
                        "RebootResponse" => self
                            .body
                            .push(BodyElement::RebootResponse(RebootResponse::default())),
                        "Reboot" => self.body.push(BodyElement::Reboot(Reboot::default())),
                        "RequestDownloadResponse" => self.body.push(
                            BodyElement::RequestDownloadResponse(RequestDownloadResponse {}),
                        ),
                        "RequestDownload" => self
                            .body
                            .push(BodyElement::RequestDownload(RequestDownload::default())),
                        "ScheduleDownloadResponse" => self.body.push(
                            BodyElement::ScheduleDownloadResponse(ScheduleDownloadResponse {}),
                        ),
                        "ScheduleDownload" => self
                            .body
                            .push(BodyElement::ScheduleDownload(ScheduleDownload::default())),
                        "ScheduleInformResponse" => self.body.push(
                            BodyElement::ScheduleInformResponse(ScheduleInformResponse {}),
                        ),
                        "ScheduleInform" => self
                            .body
                            .push(BodyElement::ScheduleInform(ScheduleInform::default())),
                        "SetParameterAttributesResponse" => {
                            self.body.push(BodyElement::SetParameterAttributesResponse(
                                SetParameterAttributesResponse {},
                            ));
                        }
                        "SetParameterAttributes" => self.body.push(
                            BodyElement::SetParameterAttributes(SetParameterAttributes::default()),
                        ),
                        "SetParameterValuesResponse" => {
                            self.body.push(BodyElement::SetParameterValuesResponse(
                                SetParameterValuesResponse::default(),
                            ));
                        }
                        "SetParameterValues" => self.body.push(BodyElement::SetParameterValues(
                            SetParameterValues::default(),
                        )),
                        "SetVouchersResponse" => self
                            .body
                            .push(BodyElement::SetVouchersResponse(SetVouchersResponse {})),
                        "SetVouchers" => self
                            .body
                            .push(BodyElement::SetVouchers(SetVouchers::default())),
                        "TransferCompleteResponse" => self.body.push(
                            BodyElement::TransferCompleteResponse(TransferCompleteResponse {}),
                        ),
                        "TransferComplete" => self
                            .body
                            .push(BodyElement::TransferComplete(TransferComplete::default())),
                        "UploadResponse" => self
                            .body
                            .push(BodyElement::UploadResponse(UploadResponse::default())),
                        "Upload" => self.body.push(BodyElement::Upload(Upload::default())),
                        _ => {}
                    }
                }
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::Inform(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::RequestDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(_unhandled) => { // the ones who dont need a start_handler, ie GetParameterValues aso
                    }
                    None => {
                        warn!(
                            "Element found under {}, but state list of bodies is empty",
                            body_element
                        );
                    }
                }
            }
            _ => {
                warn!("Unrecoqnized pattern");
            }
        }
    }

    // TODO: todo!("// match the ones who actually need and end_handler, and call their respective end_handler");
    pub fn end_handler(&mut self, path: &[String], _name: &xml::name::OwnedName) {
        let _path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        {}
    }

    #[allow(clippy::too_many_lines)]
    pub fn characters(&mut self, path: &[String], characters: &str) {
        // println!("Path: {:?} Chars: {}", path, characters);
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope", "Header", header_element] => {
                let last = self.header.last_mut();
                match last {
                    Some(HeaderElement::ID(data)) => {
                        if header_element == &"ID" {
                            data.id = characters.into();
                        }
                    }
                    Some(HeaderElement::NoMoreRequests(data)) => {
                        if header_element == &"NoMoreRequests" {
                            data.value = parse_to_int(characters, 0);
                        }
                    }
                    Some(HeaderElement::HoldRequests(data)) => {
                        if header_element == &"HoldRequests" {
                            data.hold = str2bool(characters);
                        }
                    }
                    Some(HeaderElement::SessionTimeout(data)) => {
                        if header_element == &"SessionTimeout" {
                            data.timeout = parse_to_int(characters, 0);
                        }
                    }
                    Some(HeaderElement::SupportedCWMPVersions(data)) => {
                        if header_element == &"SupportedCWMPVersions" {
                            data.value = characters.into();
                        }
                    }
                    Some(HeaderElement::UseCWMPVersion(data)) => {
                        if header_element == &"UseCWMPVersion" {
                            data.value = characters.to_string();
                        }
                    }
                    _ => {} // should never happen
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AddObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::AddObject(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::AutonomousTransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::CancelTransfer(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DeleteObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DeleteObject(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DownloadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Download(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Fault(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetOptions(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterNames(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::InformResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Inform(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::KickedResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Kicked(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::Reboot(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::RequestDownload(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ScheduleInform(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::TransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::UploadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Upload(e)) => e.characters(&path_pattern[2..], characters),
                    Some(unhandled) => {
                        println!("characters for {unhandled:?} is so far unhandled");
                    }
                    None => {
                        warn!(
                            "Element found under {}, but state list of bodies is empty",
                            body_element
                        );
                    }
                }
            }
            _ => {
                // cant find anywhere to stuff this text, ok...
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for Envelope {
    fn arbitrary(g: &mut Gen) -> Envelope {
        // cwmp version is handled a bit special, because
        // a value of Some("") becomes xmlns:cwmp="" which
        // is unparsable
        let header = Vec::<HeaderElement>::arbitrary(g);
        let body = Vec::<BodyElement>::arbitrary(g);
        let cwmp = Option::<CwmpVersion>::arbitrary(g);

        Envelope::new(cwmp.clone(), header, body)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.cwmp_version.clone(),
                self.header.clone(),
                self.body.clone(),
            )
                .shrink()
                .map(|(c, h, b)| Envelope {
                    cwmp_version: c,
                    header: h,
                    body: b,
                }),
        )
    }
}
