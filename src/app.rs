use crate::tui::Tui;
use chrono::{DateTime, Local};
use color_eyre::{
    eyre::{self, bail},
    Result,
};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Tabs, Widget},
};
use std::{fs::File, io, ops::Deref};

const TIMERS_FILENAME: &str = "countdowns.csv";

#[derive(Debug, Default)]
pub struct App {
    state: State,
    countdowns: Countdowns,
    exit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        match csv::Reader::from_path(TIMERS_FILENAME) {
            Ok(rdr) => Ok(App {
                countdowns: Countdowns::try_from(rdr)?,
                state: State::ViewTimers,
                ..App::default()
            }),
            Err(e) => match e.kind() {
                csv::ErrorKind::Io(e) if e.kind() == io::ErrorKind::NotFound => Ok(App::default()),
                _ => bail!("Error reading countdowns file"),
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
        self.countdowns
            .iter()
            .map(|countdown| {
                countdown
                    .name
                    .clone()
                    .bg(countdown.bg_color)
                    .fg(countdown.fg_color)
            })
            .collect::<Tabs>()
            .render(area, buf);
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

#[derive(Debug)]
enum NewCountdownState {
    Name,
    BgColor,
    FgColor,
    DateTime,
}

#[derive(Debug, Default)]
struct Countdowns {
    selected_idx: usize,
    countdowns: Vec<Countdown>,
}

impl Deref for Countdowns {
    type Target = Vec<Countdown>;
    fn deref(&self) -> &Self::Target {
        &self.countdowns
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
        let selected_idx = rdr.headers()?[0].parse()?;
        let countdowns = rdr
            .records()
            .map(|record| Countdown::try_from(&record?))
            .collect::<Result<_, _>>()?;

        Ok(Countdowns {
            selected_idx,
            countdowns: dbg!(countdowns),
        })
    }
}

#[derive(Debug)]
struct Countdown {
    name: String,
    bg_color: Color,
    fg_color: Color,
    datetime: DateTime<Local>,
}

impl TryFrom<&csv::StringRecord> for Countdown {
    type Error = eyre::Error;
    fn try_from(record: &csv::StringRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            name: record[0].into(),
            bg_color: record[1].parse()?,
            fg_color: record[2].parse()?,
            datetime: record[3].parse::<DateTime<Local>>()?,
        })
    }
}
