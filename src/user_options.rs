use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::Backend,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use std::{collections::HashMap, io, time::Duration};

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

pub const EXCERCISE_TIME_KEY: u8 = 0;
pub const EXCERCISE_QUANTITY_KEY: u8 = 1;
pub const EXCERCISE_REST_TIME_KEY: u8 = 2;
pub const SET_QUANTITY_KEY: u8 = 3;
pub const SET_REST_TIME_KEY: u8 = 4;

pub struct Option {
    pub name: String,
    pub value: u32,
    pub max: u32,
}

const COMMON_HEADER: &str = "Workout Timer developed by dev-michal-skiba\n\n";
const USER_OPTIONS_HEADER: &str =
    "Please use AWSD or arrows to change the options\nType Enter to accept, q to exit\n\n";
const OPTION_KEYS_LIST: [u8; 5] = [
    EXCERCISE_TIME_KEY,
    EXCERCISE_QUANTITY_KEY,
    EXCERCISE_REST_TIME_KEY,
    SET_QUANTITY_KEY,
    SET_REST_TIME_KEY,
];
const MIN_OPTION_KEY_INDEX: usize = 0;
const MAX_OPTION_KEY_INDEX: usize = OPTION_KEYS_LIST.len() - 1;

fn get_default() -> HashMap<u8, Option> {
    return HashMap::from([
        (
            EXCERCISE_TIME_KEY,
            Option {
                name: DEFAULT_EXCERCISE_TIME_NAME.to_string(),
                value: DEFAULT_EXCERCISE_TIME_VALUE,
                max: 600,
            },
        ),
        (
            EXCERCISE_QUANTITY_KEY,
            Option {
                name: DEFAULT_EXCERCISE_QUANTITY_NAME.to_string(),
                value: DEFAULT_EXCERCISE_QUANTITY_VALUE,
                max: 20,
            },
        ),
        (
            EXCERCISE_REST_TIME_KEY,
            Option {
                name: DEFAULT_EXCERCISE_REST_TIME_NAME.to_string(),
                value: DEFAULT_EXCERCISE_REST_TIME_VALUE,
                max: 600,
            },
        ),
        (
            SET_QUANTITY_KEY,
            Option {
                name: DEFAULT_SET_QUANTITY_NAME.to_string(),
                value: DEFAULT_SET_QUANTITY_VALUE,
                max: 20,
            },
        ),
        (
            SET_REST_TIME_KEY,
            Option {
                name: DEFAULT_SET_REST_TIME_NAME.to_string(),
                value: DEFAULT_SET_REST_TIME_VALUE,
                max: 600,
            },
        ),
    ]);
}

fn tui<B: Backend>(
    frame: &mut Frame<B>,
    user_options: &mut HashMap<u8, Option>,
    active_index: usize,
) {
    let size = frame.size();
    let default_style: Style = Style::default();
    let active_style: Style = Style::default().fg(Color::LightGreen);
    let mut text = Text::styled(
        COMMON_HEADER,
        Style::default().add_modifier(Modifier::ITALIC),
    );
    text.extend(Text::raw(USER_OPTIONS_HEADER));
    for (index, option_key) in OPTION_KEYS_LIST.iter().enumerate() {
        let content: String = format!(
            "{}: {}",
            user_options[option_key].name, user_options[option_key].value
        );
        if index == active_index {
            text.extend(Text::styled(content, active_style));
        } else {
            text.extend(Text::styled(content, default_style));
        }
    }
    frame.render_widget(Paragraph::new(text).block(Block::new()), size);
}

fn get_user_options<B: Backend>(
    terminal: &mut Terminal<B>,
    mut user_options: &mut HashMap<u8, Option>,
) -> io::Result<u8> {
    let mut active_index: usize = 0;
    let mut new_value: u32 = 0;
    let mut option_key: u8;
    loop {
        terminal.draw(|frame: &mut Frame<'_, B>| tui(frame, &mut user_options, active_index))?;
        if crossterm::event::poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                option_key = OPTION_KEYS_LIST[active_index];
                if KeyCode::Enter == key.code {
                    return Ok(0);
                } else if KeyCode::Char('q') == key.code {
                    return Ok(1);
                } else if (KeyCode::Down == key.code || KeyCode::Char('s') == key.code)
                    && active_index < MAX_OPTION_KEY_INDEX
                {
                    active_index += 1;
                } else if (KeyCode::Up == key.code || KeyCode::Char('w') == key.code)
                    && active_index > MIN_OPTION_KEY_INDEX
                {
                    active_index -= 1;
                } else if KeyCode::Left == key.code || KeyCode::Char('a') == key.code {
                    if user_options[&option_key].value > 0 {
                        new_value = user_options[&option_key].value - 1;
                    }
                } else if KeyCode::Right == key.code || KeyCode::Char('d') == key.code {
                    if user_options[&option_key].value < user_options[&option_key].max {
                        new_value = user_options[&option_key].value + 1;
                    }
                }
                if new_value != 0 {
                    let updated_option = Option {
                        name: user_options[&option_key].name.clone(),
                        value: new_value,
                        max: user_options[&option_key].max,
                    };
                    user_options.insert(OPTION_KEYS_LIST[active_index], updated_option);
                    new_value = 0;
                }
            }
        }
    }
}

pub fn get<B: Backend>(mut terminal: &mut Terminal<B>) -> (u8, HashMap<u8, Option>) {
    let mut user_options: HashMap<u8, Option> = get_default();
    let status: u8 =
        get_user_options(&mut terminal, &mut user_options).expect("Failed to get user options");
    return (status, user_options);
}
