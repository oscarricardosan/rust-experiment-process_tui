use std::sync::mpsc;
use std::{io, thread};
use std::error::Error;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, ListState, Paragraph, Tabs};

#[path = "app/resources/layout/main.rs"]
mod layout_main;

#[path = "app/conifg/config_render.rs"]
mod config_render;


enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CrosstermEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let layout_main= layout_main::LayoutMain::new();
            layout_main.render(frame);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event {
                event::KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                }=> {
                    panic!("mori");
                }
                event::KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear();
                    break;
                }
                event::KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    panic!("mori");
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}