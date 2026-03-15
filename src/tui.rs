use std::io;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::*;

struct App {
    keys: Vec<String>,
    selected: usize,
    input: String,
    message: String,
}

impl App {
    fn new() -> Self {
        App {
            keys: vec![],
            selected: 0,
            input: String::new(),
            message: String::new(),
        }
    }
    fn next(&mut self){
        if !self.keys.is_empty(){
            self.selected = (self.selected + 1) % self.keys.len();
        }
    }

    fn previous(&mut self) {
        if !self.keys.is_empty() {
            if self.selected == 0 {
                self.selected = self.keys.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }
}
