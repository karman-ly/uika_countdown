use crate::tui::Tui;
use chrono::{DateTime, Local};
use color_eyre::{
    eyre::{self, bail},
    Result,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::text::Span;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Padding, Paragraph, Tabs, Widget},
};
use std::time::Duration;
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
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Right => self.next_tab(),
            KeyCode::Left => self.prev_tab(),
            _ => {}
        }
        Ok(())
    }

    fn next_tab(&mut self) {
        self.countdowns.selected_idx = (self.countdowns.selected_idx + 1) % self.countdowns.len();
    }

    fn prev_tab(&mut self) {
        self.countdowns.selected_idx =
            (self.countdowns.selected_idx + self.countdowns.len() - 1) % self.countdowns.len();
    }

    fn render_view_countdowns(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let tab_count = self.countdowns.len();
        let tab_width = (area.width as usize - (tab_count - 1) * 3) / tab_count;

        let selected_countdown = self.countdowns.selected();

        self.countdowns
            .iter()
            .map(|countdown| format!("{:^tab_width$}", countdown.name))
            .collect::<Tabs>()
            .highlight_style(
                Style::new()
                    .italic()
                    .bold()
                    .bg(selected_countdown.bg_color)
                    .fg(selected_countdown.fg_color),
            )
            .select(self.countdowns.selected_idx)
            .block(
                Block::bordered()
                    .title(" Welcome to Uika Countdown! ".bold())
                    .title_alignment(Alignment::Center),
            )
            .render(layout[0], buf);

        let color_style = Style::default()
            .bg(selected_countdown.bg_color)
            .fg(selected_countdown.fg_color);

        let seconds = (selected_countdown.datetime - Local::now()).num_seconds();

        let countdown_block = Block::bordered()
            .border_type(BorderType::Double)
            .title_bottom(
                Line::from(vec![
                    " Next Tab".into(),
                    " <Right>".magenta().bold(),
                    " Prev Tab".into(),
                    " <Left>".magenta().bold(),
                    " New Countdown".into(),
                    " <N> ".magenta().bold(),
                    " Exit".into(),
                    " <?Guess?> ".magenta().bold(),
                ])
                .alignment(Alignment::Center),
            )
            .title_bottom(Line::from("❤ ").alignment(Alignment::Left))
            .title_bottom(Line::from("❤ ").alignment(Alignment::Right))
            .title(Line::from("❤ ").alignment(Alignment::Left))
            .title(Line::from("❤ ").alignment(Alignment::Right));

        let inner_area = countdown_block.inner(layout[1]);
        countdown_block.render(layout[1], buf);

        Paragraph::new(Text::from(vec![
            seconds.to_string().underlined().bold().italic().into(),
            "seconds till ".into(),
            selected_countdown.name.clone().bold().italic().into(),
        ]))
        .block(Block::new().padding(Padding::top((layout[1].height - 4) / 2)))
        .centered()
        .style(color_style)
        .render(inner_area, buf)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.state {
            State::NewCountdown(_) => {}
            State::ViewTimers => self.render_view_countdowns(area, buf),
        }
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
            countdowns,
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
            bg_color: Color::from_u32(u32::from_str_radix(&record[1], 16)?),
            fg_color: Color::from_u32(u32::from_str_radix(&record[2], 16)?),
            datetime: record[3].parse::<DateTime<Local>>()?,
        })
    }
}
