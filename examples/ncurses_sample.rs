use ncurses::*;

const MESSAGE: &str = "Merry Christmas !!";

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    init_pair(1, COLOR_BLUE, -1);
    addstr("Hello world !!");
    refresh();
    keypad(window, true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    loop {
        match getch() {
             109 => {
                let y = getmaxy(window) / 2;
                let x = getmaxx(window) / 2 - (MESSAGE.len() / 2) as i32;
                clear();
                color_set(1);
                mvaddstr(y, x, MESSAGE);
            }
            KEY_DC => break,
            _ => (),
        }
    }
    endwin();
}
