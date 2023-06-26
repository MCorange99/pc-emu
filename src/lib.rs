pub mod keyboard;

use color_eyre::Result;

pub const SCREEN_SIZE: usize = 60*80;


struct Screen {
    text: String,
    height: usize,
    width: usize,
    cur_col: usize
}

pub struct Emulator {
    screen: Screen
    
}

impl Emulator {
    pub fn new(screen_size: (usize, usize)) -> Self {
        Self {
            screen: Screen{
                text: String::with_capacity(SCREEN_SIZE),
                height: screen_size.1,
                width: screen_size.0,
                cur_col: 0

            },
        }
    }


    pub fn get_screen(&mut self) -> String {
        self.screen.text.clone()
    }

    pub fn step(&mut self) -> Result<()> {
        self.puts("a");
        Ok(())
    }


    pub fn putc(&mut self, c: char) {

        if self.screen.cur_col >= self.screen.width {
            self.screen.text.push('\n');
            self.screen.cur_col = 0;

        }

        self.screen.text.push(c);
        self.screen.cur_col += 1;

    }

    pub fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    pub fn send_keypress(&mut self, key: keyboard::KeyPress) {
        // println!("{key:?}");
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.screen.height = height;
        self.screen.width = width;
    }
}
