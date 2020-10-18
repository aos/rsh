use nix::unistd;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

use std::ffi::CString;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{clear}{goto}{blue}prompt${reset}> ",
        clear = clear::All,
        goto = cursor::Goto(1, 1),
        blue = color::Fg(color::Blue),
        reset = color::Fg(color::Reset)
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut command = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Char(c) => {
                print!("{}", c);
                command.push(c);
            },
            Key::Ctrl('d') => break,
            _ => continue,
        }
        stdout.flush().unwrap();
    }

    let f = CString::new(command).expect("CString");
    let args = [CString::new("").expect("args")];
    let env = [CString::new("").expect("env")];
    unistd::execvpe(&f, &args, &env).expect("call execve");
}
