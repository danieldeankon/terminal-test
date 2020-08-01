use std::io::{stdout, Read, Write};
use termion;
use termion::async_stdin;
use termion::clear;
use termion::raw::IntoRawMode;

use std::sync::mpsc;

fn main() {
    let mut sout = stdout().into_raw_mode().unwrap();

    write!(sout, "{}", termion::screen::ToAlternateScreen).unwrap();
    write!(sout, "{}", termion::cursor::Hide).unwrap();

    for i in 0..4 {
        write!(sout, "{}", termion::cursor::Goto(1, 2)).unwrap();
        write!(sout, "Playing round {} of 4...\n", i + 1).unwrap();

        let expected_answer = get_question(&mut sout);

        let user_answer = get_answer(&mut sout); //String::from("xyz"); //

        write!(sout, "{}", termion::cursor::Goto(1, 6)).unwrap();
        if user_answer == expected_answer {
            write!(sout, "`{}` is a match\n", user_answer).unwrap();
        } else {
            write!(sout, "`{}` is not a match\n", user_answer).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(1500));
        write!(sout, "{}", termion::clear::All).unwrap();
    }

    write!(sout, "{}", termion::cursor::Show).unwrap();
    write!(sout, "{}", termion::screen::ToMainScreen).unwrap();
}

pub fn get_answer(sout: &mut termion::raw::RawTerminal<std::io::Stdout>) -> String {
    let ms = 5000;

    let (sender, receiver) = mpsc::channel::<i32>();

    let thread = std::thread::spawn(move || {
        let mut x: i32 = ms;

        sender.send(x);
        while x >= -1 {
            std::thread::sleep(std::time::Duration::from_millis(1));
            x -= 1;
            sender.send(x);
        }
    });

    let sin = async_stdin();
    let mut stdin_bytes = sin.bytes();

    let mut input_buffer = String::from("");

    let mut pressed_enter: bool = false;
    let mut pressed_exit: bool = false;

    loop {
        let i = receiver.try_recv();

        let s = match i {
            Ok(x) if x >= 0 => x,
            Ok(_) => break,
            Err(_) => continue,
        };

        let remaining = s / 1000;
        let remaining_text = match remaining {
            1 => String::from("second"),
            _ => String::from("seconds"),
        };

        let c = stdin_bytes.next();

        match c {
            Some(Ok(x)) if x >= 0x20 && x <= 0x7e => {
                input_buffer.push(x as char);
            }
            Some(Ok(x)) if x == 127 => {
                input_buffer.pop();
                write!(sout, "{}", clear::CurrentLine).unwrap();
            }
            Some(Ok(x)) if x == 13 => {
                pressed_enter = true;
                break;
            }
            Some(Ok(x)) if x == 3 => {
                pressed_exit = true;
                break;
            }
            Some(Ok(x)) => input_buffer.push_str(&format!("{}", x)),
            Some(Err(y)) => input_buffer.push_str(&format!("ERROR {}", y)),
            None => {}
        };

        write!(sout, "{}", termion::cursor::Goto(1, 4)).unwrap();

        write!(sout, "{}", clear::CurrentLine).unwrap();
        write!(sout, "{} {} left...\n", remaining, remaining_text).unwrap();

        write!(sout, "{}", termion::cursor::Goto(1, 5)).unwrap();

        write!(sout, "{}", clear::CurrentLine).unwrap();
        write!(sout, "{}█", input_buffer).unwrap();
    }

    if !pressed_enter && !pressed_exit {
        thread.join();
    }

    write!(sout, "{}", termion::cursor::Goto(1, 5)).unwrap();

    write!(sout, "{}", clear::CurrentLine).unwrap();
    write!(sout, "{}", input_buffer).unwrap();

    input_buffer
}

fn get_question(sout: &mut termion::raw::RawTerminal<std::io::Stdout>) -> String {
    write!(sout, "{}", termion::cursor::Goto(1, 3)).unwrap();
    write!(sout, "Please type `xyz`.").unwrap();
    return String::from("xyz");
}
