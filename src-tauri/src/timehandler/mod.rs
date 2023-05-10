use std::{thread::sleep, time::Duration};

struct TimeHandlerStruct;

impl TimeHandler for TimeHandlerStruct {
    fn get_polling_delay(&self) -> Box<dyn Fn() -> ()> {
        Box::new(move || sleep(Duration::from_secs(2)))
    }
}

pub trait TimeHandler {
    fn get_polling_delay(&self) -> Box<dyn Fn() -> ()>;
}
