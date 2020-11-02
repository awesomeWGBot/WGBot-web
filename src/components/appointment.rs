use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, PartialEq)]
pub struct Appointment {
    day: NaiveDate,
    start: Option<NaiveTime>,
    end: Option<NaiveTime>,
    header: String,
    description: String,
}

impl Appointment {
    pub fn new(day: NaiveDate, start: Option<NaiveTime>, end: Option<NaiveTime>, header: String, description: String) -> Appointment {
        Appointment {
            day,
            start,
            end,
            header,
            description,
        }
    }
    pub fn get_day(&self) -> NaiveDate {
        self.day
    }
    pub fn get_start(&self) -> Option<NaiveTime> {
        self.start
    }
    pub fn get_end(&self) -> Option<NaiveTime> {
        self.end
    }
    pub fn get_header(&self) -> String {
        self.header.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}