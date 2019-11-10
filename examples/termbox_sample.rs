use termbox::*;


const MESSAGE: &str = "Merry Christmas !!";

fn main() {
    let mut tb = Termbox::open().unwrap();
    tb.set_clear_attributes(DEFAULT, DEFAULT);
    tb.clear();
    tb.put_str(0, 0, "Hello world !!", WHITE, DEFAULT);
    tb.present();

    loop {
        match tb.poll_event() {
            Event::Key(event) => {
                if event.key == KEY_CTRL_C {
                    break
                }
                if event.ch == Some('m') {
                    let x = tb.width() / 2 - (MESSAGE.len() / 2) as i32;
                    let y = tb.height() / 2;
                    tb.clear();
                    tb.put_str(x, y, MESSAGE, BLUE | BOLD, DEFAULT);
                    tb.present();
                };
            },
            _ => {},
        }
    }
}

