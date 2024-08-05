use std::sync::mpsc;
use std::{sync::mpsc::Sender, time::Duration};

#[cfg(test)]
use std::thread::JoinHandle;

use cursive_core::theme::StyleType;
use cursive_core::views::TextView;
use cursive_core::CbSink;
use cursive_core::{views::TextContent, Printer, Vec2, View};

use crate::{
    spinner::Spinner, Frames, ACCCEL_FACTOR, DEFAULT_FRAMES, DEFAULT_IDLING_FRAME, MAX_FPS, MIN_FPS,
};

pub(crate) enum SpinnerControl {
    Frames(Frames),
    Duration(Duration),
    Drop,
}

/// Spinner view
#[allow(missing_debug_implementations)]
pub struct SpinnerView {
    spin_ups: usize,
    speeds: bool,
    text_view: TextView,
    tx_spinner: Sender<SpinnerControl>,

    #[cfg(test)]
    // Used in tests to prove that the spinner
    // thread terminates after dropping the view
    join_handle: Option<JoinHandle<()>>,
}

//todo? create a builder struct for SpinnerView
impl SpinnerView {
    /// New spinner view
    ///
    /// A CbSink is needed for new spinner.
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_spinner_view::SpinnerView;
    ///
    /// let siv = cursive::default();
    /// #[allow(unused)]
    /// let spinner = SpinnerView::new(siv.cb_sink().clone());
    /// ```
    pub fn new(cb_sink: CbSink) -> Self {
        let content = TextContent::new("");
        let text_view = TextView::new_with_content(content.clone()).no_wrap();

        let (tx_spinner, rx_spinner) = mpsc::channel();

        let spinner = Spinner::new(
            DEFAULT_FRAMES,
            DEFAULT_IDLING_FRAME,
            cb_sink.clone(),
            content,
            rx_spinner,
        );

        let _join_handle = spinner.spin_loop();

        SpinnerView {
            spin_ups: 0,
            speeds: true, //todo? create kinda GearBox instead speeds
            text_view,
            tx_spinner,

            #[cfg(test)]
            join_handle: Some(_join_handle),
        }
    }

    /// Spin up the spinner
    ///
    /// You can do it as many times as you need.
    pub fn spin_up(&mut self) {
        self.spin_ups = self.spin_ups.saturating_add(1);

        self.recalc_duration();
    }

    /// Spin down the spinner
    ///
    /// To stop the spinner the numbers of spin-downs
    /// have to be equal the numbers of spin-ups.
    pub fn spin_down(&mut self) {
        self.spin_ups = self.spin_ups.saturating_sub(1);

        self.recalc_duration();
    }

    /// Stop the spinner immediately
    pub fn stop(&mut self) {
        self.spin_ups = 0;
        self.recalc_duration();
    }

    /// The number of spin-ups
    pub fn spin_ups(&self) -> usize {
        self.spin_ups
    }

    /// Is the spinner spinning
    pub fn is_spinning(&self) -> bool {
        self.spin_ups() != 0
    }

    /// Set spinner's frames
    pub fn frames(&mut self, frames: Frames) -> &mut Self {
        self.tx_spinner
            .send(SpinnerControl::Frames(frames))
            .unwrap();

        self
    }

    /// Set spinner's style
    pub fn style<S: Into<StyleType>>(&mut self, style: S) -> &mut Self {
        self.text_view.set_style(style);
        self
    }

    fn recalc_duration(&self) {
        let dur = match self.spin_ups() {
            0 => Duration::ZERO,
            spin_ups => Duration::from_secs_f32(1.0 / Self::fps(spin_ups, self.speeds) as f32),
        };
        self.tx_spinner.send(SpinnerControl::Duration(dur)).unwrap();
    }

    fn fps(spin_ups: usize, speeds: bool) -> usize {
        if !speeds || spin_ups == 0 {
            return MIN_FPS;
        }

        let fps = MIN_FPS.saturating_add(ACCCEL_FACTOR.saturating_mul(spin_ups - 1));

        match fps {
            fps if fps < MIN_FPS as usize => MIN_FPS,
            fps if fps > MAX_FPS as usize => MAX_FPS,
            _ => fps,
        }
    }

    #[cfg(test)]
    #[must_use]
    fn join_handle(&mut self) -> JoinHandle<()> {
        self.join_handle.take().unwrap()
    }
}

impl Drop for SpinnerView {
    fn drop(&mut self) {
        let _ = self.tx_spinner.send(SpinnerControl::Drop);
    }
}

impl View for SpinnerView {
    fn draw(&self, printer: &Printer) {
        self.text_view.draw(printer)
    }

    fn needs_relayout(&self) -> bool {
        self.text_view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.text_view.required_size(constraint)
    }

    fn layout(&mut self, size: Vec2) {
        self.text_view.layout(size)
    }
}

#[cursive_core::blueprint(SpinnerView::new(cb_sink.into_inner()))]
struct Blueprint {
    cb_sink: cursive_core::builder::NoConfig<CbSink>,

    #[blueprint(setter=style)]
    style: Option<StyleType>,
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use cursive;
    use ntest::timeout;

    use super::*;

    #[test]
    #[timeout(1000)]
    fn drop_running_thread() {
        let siv = cursive::default();
        let mut spinner = SpinnerView::new(siv.cb_sink().clone());

        spinner.spin_up();

        thread::sleep(Duration::from_millis(10));

        let handle = spinner.join_handle();
        drop(spinner);
        drop(siv);

        assert!(matches!(handle.join(), Ok(())));
    }

    #[test]
    #[timeout(1000)]
    fn drop_sleeping_thread() {
        let siv = cursive::default();

        let mut spinner = SpinnerView::new(siv.cb_sink().clone());

        thread::sleep(Duration::from_millis(10));

        let handle = spinner.join_handle();

        drop(spinner);
        drop(siv);

        assert!(matches!(handle.join(), Ok(())));
    }
}
