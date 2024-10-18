#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TimeWindow {
    pub window_start: u32,
    pub window_end: u32,
    pub window_mode: String,
    pub user_message: String,
    pub max_retries: i32,
}
impl TimeWindow {
    #[must_use]
    pub fn new(
        window_start: u32,
        window_end: u32,
        window_mode: String,
        user_message: String,
        max_retries: i32,
    ) -> Self {
        TimeWindow {
            window_start,
            window_end,
            window_mode,
            user_message,
            max_retries,
        }
    }
}

#[cfg(test)]
impl Arbitrary for TimeWindow {
    fn arbitrary(g: &mut Gen) -> Self {
        TimeWindow::new(
            u32::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            i32::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.window_start.clone(),
                self.window_end.clone(),
                self.window_mode.clone(),
                self.user_message.clone(),
                self.max_retries.clone(),
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
