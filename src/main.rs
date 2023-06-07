mod app;
mod painter;
mod library;
mod player;

use app::App;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut app = App::new();
    app.run()
}
