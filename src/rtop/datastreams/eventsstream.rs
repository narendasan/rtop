
extern crate rand;
use self::rand::thread_rng;

pub struct EventStream<'a> {
    events: Vec<(&'a str, &'a str)>,
    rng: rand::ThreadRng,
} 

impl EventStream<'a> {
    pub fn new() -> Self<'a> {
        EventStream {
            events: vec![
                        ("Event1", "INFO"),
                        ("Event2", "INFO"),
                        ("Event3", "CRITICAL"),
                        ("Event4", "ERROR"),
                        ("Event5", "INFO"),
                        ("Event6", "INFO"),
                        ("Event7", "WARNING"),
                        ("Event8", "INFO"),
                        ("Event9", "INFO"),
                        ("Event10", "INFO"),
                        ("Event11", "CRITICAL"),
                        ("Event12", "INFO"),
                        ("Event13", "INFO"),
                        ("Event14", "INFO"),
                        ("Event15", "INFO"),
                        ("Event16", "INFO"),
                        ("Event17", "ERROR"),
                        ("Event18", "ERROR"),
                        ("Event19", "INFO"),
                        ("Event20", "INFO"),
                        ("Event21", "WARNING"),
                        ("Event22", "INFO"),
                        ("Event23", "INFO"),
                        ("Event24", "WARNING"),
                        ("Event25", "INFO"),
                        ("Event26", "INFO"),
                    ],
            rng: thread_rng(),
        }
    }

    pub fn next(&mut self<'a>) -> (&'a str, &'a str) {
        let event = self.events[self.itr];
        self.iter += 1;
        return event;
    }
}
