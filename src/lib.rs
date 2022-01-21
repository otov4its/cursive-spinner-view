//! Spinner view for the cursive library
//!
//! # Examples
//!
//! See in `examples` folder
//! To see in action run `cargo run --example spinner`

#![deny(
    warnings,
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
    non_ascii_idents,
    unreachable_pub
)]
#![doc(test(attr(deny(warnings))))]
#![doc(html_root_url = "https://docs.rs/cursive-spinner-view/0.1.2")]

mod spinner;
mod view;

pub use view::SpinnerView;

/// Sinner frames
pub type Frames = &'static [&'static str];

/// Idling spinner frame
///
/// What to render when the spinner stops
#[derive(Debug, Clone, Copy)]
pub enum IdlingFrame {
    /// Render a custom frame
    Is(&'static str),
    /// Render the last frame
    Last,
}

/// Simple default spinner theme
pub const DEFAULT_FRAMES: Frames = &["-", "\\", "|", "/"];

/// Default idling frame is just nothing
pub const DEFAULT_IDLING_FRAME: IdlingFrame = IdlingFrame::Is("");

/// Min FPS setting
pub const MIN_FPS: usize = 10;

/// Max FPS setting
pub const MAX_FPS: usize = 30;

/// Acceleration factor
pub const ACCCEL_FACTOR: usize = 5;
