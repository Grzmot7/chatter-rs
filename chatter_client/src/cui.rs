
#[allow(dead_code)]

use crate::events::{Event, Events};
use crate::requests;
use std::{error::Error, io};
use std::sync::{ Mutex, Arc };
use std::sync::atomic::{ AtomicBool, Ordering };
use std::time::Duration;
use tokio::task;

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<(String, String)>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

pub async fn chatting(c_id: u64, user: u64) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let mut events = Events::new().await;

    // Create default app state
    let mut app = App::default();


    // Message retrieval task
    let mess: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let mess1 = mess.clone();
    let c_id_1 = c_id.clone();
    
    task::spawn( async move { loop {

        mess1.lock().unwrap().drain(..);

        let messages = requests::get_messages(c_id_1).await;
        
        let messages = match messages {
            Ok(m) => m,
            Err(_) => panic!("Thread failed."),
        };

        for m in messages {
            mess1.lock().unwrap().push(m);
        }

        async_std::task::sleep(Duration::from_millis(500)).await;

    }});

    // Message post task
    let switch = Arc::new(AtomicBool::new(false));
    let switch_1 = switch.clone();


    let c_id_2 = c_id.clone();
    let author = user.clone();
    let post: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let post_1 = post.clone();

    task::spawn( async move { loop {
        if switch_1.load(Ordering::Relaxed) == true {
            switch_1.store(false, Ordering::Relaxed);
            let message = post_1.lock().unwrap().to_string();

            requests::post_message(message, author, c_id_2).await;

            post_1.lock().unwrap().drain(..);
        }

        async_std::task::sleep(Duration::from_millis(125)).await;
    }});
    
    loop {
        // Draw UI
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let (msg, style) = match app.input_mode {
                InputMode::Normal => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing."),
                    ],
                    Style::default().add_modifier(Modifier::RAPID_BLINK),
                ),
                InputMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to record the message"),
                    ],
                    Style::default(),
                ),
            };
            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[1]);

            let input = Paragraph::new(app.input.as_ref())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[2]);
            match app.input_mode {
                InputMode::Normal =>
                    // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {},

                InputMode::Editing => {
                    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[2].x + app.input.width() as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[2].y + 1,
                    )
                },
            }
            
            let mut chat_messages: Vec<(String, String)> = Vec::new();

            for message in mess.lock().unwrap().iter() {
                chat_messages.push((message.0.clone(), message.1.clone()));
            };

            let messages: Vec<ListItem> = 
                chat_messages
                .iter()
                .rev()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                    ListItem::new(content)
                })
                .collect();
            let messages =
                List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages, chunks[0]);
        })?;


        // Handle input
        if let Event::Input(input) = events.next().await? {
            match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app.input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    }
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        let m = app.input.drain(..).collect::<String>();
                        post.lock().unwrap().push_str(&m.as_str());
                        switch.store(true, Ordering::Relaxed);
                    }
                    Key::Char(c) => {
                        app.input.push(c);
                    }
                    Key::Backspace => {
                        app.input.pop();
                    }
                    Key::Esc => {
                        app.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
