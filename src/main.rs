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
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Char(c) => {
                    text.insert(cursor_x, c);
                    cursor_x += 1;
                }
                KeyCode::Backspace => {
                    if cursor_x > 0 {
                        text.remove(cursor_x - 1);
                        cursor_x -= 1;
                    }
                }
                KeyCode::Right => {
                    if cursor_x < text.len() {
                        cursor_x += 1;
                    }
                }
                KeyCode::Left => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                }
                // exit the editor on ESC key
                KeyCode::Esc => break, 
                _ => {}
            }
        }
        
    }

    // exiting editor back to terminal
    execute!(stdout, LeaveAlternateScreen).unwrap();
    let _ = crossterm::terminal::disable_raw_mode();
}

