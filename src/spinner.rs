use std::thread::JoinHandle;
use std::time::Duration;
use std::{iter::Cycle, slice::Iter, sync::mpsc::Receiver, thread};

use cursive_core::views::TextContent;
use cursive_core::CbSink;

use crate::{
    view::{SpinnerControl, ThreadControl},
    Frames, IdlingFrame, MIN_FPS,
};

type FrameCycle = Cycle<Iter<'static, &'static str>>;

pub(crate) struct Spinner {
    frames_cycle: FrameCycle,
    idling_frame: IdlingFrame,
    duration: Duration,
    cb_sink: CbSink,
    text_content: TextContent,
    rx_spinner: Receiver<SpinnerControl>,
    rx_thread: Receiver<ThreadControl>,
}

impl Spinner {
    pub(crate) fn new(
        frames: Frames,
        idling_frame: IdlingFrame,
        cb_sink: CbSink,
        text_content: TextContent,
        rx_spinner: Receiver<SpinnerControl>,
        rx_thread: Receiver<ThreadControl>,
    ) -> Self {
        Self {
            frames_cycle: Self::cycle_iter(frames),
            idling_frame,
            duration: Duration::from_secs_f32(1.0 / MIN_FPS as f32),
            cb_sink,
            text_content,
            rx_spinner,
            rx_thread,
        }
    }

    pub(crate) fn spin_loop(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            while let Ok(thread_control) = self.rx_thread.recv() {
                match thread_control {
                    ThreadControl::Go => 'l: loop {
                        while let Ok(spinner_control) = self.rx_spinner.try_recv() {
                            match spinner_control {
                                SpinnerControl::Frames(frames) => {
                                    self.frames_cycle = Self::cycle_iter(frames)
                                }
                                SpinnerControl::Duration(dur) => self.duration = dur,
                                SpinnerControl::Stop => {
                                    match self.idling_frame {
                                        IdlingFrame::Is(frame) => self.set_frame(frame),
                                        IdlingFrame::Last => {}
                                    }

                                    break 'l;
                                }
                            }
                        }

                        self.set_next_frame();
                        thread::sleep(self.duration);
                    },

                    ThreadControl::Drop => break,
                }
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
