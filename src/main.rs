use std::io::{stdout, Write};
use crossterm::{
    cursor::{self, MoveTo}, event::{read, Event, KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};

fn main(){
    // entering alternate screen
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    let _ = crossterm::terminal::enable_raw_mode();

    // state of editor
    let mut text = String::from("pyditor - A nano inspired python editor\n");
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // continuous loop handling rendering and user input
    loop {
        // moving the cursor and rendering text
        execute!(stdout, MoveTo(0, 0)).unwrap();
        print!("{}", render_text_with_newlines(&text));
        stdout.flush().unwrap();

        // moving cursor to correct position
        execute!(stdout, MoveTo(cursor_x as u16, cursor_y as u16)).unwrap();

        // handling and reading user input
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Char(c) => {
                    let index = get_cursor_index(&text, cursor_x, cursor_y);
                    text.insert(index, c);
                    cursor_x += 1;
                }
                KeyCode::Backspace => {
                    if cursor_x > 0 || cursor_y > 0 {
                        let index = get_cursor_index(&text, cursor_x, cursor_y) - 1;
                        text.remove(index);

                        if cursor_x > 0 {
                            cursor_x -= 1;
                        } else {
                            cursor_y -= 1;
                            cursor_x = get_line_length(&text, cursor_y);
                        }
                    }
                }
                KeyCode::Right => {
                    if cursor_x < get_line_length(&text, cursor_y) {
                        cursor_x += 1;
                    } else if cursor_y < get_line_count(&text) - 1 {
                        cursor_y += 1;
                        cursor_x = 0;
                    }
                }
                KeyCode::Left => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    } else if cursor_y > 0 {
                        cursor_y -= 1;
                        cursor_x = get_line_length(&text, cursor_y);
                    }
                }
                KeyCode::Enter => {
                    let index = get_cursor_index(&text, cursor_x, cursor_y);
                    text.insert(index, '\n');
                    cursor_x = 0;
                    cursor_y += 1;
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

// helper function to render new lines
fn render_text_with_newlines(text: &str) -> String {
    text.replace('\n', "\r\n")
}

// get the index in the text based on cursor position
fn get_cursor_index(text: &str, cursor_x: usize, cursor_y: usize) -> usize {
    text.lines()
        .take(cursor_y)
        .map(|line| line.len() + 1) // +1 for the '\n'
        .sum::<usize>()
        + cursor_x
}

// get the number of lines in the text
fn get_line_count(text: &str) -> usize {
    text.lines().count()
}

// get the length of a specific line
fn get_line_length(text: &str, line_number: usize) -> usize {
    text.lines().nth(line_number).unwrap_or("").len()
}


