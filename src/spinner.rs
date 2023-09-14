use std::sync::mpsc::RecvTimeoutError;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{iter::Cycle, slice::Iter, sync::mpsc::Receiver, thread};

use cursive_core::views::TextContent;
use cursive_core::CbSink;

use crate::{view::SpinnerControl, Frames, IdlingFrame};

type FrameCycle = Cycle<Iter<'static, &'static str>>;

pub(crate) struct Spinner {
    frames_cycle: FrameCycle,
    idling_frame: IdlingFrame,
    duration: Duration,
    cb_sink: CbSink,
    text_content: TextContent,
    rx_spinner: Receiver<SpinnerControl>,
}

impl Spinner {
    pub(crate) fn new(
        frames: Frames,
        idling_frame: IdlingFrame,
        cb_sink: CbSink,
        text_content: TextContent,
        rx_spinner: Receiver<SpinnerControl>,
    ) -> Self {
        Self {
            frames_cycle: Self::cycle_iter(frames),
            idling_frame,
            duration: Duration::ZERO,
            cb_sink,
            text_content,
            rx_spinner,
        }
    }

    pub(crate) fn spin_loop(mut self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            let spinner_control = if self.duration.is_zero() {
                Some(self.rx_spinner.recv().unwrap())
            } else {
                match self.rx_spinner.recv_timeout(self.duration) {
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(e) => panic!("{:?}", e),
                    Ok(some_ctrl) => Some(some_ctrl),
                }
            };
            match spinner_control {
                Some(SpinnerControl::Drop) => {
                    break;
                }
                Some(SpinnerControl::Frames(frames)) => {
                    self.frames_cycle = Self::cycle_iter(frames)
                }
                Some(SpinnerControl::Duration(dur)) => {
                    self.duration = dur;
                    if self.duration.is_zero() {
                        match self.idling_frame {
                            IdlingFrame::Is(frame) => self.set_frame(frame),
                            IdlingFrame::Last => {}
                        }
                    }
                }
                _ => {}
            }
            if !self.duration.is_zero() {
                self.set_next_frame();
            }
        })
    }

    fn set_frame(&self, frame: &'static str) {
        let text_content = self.text_content.clone();

        // Redraw the spinner with the new frame
        self.cb_sink
            .send(Box::new(move |_| {
                text_content.set_content(frame);
            }))
            .unwrap();
    }

    fn set_next_frame(&mut self) {
        let next_frame = self.next_frame();
        self.set_frame(next_frame);
    }

    fn next_frame(&mut self) -> &'static str {
        self.frames_cycle.next().unwrap()
    }

    fn cycle_iter(frames: Frames) -> FrameCycle {
        frames.iter().cycle()
    }
}
