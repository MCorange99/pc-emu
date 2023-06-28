use color_eyre::Result;
use std::{io, thread, time::Duration};

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal, text::Line
};


use crossterm::{
    
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


fn wrap_lines(s: &str, width: usize) -> String {
    let mut wrapped = String::new();
    // println!("width: {width}");

    let mut i = 0;
    for c in s.chars() {
        wrapped.push(c);
        if i == width-1 {
            i = 0;
            wrapped.push('\n');
        } else {
            i+= 1;
        }
    }

    wrapped
}


// use glfw::{Action, Context, Key};
// use hadley_game::{self, keyboard::KeyPress};
fn main() -> Result<()> {
    // setup term
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut engine = hadley_game::Emulator::new((0, 0));


    loop {
        draw_tui(&mut terminal, &mut engine)?;
        // std::thread::sleep(std::time::Duration::from_millis(1000));
        // terminal.flush()?;

        let time_to_block = Duration::from_nanos(1);
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
    }
    

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


fn draw_tui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, engine: &mut hadley_game::Emulator) -> Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
        .title("Terminal")
        .borders(Borders::ALL);

        engine.set_screen_size((f.size().width - 2) as usize, (f.size().height - 2) as usize);

        engine.step().unwrap();

        let mut text = engine.get_screen(); 
        // wrap_lines(
        //     "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        //     , (f.size().width - 2) as usize
        // );

        
        text += "\n";

        let mut texts = vec![];

        for l in text.lines() {
            texts.push(Line::from(l));
        }

        // dbg!(&texts.len());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(f.size());


        f.render_widget(block, size);
        f.render_widget(Paragraph::new(texts.clone()), chunks[0]);
    })?;

    Ok(())
}


