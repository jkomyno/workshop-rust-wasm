pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    // Returns the date in "DD/mm/YYYY" format.
    // TODO: call this `wasm` version of the method "fmtItalian"
    // instead of "fmt_italian".
    pub fn fmt_italian(&self) -> String {
        format!("{:02}/{:02}/{:04}", &self.day, &self.month, &self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_italian_test() {
        let date = Date::new(2023, 11, 19);
        assert_eq!(date.fmt_italian(), "19/11/2023");
    }
}
