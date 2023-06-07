use crate::library::Library;
use crate::painter::Painter;
use crate::player::Player;

use std::{io, sync::mpsc, thread};

pub struct App<'a> {
    pub painter: Painter<'a>,
    pub library: Library,
    pub player: Player,
}

impl App<'_> {
    pub fn new() -> Self {
        let (painter_to_lib, lib_from_painter) = mpsc::channel();
        let (lib_to_painter, painter_from_lib) = mpsc::channel();
        App {
            painter: Painter::new(painter_to_lib, painter_from_lib),
            library: Library::new(lib_to_painter, lib_from_painter),
            player: Player::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        thread::spawn(|| {
            self.library.init();
            self.player.init();
        });
        loop {
            self.painter.draw(&self);
            self.painter.resolve_key(&self);
            if self.painter.told_me_to_exit {
                break;
            }
        }
        self.library.exit();
        self.player.exit();
        self.painter.exit();
        Ok(())
    }
}
