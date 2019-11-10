pub use pancurses;
pub use ncurses;
pub use termbox;
pub use rustbox;
pub use termion;
pub use crossterm;
pub use cursive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
