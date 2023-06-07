use crate::app::App;

use std::{
    io::{self, Stdout},
    sync::mpsc,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, widgets::ListState, Terminal};

const SEARCH_ICON: &str = "  ";

pub enum Focus {
    Main,
    Queue,
    Controls,
    SearchBar,
    Directory,
}

pub struct Painter<'a> {
    term: Option<Terminal<CrosstermBackend<Stdout>>>,

    focus: Focus,             // window focus
    dir: Vec<&'a str>,        // items for the dir window
    dir_focused: ListState,   // directory selected in dir
    refreshing: bool,         // whether the main window is refreshing
    music: Vec<&'a str>,      // items for the main window
    music_focused: ListState, // music selected in main
    search: String,           // search context

    lib_sender: mpsc::Sender<String>,     // send reqs to library
    lib_receiver: mpsc::Receiver<String>, // receive signals from library

    pub told_me_to_exit: bool, // quit signal
}

impl Painter<'_> {
    pub fn new(lib_sender: mpsc::Sender<String>, lib_receiver: mpsc::Receiver<String>) -> Self {
        Painter {
            term: None,

            focus: Focus::Main,
            dir: Vec::new(),
            dir_focused: ListState::default(),
            refreshing: true,
            music: Vec::new(),
            music_focused: ListState::default(),
            search: String::new(),

            lib_sender: lib_sender,
            lib_receiver: lib_receiver,

            told_me_to_exit: false,
        }
    }

    pub fn init(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        self.term = Some(terminal);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.term.unwrap().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.term.unwrap().show_cursor()?;
        Ok(())
    }

    pub fn resolve_key(&mut self, app: &App) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.focus {
                    Focus::Main => self.resolve_key_main(app, key),
                    Focus::Controls => self.resolve_key_controls(app, key),
                    Focus::SearchBar => self.resolve_key_search_bar(app, key),
                    Focus::Directory => self.resolve_key_directory(app, key),
                    Focus::Queue => self.resolve_key_queue(app, key),
                }
            }
        }
        Ok(())
    }

    fn resolve_key_main(&mut self, app: &App, key: KeyEvent) {
        match key.code {
            // change selection
            KeyCode::Char('j') => match self.music_focused.selected() {
                None => self.music_focused.select(Some(0)),
                Some(u) => {
                    if u > self.music.len() - 1 {
                        self.music_focused.select(Some(0));
                    } else {
                        self.music_focused.select(Some(u + 1));
                    }
                }
            },
            KeyCode::Char('k') => match self.music_focused.selected() {
                None => self.music_focused.select(Some(0)),
                Some(u) => {
                    if u == 0 {
                        self.music_focused.select(Some(self.music.len() - 1));
                    } else {
                        self.music_focused.select(Some(u - 1));
                    }
                }
            },
            // control
            KeyCode::Char('a') => todo!(), // adds selected song to play queue
            KeyCode::Char('p') => todo!(), // play now
            KeyCode::Char('A') => todo!(), // adds all songs in current list to play queue
            // change focus
            KeyCode::Char('/') => self.focus = Focus::SearchBar,
            KeyCode::Char('e') => self.focus = Focus::Directory,
            KeyCode::Char('c') => self.focus = Focus::Controls,
        }
    }

    fn resolve_key_controls(&mut self, app: &App, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => todo!(), // pause & resume
            KeyCode::Char('n') => todo!(), // next song
            KeyCode::Char('p') => todo!(), // previous song
            KeyCode::Up => todo!(),        // increase volume by 5%
            KeyCode::Down => todo!(),      // decrease volume by 5%
            KeyCode::Right => todo!(),     // fast forward 5% of total song length
            KeyCode::Left => todo!(),      // dial back 5% of total song length
            KeyCode::Char('l') => todo!(), // fast forward 5s
            KeyCode::Char('h') => todo!(), // dial back 5s
            KeyCode::Char('x') => todo!(), // toggle between list, single loop and random
            // change focus
            KeyCode::Char('/') => self.focus = Focus::SearchBar,
            KeyCode::Char('e') => self.focus = Focus::Directory,
            KeyCode::Char('m') => self.focus = Focus::Main,
            KeyCode::Char('b') => self.focus = Focus::Queue,
        }
    }

    fn resolve_key_search_bar(&mut self, app: &App, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.search.push(c),
            KeyCode::Backspace => _ = self.search.pop(),
            KeyCode::Enter => {
                self.request_search();
                self.search.clear();
                self.refreshing = true;
            }
            KeyCode::Esc => self.focus = Focus::Main,
        }
    }

    fn resolve_key_directory(&mut self, app: &App, key: KeyEvent) {
        match key.code {
            // change selection
            KeyCode::Char('j') => match self.dir_focused.selected() {
                None => self.dir_focused.select(Some(0)),
                Some(u) => {
                    if u > self.music.len() - 1 {
                        self.dir_focused.select(Some(0));
                    } else {
                        self.dir_focused.select(Some(u + 1));
                    }
                }
            },
            KeyCode::Char('k') => match self.dir_focused.selected() {
                None => self.dir_focused.select(Some(0)),
                Some(u) => {
                    if u == 0 {
                        self.dir_focused.select(Some(self.music.len() - 1));
                    } else {
                        self.dir_focused.select(Some(u - 1));
                    }
                }
            },
            // control
            KeyCode::Enter => todo!(),
            // change focus
            KeyCode::Char('/') => self.focus = Focus::SearchBar,
            KeyCode::Char('e') => self.focus = Focus::Directory,
            KeyCode::Char('c') => self.focus = Focus::Controls,
        }
    }

    fn resolve_key_queue(&mut self, app: &App, key: KeyEvent) {
        match key.code {}
    }

    pub fn draw(&mut self, app: &App) {}

    fn draw_directory(&mut self) {}
    fn draw_search_bar(&mut self) {}
    fn draw_controls(&mut self) {}
    fn draw_queue(&mut self) {}
    fn draw_main(&mut self) {}

    fn request_search(&mut self) {
        self.lib_sender.send(self.search);
    }
}
