use std::sync::mpsc;
use std::{io};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use crossterm::terminal::{enable_raw_mode};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use crate::listen_event::ThreadListenEvent;
use crate::sender_event::ThreadSendEvent;

#[path = "app/resources/layout/main.rs"]
mod layout_main;
#[path = "app/resources/layout/help.rs"]
mod layout_help;

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

pub struct StateApp<T: Backend> {
    current_menu: Menu,
    terminal: Terminal<T>,
}
#[derive(Copy, Clone)]
pub enum Menu{
    Main,
    Help,
}

fn main() -> Result<(), Box<dyn Error>> {

    let (tx, rx) = mpsc::channel();
    let rx= Rc::new(rx);
    ThreadSendEvent::handle(tx);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state_app= Rc::new(
        RefCell::new(
            StateApp{
                current_menu: Menu::Main,
                terminal,
            }
        )
    );

    enable_raw_mode().expect("cant run in raw mode");
    state_app.borrow_mut().terminal.clear();


    loop {
        let current_menu= state_app.borrow_mut().current_menu;
        state_app.borrow_mut().terminal.draw(|frame| {
            match current_menu {
                Menu::Main=> {
                    let layout= layout_main::LayoutMain::new();
                    layout.render(frame);
                }
                Menu::Help=> {
                    let layout= layout_help::LayoutHelp::new();
                    layout.render(frame);
                }
            }
        })?;

        ThreadListenEvent::handle(rx.clone(), state_app.clone());
    }

}