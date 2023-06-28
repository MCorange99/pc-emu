pub struct Screen {
    pub text: String,
    pub height: usize,
    pub width: usize,
    // pub cur_col: usize,
    pub cur_line: usize,
    pub cur_col: Vec<usize>
}

impl Screen {
    pub fn handle_new_line(&mut self) {
        self.cur_col[self.cur_line] = 0;
        
        if self.cur_line >= self.height {
            let tmp = self.text.clone();

            let tmp = tmp
                .split(|c| c == '\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            self.text = tmp[1..].join("\n");

        } else {
            self.cur_line += 1;
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cur_col[self.cur_line] > 0 {
            self.text.pop();
            self.cur_col[self.cur_line] -= 1;
        } else {
            self.text.pop();

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
            self.handle_new_line();
        } else
        if self.cur_col[self.cur_line] >= self.width {
            self.handle_new_line();
            self.text.push('\n');
        }

        self.text.push(c);
        self.cur_col[self.cur_line] += 1;

    }

    pub fn puts<S: Into<String>>(&mut self, s: S) {
        let s: String = s.into();
        for c in s.chars() {
            self.putc(c);
        }
    }
}