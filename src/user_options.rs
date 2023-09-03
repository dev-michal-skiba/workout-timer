use std::collections::HashMap;

static DEFAULT_EXCERCISE_TIME_NAME: &str = "Excercise time in seconds";
static DEFAULT_EXCERCISE_QUANTITY_NAME: &str = "Number of excercises per onse set";
static DEFAULT_EXCERCISE_REST_TIME_NAME: &str = "Rest time between excercises in seconds";
static DEFAULT_SET_QUANTITY_NAME: &str = "Number of sets";
static DEFAULT_SET_REST_TIME_NAME: &str = "Rest time between sets in seconds";

const DEFAULT_EXCERCISE_TIME_VALUE: u32 = 45;
const DEFAULT_EXCERCISE_QUANTITY_VALUE: u32 = 10;
const DEFAULT_EXCERCISE_REST_TIME_VALUE: u32 = 15;
const DEFAULT_SET_QUANTITY_VALUE: u32 = 3;
const DEFAULT_SET_REST_TIME_VALUE: u32 = 120;

#[derive(PartialEq, Eq, Hash)]
pub enum OptionKey {
    ExcerciseTime,
    ExcerciseQuantity,
    ExcerciseRestTime,
    SetQuantity,
    SetRestTime,
}

pub struct Option {
    pub name: String,
    pub value: u32,
}

fn get_default() -> HashMap<OptionKey, Option> {
    return HashMap::from([
        (
            OptionKey::ExcerciseTime,
            Option {
                name: DEFAULT_EXCERCISE_TIME_NAME.to_string(),
                value: DEFAULT_EXCERCISE_TIME_VALUE,
            },
        ),
        (
            OptionKey::ExcerciseQuantity,
            Option {
                name: DEFAULT_EXCERCISE_QUANTITY_NAME.to_string(),
                value: DEFAULT_EXCERCISE_QUANTITY_VALUE,
            },
        ),
        (
            OptionKey::ExcerciseRestTime,
            Option {
                name: DEFAULT_EXCERCISE_REST_TIME_NAME.to_string(),
                value: DEFAULT_EXCERCISE_REST_TIME_VALUE,
            },
        ),
        (
            OptionKey::SetQuantity,
            Option {
                name: DEFAULT_SET_QUANTITY_NAME.to_string(),
                value: DEFAULT_SET_QUANTITY_VALUE,
            },
        ),
        (
            OptionKey::SetRestTime,
            Option {
                name: DEFAULT_SET_REST_TIME_NAME.to_string(),
                value: DEFAULT_SET_REST_TIME_VALUE,
            },
        ),
    ]);
}

pub fn get() -> HashMap<OptionKey, Option> {
    let user_options: HashMap<OptionKey, Option> = get_default();
    return user_options;
}
