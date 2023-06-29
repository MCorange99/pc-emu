use color_eyre::Result;
use hadley_game::runner::machine_status_bits;
use std::time::Duration;

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal, text::{Line, Span}, style::Style
};


use crossterm::{
    
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// use glfw::{Action, Context, Key};
// use hadley_game::{self, keyboard::KeyPress};
fn main() -> Result<()> {
    // setup term
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut engine = hadley_game::Emulator::new((0, 0));

    engine.init();
    loop {
        draw_tui(&mut terminal, &mut engine)?;
        // std::thread::sleep(std::time::Duration::from_millis(1000));
        // terminal.flush()?;

        let time_to_block = Duration::from_millis(10);
        let time_started = std::time::SystemTime::now();
        if crossterm::event::poll(time_to_block)? {
            match crossterm::event::read().unwrap() {
                //i think this speaks for itself
                Event::Key(KeyEvent { code, kind, ..}) => {
                    if code == KeyCode::Esc {
                        break;
                    }
                    // println!("Key Event: code: {code:?} modifiers: {modifiers:?} kind: {kind:?} state: {state:?}");
                    engine.send_keypress(hadley_game::keyboard::KeyPress{
                        code: code.into(),
                        // modifier: (modifiers.bits()).into(),
                        action: kind.into()
                    });

                },
                _ => (),
            }
            // wait the remaining time
            let time_now = std::time::SystemTime::now();
            if time_started + time_to_block > time_now {
                let time_to_wait = time_to_block - time_now.duration_since(time_started)?;
                std::thread::sleep(time_to_wait);
            }
        }
        let should_exit = unsafe {
            let status = *hadley_game::runner::MACHINE_STATUS.lock().unwrap().get();
            status & machine_status_bits::MS_SHOULD_EXIT == machine_status_bits::MS_SHOULD_EXIT
        };
        if should_exit {
            break;
        }
    }
    

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}


fn draw_tui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, engine: &mut hadley_game::Emulator) -> Result<()> {
    terminal.draw(|f| {
        // let size = f.size();
        let term_block = Block::default()
            .title("Terminal")
            .borders(Borders::ALL);
        let debug_block = Block::default()
            .title("Debug")
            .borders(Borders::ALL);

        

        
        let box_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(f.size()); 


        let debug_text = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(box_chunks[1])[0];
        let terminal_text = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(box_chunks[0])[0];

        engine.set_screen_size((terminal_text.width -1) as usize, (terminal_text.height -1) as usize);
        
        engine.step().unwrap();
        
        // wrap_lines(
            //     "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            //     , (f.size().width - 2) as usize
            // );
        
        let text = engine.screen.get_screen_text();
        
        let mut lines = vec![];
        
        for i in 0..40 {
            lines.push(text[i*120..i*120+120].to_string());
        }


        let mut texts = vec![];

        let cur_line = engine.screen.cur_line;

        for (i, l) in lines.iter().enumerate() {
            if i == cur_line {
                
                let line_pre = &l[..engine.screen.cur_col[cur_line]];
                let mut cur_char =  &l[engine.screen.cur_col[cur_line]..engine.screen.cur_col[cur_line]];
                let line_post = &l[engine.screen.cur_col[cur_line]..];


                if cur_char == "" {
                    cur_char = " "
                }
                // println!("{i} '{line_pre}' '{cur_char}' '{line_post}'");

                texts.push(Line::from(vec![
                    // Span::styled(line_pre, Style::default().fg(ratatui::style::Color::Black).bg(ratatui::style::Color::White)),
                    Span::raw(line_pre),
                    Span::styled(cur_char,Style::default().fg(ratatui::style::Color::Black).bg(ratatui::style::Color::White)),
                    Span::raw(line_post),
                ]));

                // texts.push(Line::from(format!("{}{}{}", line_pre, cur_char, line_post)));
            
            } else {
                texts.push(Line::from(l.clone()));
            }
        }

        // dbg!(&texts.len());


        f.render_widget(debug_block, box_chunks[1]);
        f.render_widget(Paragraph::new(vec![
            Line::from(Span::raw(format!("cur_pos: ({}, {})", cur_line, engine.screen.cur_col[cur_line] ))),
            Line::from(Span::raw(format!("cur_cols:"))),
            Line::from(Span::raw(format!("  0: {}\n",engine.screen.cur_col[0] ))),
            Line::from(Span::raw(format!("  1: {}\n",engine.screen.cur_col[1] ))),
            Line::from(Span::raw(format!("  2: {}\n",engine.screen.cur_col[2] ))),
            Line::from(Span::raw(format!("  3: {}\n",engine.screen.cur_col[3] ))),
            Line::from(Span::raw(format!("  4: {}\n",engine.screen.cur_col[4] ))),
            Line::from(Span::raw(format!("  5: {}\n",engine.screen.cur_col[5] ))),
            Line::from(Span::raw(format!("  6: {}\n",engine.screen.cur_col[6] ))),
            Line::from(Span::raw(format!("inp_buf: {:?}\n",engine.runner.shell.input_buf )))
        ]), debug_text);

        f.render_widget(term_block, box_chunks[0]);
        f.render_widget(Paragraph::new(texts.clone()), terminal_text);
    })?;

    Ok(())
}


