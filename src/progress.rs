use crate::{
    trace,
};



#[derive(Debug)]
pub struct ProgressHolder {
    total: u128,
    done : u128,

    pub state: String,
    with_error: bool,
}

impl ProgressHolder {
    pub fn new() -> ProgressHolder {
        ProgressHolder {
            total: 0,
            done: 0,

            with_error: false,

            state: String::default(),
        }
    }

    pub fn set_total(&mut self, total: u128) {
        self.total = total;
        self.state = trace::string::ok(format!("Total job to process: {total}"));
    }

    pub fn feedback(&mut self, current_step: u128) {
        self.done  = current_step;

        let percent = 100f64 * self.done as f64 / self.total as f64;
        self.state = format!("{}",
            trace::string::info(format!("In Progress ( {} / {} ) [ {:6.02} % ]", self.done, self.total, percent)),
        );
    }

    pub fn feedback_on_err(&mut self, msg: impl AsRef<str>) {
        self.state = trace::string::error(msg);
        self.with_error = true;
    }

    pub fn is_err(&self) -> bool {
        self.with_error
    }

    pub fn get_err(&self) -> &String {
        &self.state
    }
}
