use cursive;

use cursive_spinner_view::{Frames, SpinnerView};

#[test]
fn new_default_view() {
    let siv = cursive::default();

    let spinner = SpinnerView::new(siv.cb_sink().clone());

    assert_eq!(spinner.is_spinning(), false);
    assert_eq!(spinner.spin_ups(), 0);
}

#[test]
fn frames_view() {
    const CUSTOM_FRAMES: Frames = &["1", "2", "3"];

    let siv = cursive::default();

    let mut spinner = SpinnerView::new(siv.cb_sink().clone());
    spinner.frames(CUSTOM_FRAMES);

    assert_eq!(spinner.is_spinning(), false);
}
