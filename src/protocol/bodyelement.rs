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
        let mut rng = rand::rng();
        let idxs = std::ops::Range {
            start: 0,
            end: vals.len() - 1,
        };
        let random_index: usize = rng.random_range(idxs);
        match vals.get(random_index) {
            Some(v) => v.clone(),
            None => BodyElement::AddObjectResponse(AddObjectResponse::arbitrary(g)),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            BodyElement::AddObjectResponse(x) => {
                Box::new(x.shrink().map(BodyElement::AddObjectResponse))
            }
            BodyElement::AddObject(x) => Box::new(x.shrink().map(BodyElement::AddObject)),
            &BodyElement::AutonomousDUStateChangeCompleteResponse(_) => {
                quickcheck::empty_shrinker()
            }
            BodyElement::AutonomousDUStateChangeComplete(x) => {
                Box::new(x.shrink().map(BodyElement::AutonomousDUStateChangeComplete))
            }
            &BodyElement::AutonomousTransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::AutonomousTransferComplete(x) => {
                Box::new(x.shrink().map(BodyElement::AutonomousTransferComplete))
            }
            &BodyElement::CancelTransferResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::CancelTransfer(x) => Box::new(x.shrink().map(BodyElement::CancelTransfer)),
            &BodyElement::ChangeDUStateResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::ChangeDUState(x) => Box::new(x.shrink().map(BodyElement::ChangeDUState)),
            BodyElement::DeleteObjectResponse(x) => {
                Box::new(x.shrink().map(BodyElement::DeleteObjectResponse))
            }
            BodyElement::DeleteObject(x) => Box::new(x.shrink().map(BodyElement::DeleteObject)),
            BodyElement::DownloadResponse(x) => {
                Box::new(x.shrink().map(BodyElement::DownloadResponse))
            }
            BodyElement::Download(x) => Box::new(x.shrink().map(BodyElement::Download)),
            &BodyElement::DUStateChangeCompleteResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::DUStateChangeComplete(x) => {
                Box::new(x.shrink().map(BodyElement::DUStateChangeComplete))
            }
            &BodyElement::FactoryResetResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::FactoryReset(_) => quickcheck::empty_shrinker(),
            BodyElement::Fault(x) => Box::new(x.shrink().map(BodyElement::Fault)),
            BodyElement::GetAllQueuedTransfersResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetAllQueuedTransfersResponse))
            }
            &BodyElement::GetAllQueuedTransfers(_) => quickcheck::empty_shrinker(),
            BodyElement::GetOptionsResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetOptionsResponse))
            }
            BodyElement::GetOptions(x) => Box::new(x.shrink().map(BodyElement::GetOptions)),
            BodyElement::GetParameterAttributesResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterAttributesResponse))
            }
            BodyElement::GetParameterAttributes(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterAttributes))
            }
            BodyElement::GetParameterNamesResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterNamesResponse))
            }
            BodyElement::GetParameterNames(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterNames))
            }
            BodyElement::GetParameterValuesResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterValuesResponse))
            }
            BodyElement::GetParameterValues(x) => {
                Box::new(x.shrink().map(BodyElement::GetParameterValues))
            }
            BodyElement::GetQueuedTransfersResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetQueuedTransfersResponse))
            }
            &BodyElement::GetQueuedTransfers(_) => quickcheck::empty_shrinker(),
            BodyElement::GetRPCMethodsResponse(x) => {
                Box::new(x.shrink().map(BodyElement::GetRPCMethodsResponse))
            }
            &BodyElement::GetRPCMethods(_) => quickcheck::empty_shrinker(),
            BodyElement::InformResponse(x) => Box::new(x.shrink().map(BodyElement::InformResponse)),
            BodyElement::Inform(x) => Box::new(x.shrink().map(BodyElement::Inform)),
            BodyElement::KickedResponse(x) => Box::new(x.shrink().map(BodyElement::KickedResponse)),
            BodyElement::Kicked(x) => Box::new(x.shrink().map(BodyElement::Kicked)),
            &BodyElement::RebootResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::Reboot(x) => Box::new(x.shrink().map(BodyElement::Reboot)),
            &BodyElement::RequestDownloadResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::RequestDownload(x) => {
                Box::new(x.shrink().map(BodyElement::RequestDownload))
            }
            &BodyElement::ScheduleDownloadResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::ScheduleDownload(x) => {
                Box::new(x.shrink().map(BodyElement::ScheduleDownload))
            }
            &BodyElement::ScheduleInformResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::ScheduleInform(x) => Box::new(x.shrink().map(BodyElement::ScheduleInform)),
            &BodyElement::SetParameterAttributesResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::SetParameterAttributes(x) => {
                Box::new(x.shrink().map(BodyElement::SetParameterAttributes))
            }
            BodyElement::SetParameterValuesResponse(x) => {
                Box::new(x.shrink().map(BodyElement::SetParameterValuesResponse))
            }
            BodyElement::SetParameterValues(x) => {
                Box::new(x.shrink().map(BodyElement::SetParameterValues))
            }
            &BodyElement::SetVouchersResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::SetVouchers(x) => Box::new(x.shrink().map(BodyElement::SetVouchers)),
            &BodyElement::TransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            BodyElement::TransferComplete(x) => {
                Box::new(x.shrink().map(BodyElement::TransferComplete))
            }
            BodyElement::UploadResponse(x) => Box::new(x.shrink().map(BodyElement::UploadResponse)),
            BodyElement::Upload(x) => Box::new(x.shrink().map(BodyElement::Upload)),
        }
    }
}
