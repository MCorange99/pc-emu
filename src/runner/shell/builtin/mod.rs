
use color_eyre::Result;

use crate::screen::Screen;

mod exit;
mod echo;


pub fn run_command(s: &mut Screen, args: Vec<String>) -> Result<(bool, usize)> {
    let com_name = &args[0];


    let ret = match com_name.as_str() {
        "exit" => (true, exit::exec(s, args)?),
        "echo" => (true, echo::exec(s, args)?),
        _ => (false, 0)
    };
    Ok(ret)
}