use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge},
    Frame, Terminal,
};

const TICK_RATE: Duration = Duration::from_secs(1);

#[derive(PartialEq, Eq, Hash)]
enum TimerState {
    Set,
    SetRest,
    ExcerciseRest,
}

struct Time {
    current: u32,
    current_text: String,
    max: u32,
    max_text: String,
}

struct Workout {
    progress: u16,
    is_finished: bool,
    time: Time,
}

struct Set {
    progress: u16,
    is_finished: bool,
    time: Time,
    current: u32,
    quantity: u32,
}

struct Excercise {
    progress: u16,
    is_finished: bool,
    time: Time,
    current: u32,
    quantity: u32,
}

struct Rest {
    progress: u16,
    is_finished: bool,
    time: Time,
}

pub struct Timer {
    workout: Workout,
    set: Set,
    set_rest: Rest,
    excercise: Excercise,
    excercise_rest: Rest,
    state: TimerState,
}

impl Workout {
    fn new(max_time: u32) -> Workout {
        let max_time_text: String = get_time_text(max_time);
        Workout {
            progress: 0,
            is_finished: false,
            time: Time {
                current: 0,
                current_text: String::from("00:00:00"),
                max: max_time,
                max_text: max_time_text,
            },
        }
    }

    fn on_tick(&mut self) {
        if self.is_finished == false {
            self.time.current += 1;
            self.time.current_text = get_time_text(self.time.current);
            self.progress = u16::try_from(self.time.current * 100 / self.time.max)
                .expect("Failed to calculate full time progress");
            if self.time.current == self.time.max {
                self.is_finished = true;
            }
        }
    }

    fn get_label(&mut self) -> String {
        format!("{}/{}", self.time.current_text, self.time.max_text)
    }
}

impl Set {
    fn new(max_time: u32, quantity: u32) -> Set {
        Set {
            progress: 0,
            is_finished: false,
            time: Time {
                current: 0,
                current_text: get_time_text(0),
                max: max_time,
                max_text: get_time_text(max_time),
            },
            current: 1,
            quantity: quantity,
        }
    }

    fn on_tick(&mut self, increment: u32) {
        if self.is_finished == false {
            self.time.current += increment;
            self.time.current_text = get_time_text(self.time.current);
            self.progress = u16::try_from(self.time.current * 100 / self.time.max)
                .expect("Failed to calculate full time progress");
            if self.time.current == self.time.max {
                self.is_finished = true;
            }
        }
    }

    fn clear(&mut self) {
        self.is_finished = false;
        self.progress = 0;
        self.time.current = 0;
        self.time.current_text = get_time_text(0);
    }

    fn increment(&mut self) {
        self.current += 1;
    }

    fn get_title(&mut self) -> String {
        format!("Set {}/{} Timer", self.current, self.quantity)
    }

    fn get_label(&mut self) -> String {
        format!("{}/{}", self.time.current_text, self.time.max_text)
    }
}

impl Excercise {
    fn new(max_time: u32, quantity: u32) -> Excercise {
        Excercise {
            progress: 0,
            is_finished: false,
            time: Time {
                current: 0,
                current_text: get_time_text(0),
                max: max_time,
                max_text: get_time_text(max_time),
            },
            current: 1,
            quantity: quantity,
        }
    }

    fn on_tick(&mut self, increment: u32) {
        if self.is_finished == false {
            self.time.current += increment;
            self.time.current_text = get_time_text(self.time.current);
            self.progress = u16::try_from(self.time.current * 100 / self.time.max)
                .expect("Failed to calculate full time progress");
            if self.time.current == self.time.max {
                self.is_finished = true;
            }
        }
    }

    fn clear(&mut self) {
        self.is_finished = false;
        self.progress = 0;
        self.time.current = 0;
        self.time.current_text = get_time_text(0);
    }

    fn increment(&mut self) {
        self.current += 1;
        if self.current > self.quantity {
            self.current = 1;
        }
    }

    fn get_title(&mut self) -> String {
        format!("Excercise {}/{} Timer", self.current, self.quantity)
    }

    fn get_label(&mut self) -> String {
        format!("{}/{}", self.time.current_text, self.time.max_text)
    }
}

impl Rest {
    fn new(max_time: u32) -> Rest {
        let max_time_text: String = get_time_text(max_time);
        Rest {
            progress: 0,
            is_finished: false,
            time: Time {
                current: 0,
                current_text: String::from("00:00:00"),
                max: max_time,
                max_text: max_time_text,
            },
        }
    }

    fn on_tick(&mut self, increment: u32) {
        if self.is_finished == false {
            self.time.current += increment;
            self.time.current_text = get_time_text(self.time.current);
            self.progress = u16::try_from(self.time.current * 100 / self.time.max)
                .expect("Failed to calculate full time progress");
            if self.time.current == self.time.max {
                self.is_finished = true;
            }
        }
    }

    fn clear(&mut self) {
        self.is_finished = false;
        self.progress = 0;
        self.time.current = 0;
        self.time.current_text = get_time_text(0);
    }

    fn get_label(&mut self) -> String {
        format!("{}/{}", self.time.current_text, self.time.max_text)
    }
}

impl Timer {
    pub fn new(
        excercise_time: u32,
        excercise_quantity: u32,
        excercise_rest_time: u32,
        set_quantity: u32,
        set_rest_time: u32,
    ) -> Timer {
        let mut transitions = 2 * excercise_quantity - 2;
        let set_time = excercise_quantity * excercise_time
            + (excercise_quantity - 1) * excercise_rest_time
            + transitions;
        transitions = 2 * set_quantity - 2;
        let max_time = set_quantity * set_time + (set_quantity - 1) * set_rest_time + transitions;
        Timer {
            workout: Workout::new(max_time),
            set: Set::new(set_time, set_quantity),
            set_rest: Rest::new(set_rest_time),
            excercise: Excercise::new(excercise_time, excercise_quantity),
            excercise_rest: Rest::new(excercise_rest_time),
            state: TimerState::Set,
        }
    }

    fn on_tick(&mut self) {
        let mut set_increment: u32 = 1;
        let mut set_rest_increment: u32 = 1;
        let mut excercise_increment: u32 = 1;
        let mut excercise_rest_increment: u32 = 1;
        if self.state == TimerState::Set && self.set.is_finished {
            self.state = TimerState::SetRest;
            self.set.clear();
            self.set.increment();
            self.excercise.clear();
            self.excercise.increment();
            set_rest_increment = 0;
        } else if self.state == TimerState::Set && self.excercise.is_finished {
            self.state = TimerState::ExcerciseRest;
            self.excercise.clear();
            self.excercise.increment();
            excercise_rest_increment = 0;
        } else if self.state == TimerState::SetRest && self.set_rest.is_finished {
            self.state = TimerState::Set;
            self.set_rest.clear();
            set_increment = 0;
            excercise_increment = 0;
        } else if self.state == TimerState::ExcerciseRest && self.excercise_rest.is_finished {
            self.state = TimerState::Set;
            self.excercise_rest.clear();
            excercise_increment = 0;
        }

        self.workout.on_tick();
        if self.state == TimerState::Set {
            self.set.on_tick(set_increment);
            self.excercise.on_tick(excercise_increment);
        }
        if self.state == TimerState::SetRest {
            self.set_rest.on_tick(set_rest_increment);
        }
        if self.state == TimerState::ExcerciseRest {
            self.set.on_tick(set_increment);
            self.excercise_rest.on_tick(excercise_rest_increment);
        }
    }
}

fn get_time_text(mut time: u32) -> String {
    let hours: u32 = time / 60 / 60;
    time -= hours * 60 * 60;
    let minutes: u32 = time / 60;
    time -= minutes * 60;
    format!("{hours:02}:{minutes:02}:{time:02}")
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut timer: Timer) -> io::Result<()> {
    // let mut timer: Timer = Timer::new(10, 2, 5, 5, 10);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame: &mut Frame<'_, B>| tui(frame, &mut timer))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= TICK_RATE && timer.workout.is_finished == false {
            timer.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn tui<B: Backend>(frame: &mut Frame<B>, timer: &mut Timer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(4),
                Constraint::Max(4),
                Constraint::Max(4),
                Constraint::Max(4),
            ]
            .as_ref(),
        )
        .split(frame.size());

    // Full Workout Timer
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("Full Workout Timer")
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .percent(timer.workout.progress)
        .label(timer.workout.get_label());
    frame.render_widget(gauge, chunks[0]);

    if timer.state == TimerState::Set || timer.state == TimerState::ExcerciseRest {
        // Set Timer
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(timer.set.get_title())
                    .borders(Borders::ALL),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Rgb(255, 140, 0))
                    .add_modifier(Modifier::BOLD),
            )
            .percent(timer.set.progress)
            .label(timer.set.get_label());
        frame.render_widget(gauge, chunks[1]);
    } else if timer.state == TimerState::SetRest {
        // Set Rest Timer
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Set Rest Timer")
                    .borders(Borders::ALL),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .percent(timer.set_rest.progress)
            .label(timer.set_rest.get_label());
        frame.render_widget(gauge, chunks[1]);
    }

    if timer.state == TimerState::SetRest {
    } else if timer.state == TimerState::Set {
        // Excercise Timer
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(timer.excercise.get_title())
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .percent(timer.excercise.progress)
            .label(timer.excercise.get_label());
        frame.render_widget(gauge, chunks[2]);
    } else if timer.state == TimerState::ExcerciseRest {
        // Excercise Rest Timer
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Excercise Rest Timer")
                    .borders(Borders::ALL),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .percent(timer.excercise_rest.progress)
            .label(timer.excercise_rest.get_label());
        frame.render_widget(gauge, chunks[2]);
    }
}
