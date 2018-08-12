#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: hours, minutes: minutes }.normalize()
    }

    fn normalize(self) -> Self {
        let minutes = self.minutes % 60 +
            if self.minutes < 0 {
                60
            } else {
                0
            };

        let hours = (self.hours -
            if self.minutes < 0 {
                1
            } else {
                0
            } + self.minutes / 60) % 24;


        Clock { hours: if hours < 0 { hours + 24 } else { hours }, minutes: minutes }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock { hours: self.hours, minutes: self.minutes + minutes}.normalize()
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}