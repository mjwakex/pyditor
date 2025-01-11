use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    ExecutableCommand,
};

fn main(){
    // entering alternate screen
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    let _ = crossterm::terminal::enable_raw_mode();

    // state of editor
    let mut text = String::from("pyditor - A nano inspired python editor");
    let mut cursor_x = 0;

    // continuous loop handling rendering and user input
    loop {
        // moving the cursor and rendering text
        execute!(stdout, MoveTo(0, 0)).unwrap();
        print!("{}", text);
        stdout.flush().unwrap();

        // handling and reading user input

    }
}

