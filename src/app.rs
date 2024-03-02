use std::{env, fs::File, io, ops::Deref};
use chrono::{DateTime, Local};
use color_eyre::{Result, eyre::bail};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use crate::tui::Tui;

const TIMERS_FILENAME: &str = "timers.csv";

#[derive(Debug, Default)]
pub struct App {
    state: State,
    timers: Timers,
    exit: bool,
}

#[derive(Debug, Default)]
struct Timers {
    selected_idx: usize,
    timers: Vec<Timer>,
}

impl Deref for Timers {
    type Target = Vec<Timer>;
    fn deref(&self) -> &Self::Target {
        &self.timers
    }
}

impl Timers {
    fn selected(&self) -> &Timer {
        &self[self.selected_idx]
    }
}

#[derive(Debug, Default)]
enum State {
    #[default]
    NewTimer,
    ViewTimers,
}

#[derive(Debug)]
struct Timer {
    name: String,
    datetime: DateTime<Local>,
}

fn parse_timers(mut rdr: csv::Reader<File>) -> Result<Timers> {
    let mut timers = Vec::new();
    let mut selected_timer_idx = 0;
    for (i, res) in rdr.records().enumerate() {
        let record = res?;
        let name = &record[0];
        let datetime = record[1].parse()?;
        selected_timer_idx = if &record[2] == "1" {
            i
        } else {
            selected_timer_idx
        };
        timers.push(Timer {
            name: name.into(),
            datetime,
        })
    }
    Ok(Timers {
        selected_idx: selected_timer_idx,
        timers,
    })
}

impl App {
    pub fn new() -> Result<Self> {
        match csv::Reader::from_path(env::current_dir()?.with_file_name(TIMERS_FILENAME)) {
            Ok(rdr) => Ok(App {
                timers: parse_timers(rdr)?,
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
