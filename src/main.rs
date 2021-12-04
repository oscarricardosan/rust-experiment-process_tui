use std::sync::mpsc;
use std::{io, thread};
use std::borrow::Borrow;
use std::error::Error;
use std::rc::Rc;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, ListState, Paragraph, Tabs};
use crate::listen_event::ThreadListenEvent;
use crate::sender_event::ThreadSendEvent;

#[path = "app/resources/layout/main.rs"]
mod layout_main;

#[path = "app/conifg/config_render.rs"]
mod config_render;

#[path = "app/thread/sender-event.rs"]
mod sender_event;
#[path = "app/thread/listen-event.rs"]
mod listen_event;


pub enum Event{
    Input(KeyEvent),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {

    let (tx, rx) = mpsc::channel();
    let rx= Rc::new(rx);
    ThreadSendEvent::handle(tx);

    enable_raw_mode().expect("cant run in raw mode");


    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let layout_main= layout_main::LayoutMain::new();
            layout_main.render(frame);
        })?;

        ThreadListenEvent::handle(rx.clone(), &mut terminal);
    }

    Ok(())
}