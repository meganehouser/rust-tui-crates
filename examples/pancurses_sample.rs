use pancurses::*;

const message: &str = "Merry Christmas !!";

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    init_pair(1, COLOR_BLUE, -1);
    window.printw("Hello world !!");
    window.refresh();
    window.keypad(true);
    noecho();
    curs_set(0);

    loop {
        match window.getch() {
            Some(Input::Character('m')) => {
                let y = window.get_max_y() / 2;
                let x = window.get_max_x() / 2 - (message.len() / 2) as i32;
                window.clear();
                window.color_set(1);
                window.mvaddstr(y, x, message);
            },
            Some(Input::KeyDC) => break,
            _ => (),
        }
    }
    endwin();
}
