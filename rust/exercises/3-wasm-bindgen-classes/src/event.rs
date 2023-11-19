pub struct Event {
    pub name: String,
    pub year: u16,
}

#[derive(Default)]
pub struct EventBuilder {
    name: String,
    year: Option<u16>,
}

impl EventBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_year(&mut self, year: u16) -> &Self {
        self.year = Some(year);
        self
    }

    pub fn build(self) -> Event {
        Event {
            name: self.name,
            year: self.year.unwrap_or(2023),
        }
    }
}
