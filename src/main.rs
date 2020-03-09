extern crate dirs;
extern crate pancurses;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use pancurses::*;

fn main() {
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
    window.addstr("Welcome to Task List v1.0.0. Press any key to mark a task as complete.\n");
    window.attroff(COLOR_PAIR(1));
    window.attroff(A_BOLD);
    window.refresh();
    let path_opt = dirs::home_dir();
    let mut path = match path_opt {
        Some(path2) => path2,
        None => { endwin(); panic!("Error: Could not get home directory!"); }
    };
    path.push(".task-list");
    let display = path.display();
    window.refresh();

    if !path.exists() {
        endwin();
        panic!("{} does not exist. To add tasks, simply create the file and add lines with text.", display);
    }

    let mut file = match File::open(&path) {
        Err(why) => { endwin(); panic!("Error: couldn't open {}: {}", display, why.description()) },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => { endwin(); panic!("Error: couldn't read {}: {}", display, why.description()) },
        Ok(_) => (),
    };

    if s.lines().count() == 0 {
        endwin();
        panic!("Error: there must be at least one task in the tasks file.");
    }

    let mut current_y = 1;

    for line in s.lines() {
        window.attron(COLOR_PAIR(4));
        window.addstr("[❎] ");
        window.attroff(COLOR_PAIR(4));
        window.addstr(&format!("{}\n", line.trim_end()));
    }

    for _i in 0..s.lines().count() {
        window.getch();
        window.attron(COLOR_PAIR(3));
        window.mvaddstr(current_y, 0, "[✅] ");
        window.attroff(COLOR_PAIR(3));
        current_y+=1;
    }

    window.attron(COLOR_PAIR(3));
    window.mvaddstr(current_y, 0, "All tasks done! Press any key to exit.");
    window.attroff(COLOR_PAIR(3));
    window.getch();
    endwin();

}
