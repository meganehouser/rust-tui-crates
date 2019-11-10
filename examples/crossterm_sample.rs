use crossterm::*;
use std::io::{stdout, Write};
use crossterm::screen::RawScreen;

const MESSAGE: &str = "Merry Christmas !!";

fn main() {
    let _screen = RawScreen::into_raw_mode().unwrap();
    let input = input::input();
    let mut sync_stdin = input.read_sync();

    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();

    execute!(
        stdout(),
        style::SetForegroundColor(style::Color::White),
        cursor::MoveTo(0, 0),
        Output("Hello world !!"),
        style::ResetColor
    ).unwrap();

    let (width, height) = terminal::size().unwrap();
    let x = width / 2 - (MESSAGE.len() / 2) as u16;
    let y = height / 2;

    loop {
        let event = sync_stdin.next();
        match event {
            Some(input::InputEvent::Keyboard(k)) => {
                match k {
                    input::KeyEvent::Char('m') => {
                        let styled = style::style("Merry Christmas !!")
                            .with(style::Color::Blue)
                            .attribute(style::Attribute::Bold);
                        execute!(
                                stdout(),
                                terminal::Clear(terminal::ClearType::All),
                                cursor::MoveTo(x, y),
                                style::PrintStyledContent(styled)
                            ).unwrap();
                    },
                    input::KeyEvent::Ctrl('c') => break,
                    _ => {},
                }
            },
            _ => {},
        }
    }

    execute!(
        stdout(),
        cursor::Show
    ).unwrap();
}
