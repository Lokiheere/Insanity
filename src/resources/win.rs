extern crate glfw;

use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events
        }
    }

    pub fn run(&mut self) {
        while !self.window_handle.should_close() {
            // Swap front and back buffers
            self.window_handle.swap_buffers();

            // Poll for and process events
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window_handle.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }

}


