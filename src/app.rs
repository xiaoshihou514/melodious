use crate::fs::Library;
use crate::player::Player;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use unicode_width::UnicodeWidthStr;

pub enum Focus {
    StatusBar,
    SearchBar,
    File,
    List,
}

pub enum Sort {
    Alphabetical,
    TimeAdded,
}

pub enum IndexStatus {
    Ready,        // Ready
    Initializing, // Initializing and cannot query for formatted string
    Refreshing,   // Ready but updating index
}
pub enum PlayerStatus {
    Ready,        // Ready
    Initializing, // Initializing and cannot query for formatted string
}

pub struct App {
    // ui related vars
    focus: Focus,
    search: String,
    list_items: Vec<String>,      // what shows in the list col
    list_context: (String, bool), // what is the list showing and whether it needs refreshing
    dir: (bool, Vec<String>),     // whether to show the dir side bar and its contents
    order: (Sort, bool),          // sorting method(Alphabetical vs Time_added) and a->z / z->a
    index_status: IndexStatus,
    player_status: PlayerStatus,

    // access to other stuff
    index: Library,
    player: Player,
}

impl Default for App {
    fn default() -> Self {
        App {
            focus: Focus::StatusBar,
            search: String::from("  "),
            list_items: Vec::new(),
            list_context: (String::from("Music"), true),
            dir: (false, Vec::new()),
            order: (Sort::TimeAdded, false),
            index_status: IndexStatus::Initializing,
            player_status: PlayerStatus::Initializing,

            //called once, use for a lifetime
            index: Library::new(),
            player: Player::new(),
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 40),
                Constraint::Ratio(36, 40),
                Constraint::Ratio(2, 40),
            ]
            .as_ref(),
        )
        .split(f.size());
    let searchbar = Paragraph::new(app.search.as_ref())
        .style(match app.focus {
            Focus::SearchBar => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(searchbar, chunks[0]);
    match app.focus {
        Focus::SearchBar => {
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.search.width() as u16 + 1,
                chunks[1].y,
            );
        }
        _ => {}
    }
    if app.list_context.1 {
        // TODO:  make this async and callback when finished
        app.list_context = app.index.query(app.list_context.0)
    }
    let list_items: Vec<ListItem> = app
        .list_items
        .iter()
        .enumerate()
        .map(|(_, item)| ListItem::new(format!("  {}", item)))
        .collect();
    // Create title
    let mut title = app.list_context.0.clone();
    title.push(' ');
    title.insert(0, ' ');
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title(title));
    if app.dir.0 {
        // render sidebar
        let mid_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);
        // TODO
        let dir_str = app.index.get_formatted_dirs();
        let dir_items: Vec<ListItem> = dir_str
            .iter()
            .enumerate()
            .map(|(_, item)| ListItem::new(item.as_ref()))
            .collect();
        let dir_list = List::new(dir_items).block(Block::default().borders(Borders::RIGHT));
        f.render_widget(dir_list, mid_chunk[0]);
        f.render_widget(list, mid_chunk[1]);
    } else {
        f.render_widget(list, chunks[1]);
    }
    let status_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(chunks[2]);
    let progress = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(0);
    let mut formatted: String = app.player.gen_string();
    let margin = status_bar[0].width / 2;
    for _ in 0..margin {
        formatted.push(' ');
        formatted.insert(0, ' ');
    }
    let status = Paragraph::new(formatted)
        .style(Style::default().fg(Color::Green))
        .block(Block::default());
    f.render_widget(progress, status_bar[0]);
    f.render_widget(status, status_bar[1]);
}

// Main loop
impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        'lifetime: loop {
            if let Event::Key(key) = event::read()? {
                // Process pressed key
                //  / focus on search
                //  l focus on song list
                //  f focus on directory list
                //  b focus on status bar (controls)
                //  q quit
                match self.focus {
                    Focus::StatusBar => match key.code {
                        KeyCode::Char('/') => self.focus = Focus::SearchBar,
                        KeyCode::Char('l') => self.focus = Focus::List,
                        KeyCode::Char('f') => self.focus = Focus::File,
                        KeyCode::Char('q') => break 'lifetime Ok(()),
                        _ => {}
                    },
                    Focus::SearchBar => match key.code {
                        KeyCode::Esc => self.focus = Focus::StatusBar,
                        KeyCode::Char(c) => {
                            self.search.push(c);
                        }
                        KeyCode::Backspace => {
                            if self.search != "  " {
                                self.search.pop();
                            }
                        }
                        // TODO
                        KeyCode::Enter => {
                            self.list_items = vec![];
                            self.search = "  ".to_string();
                        }
                        _ => {}
                    },
                    Focus::File => match key.code {
                        KeyCode::Char('/') => self.focus = Focus::SearchBar,
                        KeyCode::Char('l') => self.focus = Focus::List,
                        KeyCode::Char('b') => self.focus = Focus::StatusBar,
                        KeyCode::Char('q') => break 'lifetime Ok(()),
                        _ => {}
                    },
                    Focus::List => match key.code {
                        KeyCode::Char('/') => self.focus = Focus::SearchBar,
                        KeyCode::Char('f') => self.focus = Focus::File,
                        KeyCode::Char('b') => self.focus = Focus::StatusBar,
                        KeyCode::Char('q') => break 'lifetime Ok(()),
                        _ => {}
                    },
                }
                // Draw ui accordingly
                terminal.draw(|f| ui::<B>(f, &self))?;
            }
        }
    }
}
