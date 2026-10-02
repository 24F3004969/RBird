use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct AnimateAsciiImage {
    f1: Arc<Mutex<String>>,
    f2: Arc<Mutex<String>>,
    condition: Arc<AtomicBool>,
    animation_thread: Option<JoinHandle<()>>,
}

impl AnimateAsciiImage {
    pub fn new(f1: String, f2: String) -> Self {
        Self {
            f1: Arc::new(Mutex::new(f1)),
            f2: Arc::new(Mutex::new(f2)),
            condition: Arc::new(AtomicBool::new(false)),
            animation_thread: None,
        }
    }

    pub fn animate(&mut self) {
        // Prevent multiple animation threads from being started.
        if self.condition.swap(true, Ordering::SeqCst) {
            return;
        }

        let f1 = Arc::clone(&self.f1);
        let f2 = Arc::clone(&self.f2);
        let condition = Arc::clone(&self.condition);

        self.animation_thread = Some(thread::spawn(move || {
            while condition.load(Ordering::SeqCst) {
                {
                    let mut first_frame = f1
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());

                    let mut second_frame = f2
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());

                    std::mem::swap(&mut *first_frame, &mut *second_frame);
                }

                thread::sleep(Duration::from_millis(500));
            }
        }));
    }

    pub fn stop_animation(&mut self) {
        self.condition.store(false, Ordering::SeqCst);

        // Wait for the background thread to finish.
        if let Some(handle) = self.animation_thread.take() {
            let _ = handle.join();
        }
    }

    pub fn get_f1(&self) -> String {
        self.f1
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub fn is_animating(&self) -> bool {
        self.condition.load(Ordering::SeqCst)
    }
}

impl Drop for AnimateAsciiImage {
    fn drop(&mut self) {
        self.stop_animation();
    }
}