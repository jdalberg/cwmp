#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TimeWindow {
    pub window_start: u32,
    pub window_end: u32,
    pub window_mode: XmlSafeString,
    pub user_message: XmlSafeString,
    pub max_retries: i32,
}
impl TimeWindow {
    #[must_use]
    pub fn new(
        window_start: u32,
        window_end: u32,
        window_mode: &str,
        user_message: &str,
        max_retries: i32,
    ) -> Self {
        TimeWindow {
            window_start,
            window_end,
            window_mode: window_mode.into(),
            user_message: user_message.into(),
            max_retries,
        }
    }
}

#[cfg(test)]
impl Arbitrary for TimeWindow {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            window_start: u32::arbitrary(g),
            window_end: u32::arbitrary(g),
            window_mode: XmlSafeString::arbitrary(g),
            user_message: XmlSafeString::arbitrary(g),
            max_retries: i32::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.window_start,
                self.window_end,
                self.window_mode.clone(),
                self.user_message.clone(),
                self.max_retries,
            )
                .shrink()
                .map(|(ws, we, wm, um, mr)| TimeWindow {
                    window_start: ws,
                    window_end: we,
                    window_mode: wm,
                    user_message: um,
                    max_retries: mr,
                }),
        )
    }
}
