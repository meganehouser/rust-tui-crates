use cursive::*;

const MESSAGE: &str = "Merry Christmas !!";

enum State {
    HelloWorld,
    MerryChristmas,
}

struct MerryChristmasView {
    state: State,
}

impl MerryChristmasView {
    fn new() -> MerryChristmasView {
        MerryChristmasView {
            state: State::HelloWorld,
        }
    }
}

impl view::View for MerryChristmasView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        match self.state {
            State::HelloWorld => {
                printer.print((0, 0), "Hello world !!");
            }
            State::MerryChristmas => {
                let x = printer.size.x / 2 - (MESSAGE.len() / 2);
                let y = printer.size.y / 2;
                let mut style = theme::Style::default();
                style.effects.insert(theme::Effect::Bold);
                style.color = Some(theme::ColorStyle::from(theme::Color::from_256colors(12)));
                printer.with_style(style, |p| {
                    p.print((x, y), MESSAGE);
                });
            }
        };
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn on_event(&mut self, event: event::Event) -> event::EventResult {
        if event == event::Event::Char('m') {
            self.state = State::MerryChristmas;
        }

        event::EventResult::Ignored
    }
}

fn main() {
    let mut siv = Cursive::default();
    let mut theme = theme::Theme::default();
    theme
        .palette
        .set_color("view", theme::Color::TerminalDefault);
    theme
        .palette
        .set_color("foreground", theme::Color::from_256colors(15));
    theme
        .palette
        .set_color("background", theme::Color::TerminalDefault);
    siv.set_theme(theme);

    siv.add_fullscreen_layer(MerryChristmasView::new());
    siv.run();
}
