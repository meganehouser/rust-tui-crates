use rustbox::*;

const MESSAGE: &str = "Merry Christmas !!";

fn main() {
    let rustbox = RustBox::init(Default::default()).unwrap();
    rustbox.clear();
    rustbox.print(0, 0, rustbox::RB_NORMAL, Color::White, Color::Default,  "Hello world !!");
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('m') => {
                        let x = rustbox.width() / 2 - (MESSAGE.len() / 2);
                        let y = rustbox.height() / 2;
                        rustbox.clear();
                        rustbox.print(x, y, rustbox::RB_BOLD, Color::Blue, Color::Default, MESSAGE);
                        rustbox.present();
                    },
                    Key::Ctrl('c') => break,
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

