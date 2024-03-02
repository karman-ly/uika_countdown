use crate::tui::Tui;
use chrono::{DateTime, Local};
use color_eyre::{
    eyre::{self, bail},
    Result,
};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use std::{env, fs::File, io, ops::Deref};

const TIMERS_FILENAME: &str = "timers.csv";

#[derive(Debug, Default)]
pub struct App {
    state: State,
    timers: Countdowns,
    exit: bool,
}

#[derive(Debug, Default)]
struct Countdowns {
    selected_idx: usize,
    timers: Vec<Countdown>,
}

impl Deref for Countdowns {
    type Target = Vec<Countdown>;
    fn deref(&self) -> &Self::Target {
        &self.timers
    }
}

impl Countdowns {
    fn selected(&self) -> &Countdown {
        &self[self.selected_idx]
    }
}

impl TryFrom<csv::Reader<File>> for Countdowns {
    type Error = eyre::Error;
    fn try_from(mut rdr: csv::Reader<File>) -> Result<Self, Self::Error> {
        let timers = Vec::new();
        let selected_idx = rdr
            .records()
            .position(|record| record.is_ok_and(|r| &r[3] == "1"))
            .unwrap_or(0);
        Ok(Countdowns {
            selected_idx,
            timers,
        })
    }
}

#[derive(Debug)]
enum State {
    NewCountdown(NewCountdownState),
    ViewTimers,
}

impl Default for State {
    fn default() -> Self {
        Self::NewCountdown(NewCountdownState::Name)
    }
}

#[derive(Debug, Default)]
enum NewCountdownState {
    #[default]
    Name,
    Color,
    DateTime,
}

#[derive(Debug)]
struct Countdown {
    name: String,
    color: Color,
    datetime: DateTime<Local>,
}

impl TryFrom<csv::StringRecord> for Countdown {
    type Error = eyre::Error;
    fn try_from(record: csv::StringRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            name: record[0].into(),
            color: record[1].parse()?,
            datetime: record[2].parse()?,
        })
    }
}

impl App {
    pub fn new() -> Result<Self> {
        match csv::Reader::from_path(env::current_dir()?.with_file_name(TIMERS_FILENAME)) {
            Ok(rdr) => Ok(App {
                timers: Countdowns::try_from(rdr)?,
                state: State::ViewTimers,
                ..App::default()
            }),
            Err(e) => match e.kind() {
                csv::ErrorKind::Io(e) if e.kind() == io::ErrorKind::NotFound => Ok(App::default()),
                _ => bail!("Error reading timers file"),
            },
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&*self, frame.size()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        todo!()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
