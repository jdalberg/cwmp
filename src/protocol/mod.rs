use chrono::prelude::TimeZone;
use chrono::{DateTime, Utc};
use core::fmt::Debug;
use log::warn;

#[derive(Debug, PartialEq)]
pub struct ID {
    pub must_understand: bool,
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct HoldRequests {
    pub mustunderstand: bool,
    pub hold: bool,
}

#[derive(Debug, PartialEq)]
pub struct SessionTimeout {
    pub mustunderstand: bool,
    pub timeout: u32,
}

#[derive(Debug, PartialEq)]
pub enum HeaderElement {
    ID(ID),
    HoldRequests(HoldRequests),
    SessionTimeout(SessionTimeout),
}

#[derive(Debug, PartialEq)]
pub struct FaultStruct {
    code: u32,
    string: String,
}

impl FaultStruct {
    pub fn new(code: u32, string: &str) -> Self {
        FaultStruct {
            code: code,
            string: String::from(string),
        }
    }
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }
    pub fn set_string(&mut self, string: &str) {
        self.string = string.to_string();
    }
}

#[derive(Debug, PartialEq)]
pub struct AddObjectResponse {
    instance_number: u32,
    status: String,
}

impl AddObjectResponse {
    pub fn new(instance_number: u32, status: &str) -> Self {
        AddObjectResponse {
            instance_number: instance_number,
            status: status.to_string(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObjectResponse", "InstanceNumber"] => {
                self.instance_number = characters.parse().unwrap();
            }
            ["AddObjectResponse", "Status"] => {
                self.status = characters.to_string();
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddObject {
    object_name: String,
    parameter_key: String,
}

impl AddObject {
    pub fn new(object_name: &str, parameter_key: &str) -> Self {
        AddObject {
            object_name: object_name.to_string(),
            parameter_key: parameter_key.to_string(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObject", "ObjectName"] => {
                self.object_name = characters.to_string();
            }
            ["AddObject", "ParameterKey"] => {
                self.parameter_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AutonomousDUStateChangeCompleteResponse;

#[derive(Debug, PartialEq)]
pub struct AutoOpResult {
    uuid: String,
    deployment_unit_ref: String,
    version: String,
    current_state: String,
    resolved: String,
    execution_unit_ref_list: String,
    start_time: DateTime<Utc>,
    complete_time: DateTime<Utc>,
    fault: FaultStruct,
    operation_performed: String,
}

impl AutoOpResult {
    pub fn new(
        uuid: &str,
        deployment_unit_ref: &str,
        version: &str,
        current_state: &str,
        resolved: &str,
        execution_unit_ref_list: &str,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault_code: u32,
        fault_string: &str,
        operation_performed: &str,
    ) -> Self {
        AutoOpResult {
            uuid: uuid.to_string(),
            deployment_unit_ref: deployment_unit_ref.to_string(),
            version: version.to_string(),
            current_state: current_state.to_string(),
            resolved: resolved.to_string(),
            execution_unit_ref_list: execution_unit_ref_list.to_string(),
            start_time: start_time,
            complete_time: complete_time,
            fault: FaultStruct::new(fault_code, fault_string),
            operation_performed: operation_performed.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AutonomousDUStateChangeComplete {
    results: Vec<AutoOpResult>,
}

impl AutonomousDUStateChangeComplete {
    pub fn new(results: Vec<AutoOpResult>) -> Self {
        AutonomousDUStateChangeComplete { results: results }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "UUID" => e.uuid = characters.to_string(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        "CurrentState" => e.current_state = characters.to_string(),
                        "Resolved" => e.resolved = characters.to_string(),
                        "ExecutionUnitRefList" => {
                            e.execution_unit_ref_list = characters.to_string()
                        }
                        "StartTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.start_time = dt,
                            _ => {}
                        },
                        "CompleteTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.complete_time = dt,
                            _ => {}
                        },
                        "OperationPerformed" => e.operation_performed = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", "Fault", "FaultStruct", key] =>
            {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "FaultCode" => match characters.parse::<u32>() {
                            Ok(parsed) => e.fault.set_code(parsed),
                            _ => {}
                        },
                        "FaultString" => e.fault.set_string(&characters[..]),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AutonomousTransferCompleteResponse;

#[derive(Debug, PartialEq)]
pub struct AutonomousTransferComplete {
    announce_url: String,
    transfer_url: String,
    is_download: u8,
    file_type: String,
    file_size: u32,
    target_filename: String,
    fault: FaultStruct,
    start_time: DateTime<Utc>,
    complete_time: DateTime<Utc>,
}

impl AutonomousTransferComplete {
    pub fn new(
        announce_url: &str,
        transfer_url: &str,
        is_download: u8,
        file_type: &str,
        file_size: u32,
        target_filename: &str,
        fault: FaultStruct,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
    ) -> Self {
        AutonomousTransferComplete {
            announce_url: announce_url.to_string(),
            transfer_url: transfer_url.to_string(),
            is_download: is_download,
            file_type: file_type.to_string(),
            file_size: file_size,
            target_filename: target_filename.to_string(),
            fault: fault,
            start_time: start_time,
            complete_time: complete_time,
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AutonomousTransferComplete", "AnnounceURL"] => {
                self.announce_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "TransferURL"] => {
                self.transfer_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "IsDownload"] => {
                self.is_download = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "TargetFileName"] => {
                self.target_filename = characters.to_string()
            }
            ["AutonomousTransferComplete", "FileType"] => self.file_type = characters.to_string(),
            ["AutonomousTransferComplete", "FileSize"] => {
                self.file_size = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "StartTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.start_time = dt,
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "CompleteTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.complete_time = dt,
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0))
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultString"] => {
                self.fault.set_string(characters)
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CancelTransferResponse;

#[derive(Debug, PartialEq)]
pub struct CancelTransfer {
    command_key: String,
}

impl CancelTransfer {
    pub fn new(command_key: &str) -> Self {
        CancelTransfer {
            command_key: command_key.to_string(),
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["CancelTransfer", "CommandKey"] => self.command_key = characters.to_string(),

            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ChangeDUStateResponse;

#[derive(Debug, PartialEq)]
pub struct InstallOp {
    url: String,
    uuid: String,
    username: String,
    password: String,
    execution_env_ref: String,
}

impl InstallOp {
    pub fn new(
        url: &str,
        uuid: &str,
        username: &str,
        password: &str,
        execution_env_ref: &str,
    ) -> Self {
        InstallOp {
            url: url.to_string(),
            uuid: uuid.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            execution_env_ref: execution_env_ref.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UninstallOp {
    url: String,
    uuid: String,
    execution_env_ref: String,
}

impl UninstallOp {
    pub fn new(url: &str, uuid: &str, execution_env_ref: &str) -> Self {
        UninstallOp {
            url: url.to_string(),
            uuid: uuid.to_string(),
            execution_env_ref: execution_env_ref.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UpdateOp {
    url: String,
    uuid: String,
    username: String,
    password: String,
    version: String,
}
impl UpdateOp {
    pub fn new(url: &str, uuid: &str, username: &str, password: &str, version: &str) -> Self {
        UpdateOp {
            url: url.to_string(),
            uuid: uuid.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            version: version.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ChangeDUState {
    command_key: String,
    install_operations: Vec<InstallOp>,
    uninstall_operations: Vec<UninstallOp>,
    update_operations: Vec<UpdateOp>,
}

impl ChangeDUState {
    pub fn new(
        command_key: &str,
        install_operations: Vec<InstallOp>,
        uninstall_operations: Vec<UninstallOp>,
        update_operations: Vec<UpdateOp>,
    ) -> Self {
        ChangeDUState {
            command_key: command_key.to_string(),
            install_operations: install_operations,
            uninstall_operations: uninstall_operations,
            update_operations: update_operations,
        }
    }

    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["ChangeDUState", "Operations", "InstallOpStruct"] => self
                .install_operations
                .push(InstallOp::new("", "", "", "", "")),
            ["ChangeDUState", "Operations", "UninstallOpStruct"] => {
                self.uninstall_operations.push(UninstallOp::new("", "", ""))
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct"] => self
                .update_operations
                .push(UpdateOp::new("", "", "", "", "")),
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ChangeDUState", "CommandKey"] => self.command_key = characters.to_string(),
            ["ChangeDUState", "Operations", "InstallOpStruct", key] => {
                let last = self.install_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "Username" => e.username = characters.to_string(),
                        "Password" => e.password = characters.to_string(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct", key] => {
                let last = self.uninstall_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct", key] => {
                let last = self.update_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "Username" => e.username = characters.to_string(),
                        "Password" => e.password = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteObjectResponse {
    status: String,
}

impl DeleteObjectResponse {
    pub fn new(status: &str) -> Self {
        DeleteObjectResponse {
            status: status.to_string(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DeleteObjectResponse", "Status"] => self.status = characters.to_string(),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteObject {
    object_name: String,
    parameter_key: String,
}

impl DeleteObject {
    pub fn new(object_name: &str, parameter_key: &str) -> Self {
        DeleteObject {
            object_name: object_name.to_string(),
            parameter_key: parameter_key.to_string(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DeleteObject", "ObjectName"] => {
                self.object_name = characters.to_string();
            }
            ["DeleteObject", "ParameterKey"] => {
                self.parameter_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DownloadResponse {
    status: String,
    start_time: DateTime<Utc>,
    complete_time: DateTime<Utc>,
}

impl DownloadResponse {
    pub fn new(status: &str, start_time: DateTime<Utc>, complete_time: DateTime<Utc>) -> Self {
        DownloadResponse {
            status: status.to_string(),
            start_time: start_time,
            complete_time: complete_time,
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DownloadResponse", "Status"] => {
                self.status = characters.to_string();
            }
            ["DownloadResponse", "StartTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.start_time = dt,
                _ => {}
            },
            ["DownloadResponse", "CompleteTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.complete_time = dt,
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Download {
    command_key: String,
    file_type: String,
    url: String,
    username: String,
    password: String,
    file_size: u32,
    target_filename: String,
    delay_seconds: u32,
    success_url: String,
    failure_url: String,
}

impl Download {
    pub fn new(
        command_key: &str,
        file_type: &str,
        url: &str,
        username: &str,
        password: &str,
        file_size: u32,
        target_filename: &str,
        delay_seconds: u32,
        success_url: &str,
        failure_url: &str,
    ) -> Self {
        Download {
            command_key: command_key.to_string(),
            file_type: file_type.to_string(),
            url: url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            file_size: file_size,
            target_filename: target_filename.to_string(),
            delay_seconds: delay_seconds,
            success_url: success_url.to_string(),
            failure_url: failure_url.to_string(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Download", "CommandKey"] => self.command_key = characters.to_string(),
            ["Download", "FileType"] => self.file_type = characters.to_string(),
            ["Download", "URL"] => self.url = characters.to_string(),
            ["Download", "Username"] => self.username = characters.to_string(),
            ["Download", "Password"] => self.password = characters.to_string(),
            ["Download", "FileSize"] => self.file_size = parse_to_int(characters, 0),
            ["Download", "TargetFileName"] => self.target_filename = characters.to_string(),
            ["Download", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            ["Download", "SuccessURL"] => self.success_url = characters.to_string(),
            ["Download", "FailureURL"] => self.failure_url = characters.to_string(),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OpResult {
    uuid: String,
    deployment_unit_ref: String,
    version: String,
    current_state: String,
    resolved: u32,
    execution_unit_ref_list: String,
    start_time: DateTime<Utc>,
    complete_time: DateTime<Utc>,
    fault: FaultStruct,
}

impl OpResult {
    pub fn new(
        uuid: &str,
        deployment_unit_ref: &str,
        version: &str,
        current_state: &str,
        resolved: u32,
        execution_unit_ref_list: &str,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault: FaultStruct,
    ) -> Self {
        OpResult {
            uuid: uuid.to_string(),
            deployment_unit_ref: deployment_unit_ref.to_string(),
            version: version.to_string(),
            current_state: current_state.to_string(),
            resolved: resolved,
            execution_unit_ref_list: execution_unit_ref_list.to_string(),
            start_time: start_time,
            complete_time: complete_time,
            fault: fault,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DUStateChangeCompleteResponse;

#[derive(Debug, PartialEq)]
pub struct DUStateChangeComplete {
    command_key: String,
    results: Vec<OpResult>,
}

impl DUStateChangeComplete {
    pub fn new(command_key: &str, results: Vec<OpResult>) -> Self {
        DUStateChangeComplete {
            command_key: command_key.to_string(),
            results: results,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["DUStateChangeComplete", "Results"] => self.results.push(OpResult::new(
                "",
                "",
                "",
                "",
                0,
                "",
                Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                FaultStruct::new(0, ""),
            )),
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DUStateChangeComplete", "CommandKey"] => self.command_key = characters.to_string(),
            ["DUStateChangeComplete", "Results", "OpResultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "UUID" => e.uuid = characters.to_string(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        "CurrentState" => e.current_state = characters.to_string(),
                        "Resolved" => e.resolved = parse_to_int(characters, 0),
                        "ExecutionUnitRefList" => {
                            e.execution_unit_ref_list = characters.to_string()
                        }
                        "StartTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.start_time = dt,
                            _ => {}
                        },
                        "CompleteTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.complete_time = dt,
                            _ => {}
                        },
                        _ => {}
                    },
                    None => {}
                }
            }
            ["DUStateChangeComplete", "Results", "OpResultStruct", "Fault", "FaultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "FaultCode" => e.fault.set_code(parse_to_int(characters, 0)),
                        "FaultString" => e.fault.set_string(characters),
                        _ => {}
                    },
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FactoryResetResponse;

#[derive(Debug, PartialEq)]
pub struct FactoryReset;

#[derive(Debug, PartialEq)]
struct FaultDetail {
    code: u32,
    string: String,
}

#[derive(Debug, PartialEq)]
pub struct Fault {
    faultcode: String,
    faultstring: String,
    detail: FaultDetail,
}

impl Fault {
    pub fn new(faultcode: &str, faultstring: &str, code: u32, string: &str) -> Self {
        Fault {
            faultcode: faultcode.to_string(),
            faultstring: faultstring.to_string(),
            detail: FaultDetail {
                code: code,
                string: string.to_string(),
            },
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Fault", "faultcode"] => {
                self.faultcode = characters.to_string();
            }
            ["Fault", "faultstring"] => {
                self.faultstring = characters.to_string();
            }
            ["Fault", "detail", "Fault", "FaultCode"] => {
                self.detail.code = parse_to_int(characters, 0);
            }
            ["Fault", "detail", "Fault", "FaultString"] => {
                self.detail.string = characters.to_string();
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AllQueuedTransfers {
    command_key: String,
    state: String,
    is_download: u8,
    file_type: String,
    file_size: u32,
    target_filename: String,
}

impl AllQueuedTransfers {
    pub fn new(
        command_key: &str,
        state: &str,
        is_download: u8,
        file_type: &str,
        file_size: u32,
        target_filename: &str,
    ) -> Self {
        AllQueuedTransfers {
            command_key: command_key.to_string(),
            state: state.to_string(),
            is_download: is_download,
            file_type: file_type.to_string(),
            file_size: file_size,
            target_filename: target_filename.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetAllQueuedTransfersResponse {
    transfer_list: Vec<AllQueuedTransfers>,
}

impl GetAllQueuedTransfersResponse {
    pub fn new(transfer_list: Vec<AllQueuedTransfers>) -> Self {
        GetAllQueuedTransfersResponse {
            transfer_list: transfer_list,
        }
    }

    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct"] => self
                .transfer_list
                .push(AllQueuedTransfers::new("", "", 0, "", 0, "")),
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct", key] => {
                match self.transfer_list.last_mut() {
                    Some(last) => match key {
                        "CommandKey" => last.command_key = characters.to_string(),
                        "State" => last.state = characters.to_string(),
                        "IsDownload" => last.is_download = parse_to_int(characters, 0),
                        "FileType" => last.file_type = characters.to_string(),
                        "FileSize" => last.file_size = parse_to_int(characters, 0),
                        "TargetFileName" => last.target_filename = characters.to_string(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetAllQueuedTransfers;

#[derive(Debug, PartialEq)]
pub struct OptionStruct {
    option_name: String,
    voucher_sn: String,
    state: u8,
    mode: String,
    start_date: DateTime<Utc>,
    expiration_date: DateTime<Utc>,
    is_transferable: u8,
}

impl OptionStruct {
    pub fn new(
        option_name: &str,
        voucher_sn: &str,
        state: u8,
        mode: &str,
        start_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        is_transferable: u8,
    ) -> Self {
        OptionStruct {
            option_name: option_name.to_string(),
            voucher_sn: voucher_sn.to_string(),
            state: state,
            mode: mode.to_string(),
            start_date: start_date,
            expiration_date: expiration_date,
            is_transferable: is_transferable,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetOptionsResponse {
    option_list: Vec<OptionStruct>,
}

impl GetOptionsResponse {
    pub fn new(option_list: Vec<OptionStruct>) -> Self {
        GetOptionsResponse {
            option_list: option_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetOptionsResponse", "OptionList", "OptionStruct"] => {
                self.option_list.push(OptionStruct::new(
                    "",
                    "",
                    0,
                    "",
                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                    0,
                ))
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetOptionsResponse", "OptionList", "OptionStruct", key] => {
                match self.option_list.last_mut() {
                    Some(last) => match key {
                        "OptionName" => last.option_name = characters.to_string(),
                        "VoucherSN" => last.voucher_sn = characters.to_string(),
                        "State" => last.state = parse_to_int(characters, 0),
                        "Mode" => last.mode = characters.to_string(),
                        "StartDate" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => last.start_date = dt,
                            _ => {}
                        },
                        "ExpirationDate" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => last.expiration_date = dt,
                            _ => {}
                        },
                        "IsTransferable" => last.is_transferable = parse_to_int(characters, 0),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetOptions {
    option_name: Option<String>,
}

impl GetOptions {
    pub fn new(option_name: &str) -> Self {
        GetOptions {
            option_name: Some(option_name.to_string()),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetOptions", "OptionName"] => self.option_name = Some(characters.to_string()),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetParameterAttributes {
    pub parameternames: Vec<String>,
}

impl GetParameterAttributes {
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterAttributes", "ParameterNames", "string"] => {
                self.parameternames.push(characters.to_string())
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParameterAttribute {
    name: String,
    notification: String,
    accesslist: Vec<String>,
}
impl ParameterAttribute {
    pub fn new(name: &str, notification: &str, accesslist: Vec<&str>) -> Self {
        ParameterAttribute {
            name: name.to_string(),
            notification: notification.to_string(),
            accesslist: accesslist.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetParameterAttributesResponse {
    parameters: Vec<ParameterAttribute>,
}

impl GetParameterAttributesResponse {
    pub fn new(parameters: Vec<ParameterAttribute>) -> Self {
        GetParameterAttributesResponse {
            parameters: parameters,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct"] => self
                .parameters
                .push(ParameterAttribute::new("", "", vec![])),
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "Name"] =>
            {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.name = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "Notification"] =>
            {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.notification = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "AccessList", "string"] =>
            {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.accesslist.push(characters.to_string()),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ParameterInfoStruct {
    name: String,
    writable: u8,
}

impl ParameterInfoStruct {
    pub fn new(name: &str, writable: u8) -> Self {
        ParameterInfoStruct {
            name: name.to_string(),
            writable: writable,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetParameterNamesResponse {
    parameter_list: Vec<ParameterInfoStruct>,
}

impl GetParameterNamesResponse {
    pub fn new(parameter_list: Vec<ParameterInfoStruct>) -> Self {
        GetParameterNamesResponse {
            parameter_list: parameter_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct"] => {
                self.parameter_list.push(ParameterInfoStruct::default())
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Name"] => {
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.name = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Writable"] => {
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.writable = parse_to_int(characters, 0),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetParameterNames {
    parameter_path: String,
    next_level: u32,
}
impl GetParameterNames {
    pub fn new(parameter_path: &str, next_level: u32) -> Self {
        GetParameterNames {
            parameter_path: parameter_path.to_string(),
            next_level: next_level,
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNames", "ParameterPath"] => self.parameter_path = characters.to_string(),
            ["GetParameterNames", "NextLevel"] => self.next_level = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParameterValue {
    name: String,
    r#type: String,
    value: String,
}

impl ParameterValue {
    pub fn new(name: &str, param_type: &str, value: &str) -> Self {
        ParameterValue {
            name: name.to_string(),
            r#type: param_type.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetParameterValues {
    parameternames: Vec<String>,
}

impl GetParameterValues {
    pub fn new(parameternames: Vec<String>) -> Self {
        GetParameterValues {
            parameternames: parameternames,
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterValues", "ParameterNames", "string"] => {
                self.parameternames.push(characters.to_string());
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GetParameterValuesResponse {
    parameters: Vec<ParameterValue>,
}

impl GetParameterValuesResponse {
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        GetParameterValuesResponse {
            parameters: parameters,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct"] => {
                self.parameters.push(ParameterValue::new("", "", ""))
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.r#type = extract_attribute(attributes, "type"),
                    None => {}
                }
            }
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Name"] => {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.name = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.value = characters.to_string(),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct QueuedTransferStruct {
    command_key: Option<String>,
    state: Option<String>,
}

impl QueuedTransferStruct {
    pub fn new(command_key: &str, state: &str) -> Self {
        QueuedTransferStruct {
            command_key: Some(command_key.to_string()),
            state: Some(state.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetQueuedTransfersResponse {
    transfer_list: Vec<QueuedTransferStruct>,
}

impl GetQueuedTransfersResponse {
    pub fn new(transfer_list: Vec<QueuedTransferStruct>) -> Self {
        GetQueuedTransfersResponse {
            transfer_list: transfer_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct"] => {
                self.transfer_list.push(QueuedTransferStruct::default())
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", key] => {
                let last = self.transfer_list.last_mut();
                match last {
                    Some(e) => match key {
                        "CommandKey" => e.command_key = Some(characters.to_string()),
                        "State" => e.state = Some(characters.to_string()),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetQueuedTransfers {}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetRPCMethodsResponse {
    method_list: Vec<String>,
}

impl GetRPCMethodsResponse {
    pub fn new(method_list: Vec<&str>) -> Self {
        GetRPCMethodsResponse {
            method_list: method_list.iter().map(|s| s.to_string()).collect(),
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetRPCMethodsResponse", "MethodList", "string"] => {
                self.method_list.push(characters.to_string());
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct GetRPCMethods {}

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq, Default)]
pub struct Envelope {
    pub cwmp: Option<String>,
    pub header: Vec<HeaderElement>,
    pub body: Vec<BodyElement>,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            cwmp: None,
            header: vec![],
            body: vec![],
        }
    }
    fn start_handler(
        &mut self,
        path: &Vec<String>,
        name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
        namespace: &xml::namespace::Namespace,
    ) {
        // match out all the elements in path. If the path goes into body,
        // call the start_handler for each element in the Body vector
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope"] => {
                // find the cwmp attribute, and parse it
                let cwmp_filter = attributes
                    .iter()
                    .filter(|&x| x.name.local_name == "cwmp")
                    .next();
                if cwmp_filter.is_some() {
                    self.cwmp = Some(cwmp_filter.unwrap().value.to_string());
                } else {
                    // search through the namespaces to find a cwmp value
                    match namespace.get("cwmp") {
                        Some(ns) => self.cwmp = Some(ns.to_string()),
                        None => self.cwmp = None,
                    }
                }
            }
            ["Envelope", "Header", header_element] => {
                // check if there is a mustUnderstand attribute, and if so, check
                // if we actually understand the header_element given
                let must_understand_filter = attributes
                    .iter()
                    .filter(|&x| x.name.local_name == "mustUnderstand")
                    .next();

                let must_understand: bool = must_understand_filter.is_some();
                match *header_element {
                    "ID" => self.header.push(HeaderElement::ID(ID {
                        must_understand: must_understand,
                        id: String::from(""),
                    })),
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
                        "AddObject" => self.body.push(BodyElement::AddObject(AddObject {
                            object_name: String::from(""),
                            parameter_key: String::from(""),
                        })),
                        "AddObjectResponse" => {
                            self.body
                                .push(BodyElement::AddObjectResponse(AddObjectResponse {
                                    instance_number: 0,
                                    status: String::from("0"),
                                }))
                        }
                        "AutonomousDUStateChangeCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousDUStateChangeCompleteResponse(
                                    AutonomousDUStateChangeCompleteResponse {},
                                ))
                        }
                        "AutonomousDUStateChangeComplete" => {
                            self.body.push(BodyElement::AutonomousDUStateChangeComplete(
                                AutonomousDUStateChangeComplete::new(vec![AutoOpResult::new(
                                    "",
                                    "",
                                    "",
                                    "",
                                    "",
                                    "",
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    0,
                                    "",
                                    "",
                                )]),
                            ))
                        }
                        "AutonomousTransferCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousTransferCompleteResponse(
                                    AutonomousTransferCompleteResponse {},
                                ))
                        }
                        "AutonomousTransferComplete" => {
                            self.body.push(BodyElement::AutonomousTransferComplete(
                                AutonomousTransferComplete::new(
                                    "",
                                    "",
                                    0,
                                    "",
                                    0,
                                    "",
                                    FaultStruct::new(0, ""),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                ),
                            ))
                        }
                        "CancelTransferResponse" => self.body.push(
                            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
                        ),
                        "CancelTransfer" => self
                            .body
                            .push(BodyElement::CancelTransfer(CancelTransfer::new(""))),
                        "ChangeDUStateResponse" => self
                            .body
                            .push(BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {})),
                        "ChangeDUState" => {
                            self.body
                                .push(BodyElement::ChangeDUState(ChangeDUState::new(
                                    "",
                                    vec![],
                                    vec![],
                                    vec![],
                                )))
                        }
                        "DeleteObjectResponse" => self.body.push(
                            BodyElement::DeleteObjectResponse(DeleteObjectResponse::new("")),
                        ),
                        "DeleteObject" => self
                            .body
                            .push(BodyElement::DeleteObject(DeleteObject::new("", ""))),
                        "DownloadResponse" => {
                            self.body
                                .push(BodyElement::DownloadResponse(DownloadResponse::new(
                                    "",
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                )))
                        }
                        "Download" => self.body.push(BodyElement::Download(Download::new(
                            "", "", "", "", "", 0, "", 0, "", "",
                        ))),
                        "DUStateChangeCompleteResponse" => {
                            self.body.push(BodyElement::DUStateChangeCompleteResponse(
                                DUStateChangeCompleteResponse {},
                            ))
                        }
                        "DUStateChangeComplete" => {
                            self.body.push(BodyElement::DUStateChangeComplete(
                                DUStateChangeComplete::new("", vec![]),
                            ))
                        }
                        "FactoryResetResponse" => self
                            .body
                            .push(BodyElement::FactoryResetResponse(FactoryResetResponse {})),
                        "FactoryReset" => {
                            self.body.push(BodyElement::FactoryReset(FactoryReset {}))
                        }
                        "Fault" => self
                            .body
                            .push(BodyElement::Fault(Fault::new("", "", 0, ""))),
                        "GetAllQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetAllQueuedTransfersResponse(
                                GetAllQueuedTransfersResponse::new(vec![]),
                            ))
                        }
                        "GetAllQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {})),
                        "GetOptionsResponse" => self.body.push(BodyElement::GetOptionsResponse(
                            GetOptionsResponse::new(vec![]),
                        )),
                        "GetOptions" => self.body.push(BodyElement::GetOptions(Default::default())),
                        "GetParameterAttributes" => self.body.push(
                            BodyElement::GetParameterAttributes(GetParameterAttributes {
                                parameternames: vec![],
                            }),
                        ),
                        "GetParameterAttributesResponse" => {
                            self.body.push(BodyElement::GetParameterAttributesResponse(
                                GetParameterAttributesResponse { parameters: vec![] },
                            ))
                        }
                        "GetParameterNamesResponse" => {
                            self.body.push(BodyElement::GetParameterNamesResponse(
                                GetParameterNamesResponse::default(),
                            ))
                        }
                        "GetParameterNames" => self
                            .body
                            .push(BodyElement::GetParameterNames(GetParameterNames::default())),
                        "GetParameterValues" => {
                            self.body
                                .push(BodyElement::GetParameterValues(GetParameterValues {
                                    parameternames: vec![],
                                }))
                        }
                        "GetParameterValuesResponse" => {
                            self.body.push(BodyElement::GetParameterValuesResponse(
                                GetParameterValuesResponse { parameters: vec![] },
                            ))
                        }
                        "GetQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetQueuedTransfersResponse(
                                GetQueuedTransfersResponse::new(vec![]),
                            ))
                        }
                        "GetQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetQueuedTransfers(GetQueuedTransfers {})),
                        "GetRPCMethodsResponse" => self.body.push(
                            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::new(vec![])),
                        ),
                        "GetRPCMethods" => {
                            self.body.push(BodyElement::GetRPCMethods(GetRPCMethods {}))
                        }
                        _ => {}
                    }
                }
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
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

    fn end_handler(&mut self, path: &Vec<String>, _name: &xml::name::OwnedName) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            // match the ones who actually need and end_handler, and call their
            // respective end_handler
            _ => {}
        }
    }

    fn characters(&mut self, path: &Vec<String>, characters: &String) {
        // println!("Path: {:?} Chars: {}", path, characters);
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope", "Header", "ID"] => {
                // find the ID header element created by start_handler of Envelope, and
                // set the id tag therein
                for elem in self.header.iter_mut() {
                    match elem {
                        HeaderElement::ID(ref data) => {
                            let new_id = HeaderElement::ID(ID {
                                must_understand: data.must_understand,
                                id: characters.to_string(),
                            });
                            *elem = new_id;
                            println!("New header ID element set");
                        }
                        _ => println!("Dont care about elem: {:?}", elem),
                    }
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AddObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::AddObject(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::AutonomousTransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::CancelTransfer(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DeleteObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DeleteObject(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DownloadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Download(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Fault(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetOptions(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterNames(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(unhandled) => {
                        println!("characters for {:?} is so far unhandled", unhandled);
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

// private functions
fn extract_attribute(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    attrib_name: &str,
) -> String {
    let f = attributes
        .iter()
        .filter(|&x| x.name.local_name == attrib_name)
        .next();
    match f {
        Some(e) => e.value.to_string(),
        None => String::from(""),
    }
}

pub trait Parseable {}
impl Parseable for u32 {}
impl Parseable for u8 {}

fn parse_to_int<T: Parseable + std::str::FromStr>(chars: &String, default: T) -> T {
    match chars.parse::<T>() {
        Ok(parsed) => parsed,
        _ => default,
    }
}

impl State {
    pub fn new() -> Self {
        State {
            last_text: String::from(""),
            envelope: Envelope::new(),
            path: vec![],
        }
    }
    pub fn start_handler(
        &mut self,
        name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
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

pub struct State {
    pub path: Vec<String>,
    pub last_text: String,
    pub envelope: Envelope,
}
