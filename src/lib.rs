pub mod keyboard;
pub mod screen;
pub mod shell;

use screen::Screen;
use color_eyre::Result;
use keyboard::KeyCode;





pub struct Emulator {
    screen: Screen
    
}

impl Emulator {
    pub fn new(screen_size: (usize, usize)) -> Self {
        Self {
            screen: Screen{
                text: String::with_capacity(screen_size.0 * screen_size.1),
                height: screen_size.1,
                width: screen_size.0,
                cur_col: vec![0; 100],
                cur_line: 0
            },
        }
    }


    pub fn get_screen(&mut self) -> String {
        self.screen.text.clone()
    }

    pub fn step(&mut self) -> Result<()> {
        // self.puts("a");
        Ok(())
    }


    pub fn send_keypress(&mut self, key: keyboard::KeyPress) {
        match key {
            keyboard::KeyPress{ code, ..} => {
                match code {
                    KeyCode::Backspace => self.screen.handle_backspace(),
                    KeyCode::Enter => self.screen.handle_new_line(),
                    KeyCode::Left => self.screen.handle_cur_left(),
                    KeyCode::Right => self.screen.handle_cur_right(),
                    KeyCode::Up => self.screen.handle_cur_up(),
                    KeyCode::Down => self.screen.handle_cur_down(),
                    KeyCode::Home => self.screen.handle_cur_left_max(),
                    KeyCode::End => self.screen.handle_cur_right_max(),
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
        }

        // self.screen.puts(format!("{:?}\n", key))
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.screen.height = height;
        self.screen.width = width;
    }
}
