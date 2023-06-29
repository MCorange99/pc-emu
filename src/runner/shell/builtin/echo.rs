use std::fmt::Write;


pub fn exec(screen: &mut crate::screen::Screen, argv: Vec<String>) -> color_eyre::Result<usize> {
    if argv.len() < 2 {
        writeln!(screen, "")?;

    } else {
        writeln!(screen, "{}", argv[1])?;
    }

    Ok(0)
}