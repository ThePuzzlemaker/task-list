extern crate clap;
extern crate dirs;
extern crate pancurses;

use clap::{App, Arg};
use pancurses::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("Task List")
                    .version("1.0.2")
                    .author("ThePuzzlemaker <tpzker@thepuzzlemaker.info>")
                    .about("A simple program to track tasks to do regularly. It does not save completed tasks.")
                    .arg(Arg::with_name("custom_chars")
                            .short("c")
                            .long("custom_chars")
                            .value_name("chars")
                            .help("Sets the characters for use with the task list. (Example: -c \"X√\" will use 'X' for an uncompleted task and '√' for a completed task)")
                            .takes_value(true))
                    .arg(Arg::with_name("emoji")
                            .short("e")
                            .long("emoji")
                            .takes_value(false)
                            .help("Uses emoji for the task list if this flag is present."))
                    .get_matches();

    let emoji_chars: [char; 2] = ['❌', '✅'];
    let standard_chars: [char; 2] = ['X', '√'];
    let mut chars: Vec<char> = vec![];

    if !matches.is_present("custom_chars") {
        if matches.is_present("emoji") {
            chars.push(emoji_chars[0]);
            chars.push(emoji_chars[1]);
        } else {
            chars.push(standard_chars[0]);
            chars.push(standard_chars[1]);
        }
    } else {
        let value = matches.value_of("custom_chars").unwrap_or("X√");
        if value.len() != 2 {
            panic!("Error: only two characters can be provided for custom characters.");
        } else {
            chars.push(value.chars().next().unwrap_or('X'));
            chars.push(value.chars().nth(1).unwrap_or('√'));
        }
    }

    let window = initscr();
    noecho();
    curs_set(0);

    if !has_colors() {
        endwin();
        panic!("Error: your terminal does not support color!");
    }

    start_color();
    use_default_colors();
    init_pair(1, COLOR_BLUE, -1);
    init_pair(2, COLOR_YELLOW, -1);
    init_pair(3, COLOR_GREEN, -1);
    init_pair(4, COLOR_RED, -1);

    window.attron(COLOR_PAIR(1));
    window.attron(A_BOLD);
    window.addstr("Welcome to Task List v1.0.2. Press any key to mark a task as complete.\n");
    window.attroff(COLOR_PAIR(1));
    window.attroff(A_BOLD);
    window.refresh();
    let path_opt = dirs::home_dir();
    let mut path = match path_opt {
        Some(path2) => path2,
        None => {
            endwin();
            panic!("Error: Could not get home directory!");
        }
    };
    path.push(".task-list");
    let display = path.display();
    window.refresh();

    if !path.exists() {
        endwin();
        panic!(
            "{} does not exist. To add tasks, simply create the file and add lines with text.",
            display
        );
    }

    let mut file = match File::open(&path) {
        Err(why) => {
            endwin();
            panic!("Error: couldn't open {}: {}", display, why)
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        endwin();
        panic!("Error: couldn't read {}: {}", display, why);
    }

    if s.lines().count() == 0 {
        endwin();
        panic!("Error: there must be at least one task in the tasks file.");
    }

    let mut current_y = 1;

    for line in s.lines() {
        window.attron(COLOR_PAIR(4));
        window.addstr(&format!("[{}] ", chars[0]));
        window.attroff(COLOR_PAIR(4));
        window.addstr(&format!("{}\n", line.trim_end()));
    }

    for _i in 0..s.lines().count() {
        window.getch();
        window.attron(COLOR_PAIR(3));
        window.mvaddstr(current_y, 0, &format!("[{}] ", chars[1]));
        window.attroff(COLOR_PAIR(3));
        current_y += 1;
    }

    window.attron(COLOR_PAIR(3));
    window.mvaddstr(current_y, 0, "All tasks done! Press any key to exit.");
    window.attroff(COLOR_PAIR(3));
    window.getch();
    endwin();
}
