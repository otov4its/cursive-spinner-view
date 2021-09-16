use cursive::theme::Color;
use cursive::{
    traits::*,
    views::{Button, Dialog, DummyView, LinearLayout, ViewRef},
    Cursive,
};

use cursive_spinner_view::SpinnerView;

mod themes;
use themes::{DEFAULT, DOTS, GLOBE};

fn main() {
    let mut siv = cursive::default();

    let spinner = SpinnerView::new(siv.cb_sink().clone());
    let status_spinner = spinner.with_name("spinner");

    let spin_buttons = LinearLayout::vertical()
        .child(Button::new("spin up", spin_up))
        .child(Button::new("spin down", spin_down))
        .child(Button::new("stop", stop));

    let theme_buttons = LinearLayout::vertical()
        .child(Button::new("default theme", set_default_theme))
        .child(Button::new("dots theme", set_dots_theme))
        .child(Button::new("Wow! theme", set_wow_theme));

    let buttons = LinearLayout::horizontal()
        .child(spin_buttons)
        .child(DummyView.fixed_width(2))
        .child(theme_buttons);

    let main_layout = LinearLayout::horizontal()
        .child(status_spinner)
        .child(DummyView.fixed_width(5))
        .child(buttons);

    siv.add_layer(
        Dialog::around(main_layout)
            .button("Quit", Cursive::quit)
            .padding_lrtb(5, 1, 1, 1),
    );

    siv.run();
}

fn spin_up(siv: &mut Cursive) {
    get_spinner(siv).spin_up();
}

fn spin_down(siv: &mut Cursive) {
    get_spinner(siv).spin_down();
}

fn stop(siv: &mut Cursive) {
    get_spinner(siv).stop();
}

fn set_default_theme(siv: &mut Cursive) {
    get_spinner(siv)
        .frames(DEFAULT)
        .style(Color::parse("black").unwrap());
}

fn set_dots_theme(siv: &mut Cursive) {
    get_spinner(siv)
        .frames(DOTS)
        .style(Color::parse("black").unwrap());
}

fn set_wow_theme(siv: &mut Cursive) {
    get_spinner(siv)
        .frames(GLOBE)
        .style(Color::parse("blue").unwrap());
}

fn get_spinner(siv: &mut Cursive) -> ViewRef<SpinnerView> {
    siv.find_name::<SpinnerView>("spinner").unwrap()
}
