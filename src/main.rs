use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdout, Write, Read};
use crossterm::{
    cursor::MoveTo, event::{read, Event, KeyCode, KeyModifiers}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};

fn main(){

    // get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pyditor <filename>");
        return;
    }
    let filename = &args[1];

    // open the file or create it if it doesn't exist
    let mut text = String::new();
    if std::path::Path::new(filename).exists() {
        // file exists, read its content
        let mut file = File::open(filename).expect("Unable to open file");
        file.read_to_string(&mut text).expect("Unable to read file");
    } else {
        // file doesn't exist, create a new one
        println!("New file created: {}", filename);
    }

    // entering alternate screen
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    let _ = crossterm::terminal::enable_raw_mode();

    // state of editor
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
                    if event.modifiers == KeyModifiers::CONTROL && c == 's' {
                        // Ctrl+S to save
                        save_to_file(&text, filename);
                    } else {
                        // normal typing
                        let index = get_cursor_index(&text, cursor_x, cursor_y);
                        text.insert(index, c);
                        cursor_x += 1;
                    }
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
    let mut index = 0;
    for (i, line) in text.lines().enumerate() {
        if i == cursor_y {
            index += cursor_x.min(line.len()); // prevent out-of-bounds
            break;
        }
        index += line.len() + 1; // +1 for the '\n'
    }
    // ensure index is a valid character boundary
    while !text.is_char_boundary(index) && index > 0 {
        index -= 1;
    }
    index
}

// get the number of lines in the text
fn get_line_count(text: &str) -> usize {
    text.lines().count()
}

// get the length of a specific line
fn get_line_length(text: &str, line_number: usize) -> usize {
    text.lines().nth(line_number).unwrap_or("").len()
}

// save the file
fn save_to_file(text: &str, filename: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // overwrite
        .open(filename)
        .expect("Unable to open file for saving");

    file.write_all(text.as_bytes())
        .expect("Unable to write data to file");

    println!("File saved as '{}'", filename);
}


