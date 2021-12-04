use std::error::Error;
use std::io::Stdout;
use std::process::exit;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use crossterm::event;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::Event;

pub struct ThreadListenEvent {
}

impl ThreadListenEvent {
    pub fn handle(
        rx: Rc<Receiver<Event>>,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>
    ) -> Result<(), Box<dyn Error>>  {

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
                    exit(0);
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

        Ok(())
    }
}
