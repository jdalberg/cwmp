use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use super::gen_utc_date;
use super::FaultStruct;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OpResult {
    pub uuid: String,
    pub deployment_unit_ref: String,
    pub version: String,
    pub current_state: String,
    pub resolved: u32,
    pub execution_unit_ref_list: String,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
    pub fault: FaultStruct,
}

impl OpResult {
    pub fn new(
        uuid: String,
        deployment_unit_ref: String,
        version: String,
        current_state: String,
        resolved: u32,
        execution_unit_ref_list: String,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault: FaultStruct,
    ) -> Self {
        OpResult {
            uuid,
            deployment_unit_ref,
            version,
            current_state,
            resolved,
            execution_unit_ref_list,
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OpResult {
    fn arbitrary(g: &mut Gen) -> Self {
        OpResult::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            gen_utc_date(2014, 11, 28, 12, 0, 9),
            gen_utc_date(2014, 11, 29, 12, 0, 9),
            FaultStruct::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.uuid.clone(),
                self.deployment_unit_ref.clone(),
                self.version.clone(),
                self.current_state.clone(),
                self.resolved.clone(),
                self.execution_unit_ref_list.clone(),
                self.fault.clone(),
            )
                .shrink()
                .map(|(u, dur, v, cs, r, eurl, f)| OpResult {
                    uuid: u,
                    deployment_unit_ref: dur,
                    version: v,
                    current_state: cs,
                    resolved: r,
                    execution_unit_ref_list: eurl,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                    fault: f,
                }),
        )
    }
}
