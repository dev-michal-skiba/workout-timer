mod terminal;
mod timer;
mod user_options;

use std::collections::HashMap;

use timer::Timer;
use user_options::{
    Option, EXCERCISE_QUANTITY_KEY, EXCERCISE_REST_TIME_KEY, EXCERCISE_TIME_KEY, SET_QUANTITY_KEY,
    SET_REST_TIME_KEY,
};

fn main() {
    let mut terminal = terminal::get().expect("Failed to get terminal");
    let user_options_result: (u8, HashMap<u8, Option>) = user_options::get(&mut terminal);
    let user_options_status: u8 = user_options_result.0;
    let user_options: HashMap<u8, Option> = user_options_result.1;
    if user_options_status == 0 {
        let timer: Timer = Timer::new(
            user_options[&EXCERCISE_TIME_KEY].value,
            user_options[&EXCERCISE_QUANTITY_KEY].value,
            user_options[&EXCERCISE_REST_TIME_KEY].value,
            user_options[&SET_QUANTITY_KEY].value,
            user_options[&SET_REST_TIME_KEY].value,
        );
        timer::run(&mut terminal, timer).expect("Failed to run timer");
    }
    terminal::restore(terminal).expect("Failed to restore terminal");
}
