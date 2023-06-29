pub mod keyboard;
pub mod screen;
pub mod runner;
pub mod libs;

use runner::Runner;
use screen::Screen;
use color_eyre::Result;
use keyboard::{KeyCode, KeyAction};





pub struct Emulator {
    pub screen: Screen,
    pub runner: Runner,
}

impl Emulator {
    pub fn new(screen_size: (usize, usize)) -> Self {
        let mut s = Self {
            screen: Screen{
                height: screen_size.1,
                width: screen_size.0,
                cur_col: vec![0; 100],
                cur_line: 0
            },
            runner: runner::Runner::new()
        };

        
        s.screen.init().unwrap();
        s
    }

    pub fn init(&mut self) {
        self.runner.shell.start();
    }


    pub fn step(&mut self) -> Result<()> {
        self.runner.shell.step(&mut self.screen)?;

        Ok(())
    }


    pub fn send_keypress(&mut self, key: keyboard::KeyPress) {
        match key.clone() {
            keyboard::KeyPress{ code, action } => {
                match action {
                    KeyAction::Press |
                    KeyAction::Repeat => {

                        // simple_event!(KeyboardEvent, keyboard::KeyPress, key);
                        self.runner.shell.kb_event(key);
                        match code {
                            KeyCode::Backspace => self.screen.handle_backspace(),
                            KeyCode::Enter => self.screen.handle_new_line(true),
                            KeyCode::Left => (), // self.screen.handle_cur_left(),
                            KeyCode::Right => (), // self.screen.handle_cur_right(),
                            KeyCode::Up => (), // self.screen.handle_cur_up(),
                            KeyCode::Down => (), // self.screen.handle_cur_down(),
                            KeyCode::Home => (), // self.screen.handle_cur_left_max(),
                            KeyCode::End => (), // self.screen.handle_cur_right_max(),
                            KeyCode::PageUp => (),
                            KeyCode::PageDown => (),
                            KeyCode::Tab => (),
                            KeyCode::BackTab => todo!(),
                            KeyCode::Delete => (),
                            KeyCode::Insert => (),
                            KeyCode::F(_) => (),
                            KeyCode::Char(c) => self.screen.putc(c),
                            KeyCode::Null => (),
                            KeyCode::Esc => (),
                            KeyCode::CapsLock => (),
                            KeyCode::ScrollLock => (),
                            KeyCode::NumLock => (),
                            KeyCode::PrintScreen => (),
                            KeyCode::Pause => (),
                            KeyCode::Menu => (),
                            KeyCode::KeypadBegin => (),
                            KeyCode::Media(_) => (),
                            KeyCode::Modifier(_) => (),
                        }
                    }
                    KeyAction::Release => (),
                }
            }
        }

        // self.screen.puts(format!("{:?}\n", key))
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.screen.height = height;
        self.screen.width = width;
    }
}
