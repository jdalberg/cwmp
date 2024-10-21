use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::{FaultStruct, XmlSafeString};

#[cfg(test)]
use super::gen_utc_date;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonOpResult {
    pub uuid: XmlSafeString,
    pub deployment_unit_ref: XmlSafeString,
    pub version: XmlSafeString,
    pub current_state: XmlSafeString,
    pub resolved: XmlSafeString,
    pub execution_unit_ref_list: XmlSafeString,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
    pub fault: FaultStruct,
    pub operation_performed: XmlSafeString,
}

impl AutonOpResult {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
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
        AutonOpResult {
            uuid: uuid.into(),
            deployment_unit_ref: deployment_unit_ref.into(),
            version: version.into(),
            current_state: current_state.into(),
            resolved: resolved.into(),
            execution_unit_ref_list: execution_unit_ref_list.into(),
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault: FaultStruct::new(fault_code, fault_string),
            operation_performed: operation_performed.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonOpResult {
    fn arbitrary(g: &mut Gen) -> Self {
        let bogus_st = gen_utc_date(2014, 11, 28, 12, 0, 9);
        let bogus_ct = gen_utc_date(2014, 11, 28, 12, 0, 9);

        Self {
            uuid: XmlSafeString::arbitrary(g),
            deployment_unit_ref: XmlSafeString::arbitrary(g),
            version: XmlSafeString::arbitrary(g),
            current_state: XmlSafeString::arbitrary(g),
            resolved: XmlSafeString::arbitrary(g),
            execution_unit_ref_list: XmlSafeString::arbitrary(g),
            start_time: Some(bogus_st),
            complete_time: Some(bogus_ct),
            fault: FaultStruct::arbitrary(g),
            operation_performed: XmlSafeString::arbitrary(g),
        }
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
                    uuid,
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
