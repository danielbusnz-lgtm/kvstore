use std::io;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{ExecutableCommand, cursor};
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

    fn next(&mut self) {
        if !self.keys.is_empty() {
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

fn send_command(stream: &mut TcpStream, command: &str) -> String {
    let mut line = command.to_string();
    line.push('\n');
    stream.write_all(line.as_bytes()).unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response.trim().to_string()
}

fn fetch_keys(stream: &mut TcpStream) -> Vec<String> {
    let response = send_command(stream, "KEYS");
    if response.is_empty() || response.starts_with("ERR") {
        return vec![];
    }
    response.split(',').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect()
}

fn ui(f: &mut Frame, app: &App, value: &str){
    let vertical = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
    ]).split(f.size());
    let horizontal = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ]).split(vertical[0]);
    
    let items: Vec<ListItem> = app.keys.iter().enumerate().map(|(i, k)| {
        if i == app.selected {
            ListItem::new(format!("> {}", k)).style(Style::default().fg(Color::Yellow))
        } else {
            ListItem::new(format!("  {}", k))
        }
    }).collect();
    
    let keys_list = List::new(items).block(Block::default().borders(Borders::ALL).title("Keys"));
    let value_block =  Paragraph::new(value).block(Block::default().borders(Borders::ALL).title("Value"));
    let input_text = format!("> {}", app.input);
    let input_block = Paragraph::new(input_text.as_str()).block(Block::default().borders(Borders::ALL).title(app.message.as_str()));

    f.render_widget(keys_list, horizontal[0]);
    f.render_widget(value_block, horizontal[1]);
    f.render_widget(input_block, vertical[1]);

    f.set_cursor(
        vertical[1].x + app.input.len() as u16 + 3,
        vertical[1].y + 1,
    );
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("Could not connect to server");
    let mut app = App::new();
    app.keys = fetch_keys(&mut stream);

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    io::stdout().execute(cursor::SetCursorStyle::SteadyBlock)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    loop {
        let value = if !app.keys.is_empty() {
            send_command(&mut stream, &format!("GET {}", app.keys[app.selected]))
        } else {
            String::new()
        };

        terminal.draw(|f| ui(f, &app, &value))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }
                match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Enter => {
                        let response = send_command(&mut stream, &app.input);
                        app.message = response;
                        app.input.clear();
                        app.keys = fetch_keys(&mut stream);
                        app.selected = 0;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
