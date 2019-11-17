use cursive::*;

const MESSAGE: &str = "Merry Christmas !!";

enum State {
    HelloWorld,
    MerryChristmas,
}

struct StateContainer {
    state: State,
}

impl StateContainer {
    fn new() -> StateContainer{
        StateContainer { state: State::HelloWorld }
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

    let mut state = StateContainer::new();
    let mut canvas = views::Canvas::new(state);
    canvas.set_required_size(|_, constraint| {constraint});
    canvas.set_on_event(|state, event| {
        if event == event::Event::Char('m') {
            state.state = State::MerryChristmas;
        }
        event::EventResult::Ignored
    });
    canvas.set_draw(|state, printer| {
        match state.state {
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
    });
    siv.add_fullscreen_layer(canvas);
    siv.run();
}
