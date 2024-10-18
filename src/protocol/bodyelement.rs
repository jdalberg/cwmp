use super::{
    AddObject, AddObjectResponse, AutonomousDUStateChangeComplete,
    AutonomousDUStateChangeCompleteResponse, AutonomousTransferComplete,
    AutonomousTransferCompleteResponse, CancelTransfer, CancelTransferResponse, ChangeDUState,
    ChangeDUStateResponse, DUStateChangeComplete, DUStateChangeCompleteResponse, DeleteObject,
    DeleteObjectResponse, Download, DownloadResponse, FactoryReset, FactoryResetResponse, Fault,
    GetAllQueuedTransfers, GetAllQueuedTransfersResponse, GetOptions, GetOptionsResponse,
    GetParameterAttributes, GetParameterAttributesResponse, GetParameterNames,
    GetParameterNamesResponse, GetParameterValues, GetParameterValuesResponse, GetQueuedTransfers,
    GetQueuedTransfersResponse, GetRPCMethods, GetRPCMethodsResponse, Inform, InformResponse,
    Kicked, KickedResponse, Reboot, RebootResponse, RequestDownload, RequestDownloadResponse,
    ScheduleDownload, ScheduleDownloadResponse, ScheduleInform, ScheduleInformResponse,
    SetParameterAttributes, SetParameterAttributesResponse, SetParameterValues,
    SetParameterValuesResponse, SetVouchers, SetVouchersResponse, TransferComplete,
    TransferCompleteResponse, Upload, UploadResponse,
};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum BodyElement {
    AddObjectResponse(AddObjectResponse),
    AddObject(AddObject),
    AutonomousDUStateChangeCompleteResponse(AutonomousDUStateChangeCompleteResponse),
    AutonomousDUStateChangeComplete(AutonomousDUStateChangeComplete),
    AutonomousTransferCompleteResponse(AutonomousTransferCompleteResponse),
    AutonomousTransferComplete(AutonomousTransferComplete),
    CancelTransferResponse(CancelTransferResponse),
    CancelTransfer(CancelTransfer),
    ChangeDUStateResponse(ChangeDUStateResponse),
    ChangeDUState(ChangeDUState),
    DeleteObjectResponse(DeleteObjectResponse),
    DeleteObject(DeleteObject),
    DownloadResponse(DownloadResponse),
    Download(Download),
    DUStateChangeCompleteResponse(DUStateChangeCompleteResponse),
    DUStateChangeComplete(DUStateChangeComplete),
    FactoryResetResponse(FactoryResetResponse),
    FactoryReset(FactoryReset),
    Fault(Fault),
    GetAllQueuedTransfersResponse(GetAllQueuedTransfersResponse),
    GetAllQueuedTransfers(GetAllQueuedTransfers),
    GetOptionsResponse(GetOptionsResponse),
    GetOptions(GetOptions),
    GetParameterAttributes(GetParameterAttributes),
    GetParameterAttributesResponse(GetParameterAttributesResponse),
    GetParameterNamesResponse(GetParameterNamesResponse),
    GetParameterNames(GetParameterNames),
    GetParameterValues(GetParameterValues),
    GetParameterValuesResponse(GetParameterValuesResponse),
    GetQueuedTransfersResponse(GetQueuedTransfersResponse),
    GetQueuedTransfers(GetQueuedTransfers),
    GetRPCMethodsResponse(GetRPCMethodsResponse),
    GetRPCMethods(GetRPCMethods),
    InformResponse(InformResponse),
    Inform(Inform),
    KickedResponse(KickedResponse),
    Kicked(Kicked),
    RebootResponse(RebootResponse),
    Reboot(Reboot),
    RequestDownloadResponse(RequestDownloadResponse),
    RequestDownload(RequestDownload),
    ScheduleDownloadResponse(ScheduleDownloadResponse),
    ScheduleDownload(ScheduleDownload),
    ScheduleInformResponse(ScheduleInformResponse),
    ScheduleInform(ScheduleInform),
    SetParameterAttributesResponse(SetParameterAttributesResponse),
    SetParameterAttributes(SetParameterAttributes),
    SetParameterValuesResponse(SetParameterValuesResponse),
    SetParameterValues(SetParameterValues),
    SetVouchersResponse(SetVouchersResponse),
    SetVouchers(SetVouchers),
    TransferCompleteResponse(TransferCompleteResponse),
    TransferComplete(TransferComplete),
    UploadResponse(UploadResponse),
    Upload(Upload),
}

#[cfg(test)]
impl Arbitrary for BodyElement {
    fn arbitrary(g: &mut Gen) -> Self {
        let vals = vec![
            BodyElement::AddObjectResponse(AddObjectResponse::arbitrary(g)),
            BodyElement::AddObject(AddObject::arbitrary(g)),
            BodyElement::AutonomousDUStateChangeCompleteResponse(
                AutonomousDUStateChangeCompleteResponse {},
            ),
            BodyElement::AutonomousDUStateChangeComplete(
                AutonomousDUStateChangeComplete::arbitrary(g),
            ),
            BodyElement::AutonomousTransferCompleteResponse(AutonomousTransferCompleteResponse {}),
            BodyElement::AutonomousTransferComplete(AutonomousTransferComplete::arbitrary(g)),
            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
            BodyElement::CancelTransfer(CancelTransfer::arbitrary(g)),
            BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {}),
            BodyElement::ChangeDUState(ChangeDUState::arbitrary(g)),
            BodyElement::DeleteObjectResponse(DeleteObjectResponse::arbitrary(g)),
            BodyElement::DeleteObject(DeleteObject::arbitrary(g)),
            BodyElement::DownloadResponse(DownloadResponse::arbitrary(g)),
            BodyElement::Download(Download::arbitrary(g)),
            BodyElement::DUStateChangeCompleteResponse(DUStateChangeCompleteResponse {}),
            BodyElement::DUStateChangeComplete(DUStateChangeComplete::arbitrary(g)),
            BodyElement::FactoryResetResponse(FactoryResetResponse {}),
            BodyElement::FactoryReset(FactoryReset {}),
            BodyElement::Fault(Fault::arbitrary(g)),
            BodyElement::GetAllQueuedTransfersResponse(GetAllQueuedTransfersResponse::arbitrary(g)),
            BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {}),
            BodyElement::GetOptionsResponse(GetOptionsResponse::arbitrary(g)),
            BodyElement::GetOptions(GetOptions::arbitrary(g)),
            BodyElement::GetParameterAttributes(GetParameterAttributes::arbitrary(g)),
            BodyElement::GetParameterAttributesResponse(GetParameterAttributesResponse::arbitrary(
                g,
            )),
            BodyElement::GetParameterNamesResponse(GetParameterNamesResponse::arbitrary(g)),
            BodyElement::GetParameterNames(GetParameterNames::arbitrary(g)),
            BodyElement::GetParameterValues(GetParameterValues::arbitrary(g)),
            BodyElement::GetParameterValuesResponse(GetParameterValuesResponse::arbitrary(g)),
            BodyElement::GetQueuedTransfersResponse(GetQueuedTransfersResponse::arbitrary(g)),
            BodyElement::GetQueuedTransfers(GetQueuedTransfers {}),
            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::arbitrary(g)),
            BodyElement::GetRPCMethods(GetRPCMethods {}),
            BodyElement::InformResponse(InformResponse::arbitrary(g)),
            BodyElement::Inform(Inform::arbitrary(g)),
            BodyElement::KickedResponse(KickedResponse::arbitrary(g)),
            BodyElement::Kicked(Kicked::arbitrary(g)),
            BodyElement::RebootResponse(RebootResponse {}),
            BodyElement::Reboot(Reboot::arbitrary(g)),
            BodyElement::RequestDownloadResponse(RequestDownloadResponse {}),
            BodyElement::RequestDownload(RequestDownload::arbitrary(g)),
            BodyElement::ScheduleDownloadResponse(ScheduleDownloadResponse {}),
            BodyElement::ScheduleDownload(ScheduleDownload::arbitrary(g)),
            BodyElement::ScheduleInformResponse(ScheduleInformResponse {}),
            BodyElement::ScheduleInform(ScheduleInform::arbitrary(g)),
            BodyElement::SetParameterAttributesResponse(SetParameterAttributesResponse {}),
            BodyElement::SetParameterAttributes(SetParameterAttributes::arbitrary(g)),
            BodyElement::SetParameterValuesResponse(SetParameterValuesResponse::arbitrary(g)),
            BodyElement::SetParameterValues(SetParameterValues::arbitrary(g)),
            BodyElement::SetVouchersResponse(SetVouchersResponse {}),
            BodyElement::SetVouchers(SetVouchers::arbitrary(g)),
            BodyElement::TransferCompleteResponse(TransferCompleteResponse {}),
            BodyElement::TransferComplete(TransferComplete::arbitrary(g)),
            BodyElement::UploadResponse(UploadResponse::arbitrary(g)),
            BodyElement::Upload(Upload::arbitrary(g)),
        ];
        let mut rng = rand::thread_rng();
        let idxs = std::ops::Range {
            start: 0,
            end: vals.len() - 1,
        };
        let random_index: usize = rng.gen_range(idxs);
        match vals.get(random_index) {
            Some(v) => v.clone(),
            None => BodyElement::AddObjectResponse(AddObjectResponse::arbitrary(g)),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            &BodyElement::AddObjectResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::AddObjectResponse(s)))
            }
            &BodyElement::AddObject(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::AddObject(s)))
            }
            &BodyElement::AutonomousDUStateChangeCompleteResponse(_) => {
                quickcheck::empty_shrinker()
            }
            &BodyElement::AutonomousDUStateChangeComplete(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::AutonomousDUStateChangeComplete(s)),
            ),
            &BodyElement::AutonomousTransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::AutonomousTransferComplete(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::AutonomousTransferComplete(s)),
            ),
            &BodyElement::CancelTransferResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::CancelTransfer(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::CancelTransfer(s)))
            }
            &BodyElement::ChangeDUStateResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ChangeDUState(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ChangeDUState(s)))
            }
            &BodyElement::DeleteObjectResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DeleteObjectResponse(s)))
            }
            &BodyElement::DeleteObject(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DeleteObject(s)))
            }
            &BodyElement::DownloadResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DownloadResponse(s)))
            }
            &BodyElement::Download(ref x) => Box::new(x.shrink().map(|s| BodyElement::Download(s))),
            &BodyElement::DUStateChangeCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::DUStateChangeComplete(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DUStateChangeComplete(s)))
            }
            &BodyElement::FactoryResetResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::FactoryReset(_) => quickcheck::empty_shrinker(),
            &BodyElement::Fault(ref x) => Box::new(x.shrink().map(|s| BodyElement::Fault(s))),
            &BodyElement::GetAllQueuedTransfersResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetAllQueuedTransfersResponse(s)),
            ),
            &BodyElement::GetAllQueuedTransfers(_) => quickcheck::empty_shrinker(),
            &BodyElement::GetOptionsResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetOptionsResponse(s)))
            }
            &BodyElement::GetOptions(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetOptions(s)))
            }
            &BodyElement::GetParameterAttributesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterAttributesResponse(s)),
            ),
            &BodyElement::GetParameterAttributes(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterAttributes(s)))
            }
            &BodyElement::GetParameterNamesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterNamesResponse(s)),
            ),
            &BodyElement::GetParameterNames(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterNames(s)))
            }
            &BodyElement::GetParameterValuesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterValuesResponse(s)),
            ),
            &BodyElement::GetParameterValues(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterValues(s)))
            }
            &BodyElement::GetQueuedTransfersResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetQueuedTransfersResponse(s)),
            ),
            &BodyElement::GetQueuedTransfers(_) => quickcheck::empty_shrinker(),
            &BodyElement::GetRPCMethodsResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetRPCMethodsResponse(s)))
            }
            &BodyElement::GetRPCMethods(_) => quickcheck::empty_shrinker(),
            &BodyElement::InformResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::InformResponse(s)))
            }
            &BodyElement::Inform(ref x) => Box::new(x.shrink().map(|s| BodyElement::Inform(s))),
            &BodyElement::KickedResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::KickedResponse(s)))
            }
            &BodyElement::Kicked(ref x) => Box::new(x.shrink().map(|s| BodyElement::Kicked(s))),
            &BodyElement::RebootResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::Reboot(ref x) => Box::new(x.shrink().map(|s| BodyElement::Reboot(s))),
            &BodyElement::RequestDownloadResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::RequestDownload(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::RequestDownload(s)))
            }
            &BodyElement::ScheduleDownloadResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ScheduleDownload(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ScheduleDownload(s)))
            }
            &BodyElement::ScheduleInformResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ScheduleInform(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ScheduleInform(s)))
            }
            &BodyElement::SetParameterAttributesResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::SetParameterAttributes(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetParameterAttributes(s)))
            }
            &BodyElement::SetParameterValuesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::SetParameterValuesResponse(s)),
            ),
            &BodyElement::SetParameterValues(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetParameterValues(s)))
            }
            &BodyElement::SetVouchersResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::SetVouchers(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetVouchers(s)))
            }
            &BodyElement::TransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::TransferComplete(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::TransferComplete(s)))
            }
            &BodyElement::UploadResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::UploadResponse(s)))
            }
            &BodyElement::Upload(ref x) => Box::new(x.shrink().map(|s| BodyElement::Upload(s))),
        }
    }
}
