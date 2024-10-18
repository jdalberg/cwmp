use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::FaultStruct;

#[cfg(test)]
use super::gen_utc_date;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonOpResult {
    pub uuid: String,
    pub deployment_unit_ref: String,
    pub version: String,
    pub current_state: String,
    pub resolved: String,
    pub execution_unit_ref_list: String,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
    pub fault: FaultStruct,
    pub operation_performed: String,
}

impl AutonOpResult {
    pub fn new(
        uuid: String,
        deployment_unit_ref: String,
        version: String,
        current_state: String,
        resolved: String,
        execution_unit_ref_list: String,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault_code: u32,
        fault_string: String,
        operation_performed: String,
    ) -> Self {
        AutonOpResult {
            uuid: uuid.to_string(),
            deployment_unit_ref: deployment_unit_ref.to_string(),
            version: version.to_string(),
            current_state: current_state.to_string(),
            resolved: resolved.to_string(),
            execution_unit_ref_list: execution_unit_ref_list.to_string(),
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault: FaultStruct::new(fault_code, fault_string),
            operation_performed: operation_performed.to_string(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonOpResult {
    fn arbitrary(g: &mut Gen) -> Self {
        let bogus_st = gen_utc_date(2014, 11, 28, 12, 0, 9);
        let bogus_ct = gen_utc_date(2014, 11, 28, 12, 0, 9);

        AutonOpResult::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            bogus_st,
            bogus_ct,
            u32::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        // we will remove times from shrinking since qc only supports a
        // tuple size of max 8, and then make the times constant across
        // arbitration
        Box::new(
            (
                self.uuid.clone(),
                self.deployment_unit_ref.clone(),
                self.version.clone(),
                self.current_state.clone(),
                self.resolved.clone(),
                self.execution_unit_ref_list.clone(),
                // only 8 elements allowed by quickcheck in a tuple
                // self.start_time.clone(),
                // self.complete_time.clone(),
                self.fault.clone(),
                self.operation_performed.clone(),
            )
                .shrink()
                .map(|(uuid, dur, ver, cs, res, eurl, f, op)| AutonOpResult {
                    uuid: uuid,
                    deployment_unit_ref: dur,
                    version: ver,
                    current_state: cs,
                    resolved: res,
                    execution_unit_ref_list: eurl,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                    fault: f,
                    operation_performed: op,
                }),
        )
    }
}
