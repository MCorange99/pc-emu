use color_eyre::Result;

#[derive(Debug, Clone)]
pub struct Screen {
    pub height: usize,
    pub width: usize,
    // pub cur_col: usize,
    pub cur_line: usize,
    pub cur_col: Vec<usize>
}

impl Screen {

    pub fn init(&mut self) -> Result<()>{

        for i in unsafe{crate::runner::get_prog_mem()}[0..0x12c0].iter_mut() {
            *i = ' ' as u8;
        }

        unsafe{crate::runner::get_prog_mem()[121] = '\n' as u8;}
        Ok(())
    }

    pub fn get_screen_text(&mut self) -> String {
        let chars = unsafe{&crate::runner::get_prog_mem()[0..0x12c0]};

        unsafe {String::from_utf8_unchecked(chars.to_vec())}
    }

    pub fn set_screen_text(&mut self, mut s: String) {
        let chars: &mut [u8] = unsafe{&mut crate::runner::get_prog_mem()[0..0x12c0]};
        let s_chars = unsafe {&*s.as_mut_vec().as_slice()};
            
        // if chars.len() != s_chars.len() {
        //     panic!("string len is not the same as screen buffer len");
        // }


        // chars.copy_from_slice(s_chars);
        
        for (i, c) in s_chars.iter().enumerate() {
            chars[i] = *c;
        }
    }

    pub fn handle_new_line(&mut self, add_new_line: bool) {
        
        if self.cur_line >= self.height {
            let tmp = self.get_screen_text().clone();

            let tmp = tmp
                .split(|c| c == '\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            self.set_screen_text(tmp[1..].join("\n"));

        } else {
            self.cur_line += 1;
        }
        self.cur_col[self.cur_line] = 0;

        if add_new_line {
            unsafe {crate::runner::get_prog_mem()[120 * self.cur_line + self.cur_col[self.cur_line]] = b'\n';};
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cur_col[self.cur_line] > 0 {
            self.delete_char_from_line(self.cur_line);
            self.cur_col[self.cur_line] -= 1;
        } else {
            if self.cur_line > 0 {
                self.delete_char_from_line(self.cur_line);
                self.cur_line -= 1;
            }
        }

    }

    fn delete_char_from_line(&mut self, line: usize) {
        // let mut lines: Vec<String> = self.get_screen_text().lines().map(|a|a.to_string()).collect();
        // lines[line].pop();
        // self.set_screen_text(lines.join(""));
        unsafe {
            crate::runner::get_prog_mem()[
                120*line + self.cur_col[self.cur_line] - 1
            ] = b' ';
        }

    }

    pub fn handle_cur_up(&mut self) {
        if self.cur_line > 0 {
            self.cur_line -= 1;
        }
    }

    pub fn handle_cur_down(&mut self) {
        if self.cur_line < self.height {
            self.cur_line += 1;
        }
    }

    pub fn handle_cur_left(&mut self) {
        if self.cur_col[self.cur_line] > 0 {
            self.cur_col[self.cur_line]-= 1;
        }
    }

    pub fn handle_cur_left_max(&mut self) {
        self.cur_col[self.cur_line] = 0;
    }

    pub fn handle_cur_right(&mut self) {
        if self.cur_col[self.cur_line] < self.height {
            self.cur_col[self.cur_line] += 1;
        }
    }
    pub fn handle_cur_right_max(&mut self) {
        self.cur_col[self.cur_line] = self.height-1;
    }




    pub fn putc(&mut self, c: char) {

        if c == '\n' {
            self.handle_new_line(false);
        } else
        if self.cur_col[self.cur_line] >= self.width {
            self.handle_new_line(true);
        }
        unsafe {crate::runner::get_prog_mem()[120 * self.cur_line + self.cur_col[self.cur_line]] = c as u8;};

        self.cur_col[self.cur_line] += 1;

    }

    pub fn puts(&mut self, s: String) {
        for c in s.chars() {
            self.putc(c);
        }
    }


    // pub fn read_screen() -> String {
    // }

}

impl std::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.puts(s.to_string());
        Ok(())
    }
}