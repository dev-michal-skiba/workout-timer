mod terminal;
mod timer;
mod user_options;

use std::collections::HashMap;

use timer::Timer;
use user_options::{Option, OptionKey};

fn main() {
    let mut terminal = terminal::get().expect("Failed to get terminal");
    let user_options: HashMap<OptionKey, Option> = user_options::get();
    let timer: Timer = Timer::new(
        user_options[&OptionKey::ExcerciseTime].value,
        user_options[&OptionKey::ExcerciseQuantity].value,
        user_options[&OptionKey::ExcerciseRestTime].value,
        user_options[&OptionKey::SetQuantity].value,
        user_options[&OptionKey::SetRestTime].value,
    );
    timer::run(&mut terminal, timer).expect("Failed to run timer");
    terminal::restore(terminal).expect("Failed to restore terminal");
}
