
use color_eyre::Result;

use crate::screen::Screen;

use super::Shell;

mod exit;
mod echo;
mod pwd;
mod host_pwd;
mod help;
mod cd;
mod ls;
mod mkdir;
mod touch;
mod rm;
mod hsc;


pub fn run_command(s: &mut Screen, sh: &mut Shell, args: Vec<String>) -> Result<(bool, usize)> {
    let com_name = &args[0];


    let ret = match com_name.as_str() {
        "exit" => (true, exit::exec(s, args)?),
        "echo" => (true, echo::exec(s, args)?),
        "pwd"  => (true, pwd::exec(s, sh, args)?),
        "hpwd" => (true, host_pwd::exec(s, sh, args)?),
        "help" => (true, help::exec(s, sh, args)?),
        "cd" => (true, cd::exec(s, sh, args)?),
        "ls" => (true, ls::exec(s, sh, args)?),
        "mkdir" => (true, mkdir::exec(s, sh, args)?),
        "touch" => (true, touch::exec(s, sh, args)?),
        "rm" => (true, rm::exec(s, sh, args)?),
        "hsc" => (true, hsc::exec(s, sh, args)?),
        _ => (false, 0)
    };
    Ok(ret)
}