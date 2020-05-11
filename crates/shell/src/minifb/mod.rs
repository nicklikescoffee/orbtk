//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::RefCell,
    char,
    collections::HashMap,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

use derive_more::Constructor;

pub use super::native::*;

use minifb;

use crate::{prelude::*, render::*, utils::*};

use self::states::*;
mod states;

pub fn initialize() {}

fn unicode_to_key_event(uni_char: u32) -> Option<KeyEvent> {
    let mut text = String::new();

    let key = if let Some(character) = char::from_u32(uni_char) {
        text = character.to_string();
        Key::from(character)
    } else {
        Key::Unknown
    };

    if key == Key::Up
        || key == Key::Down
        || key == Key::Left
        || key == Key::Right
        || key == Key::Backspace
        || key == Key::Control
        || key == Key::Home
        || key == Key::Escape
        || key == Key::Delete
        || key == Key::Unknown
    {
        return None;
    }

    Some(KeyEvent {
        key,
        state: ButtonState::Down,
        text,
    })
}

struct KeyInputCallBack {
    key_events: Rc<RefCell<Vec<KeyEvent>>>,
}

impl minifb::InputCallback for KeyInputCallBack {
    fn add_char(&mut self, uni_char: u32) {
        if let Some(key_event) = unicode_to_key_event(uni_char) {
            self.key_events.borrow_mut().push(key_event);
        }
    }
}

struct KeyHelper(bool, minifb::Key, Key);

// /// Concrete implementation of the window shell.
// pub struct Shell<A>
// where
//     A: WindowAdapter,
// {
//     window: minifb::Window,
//     render_context_2_d: RenderContext2D,
//     adapter: A,
//     mouse_pos: (f32, f32),
//     button_down: (bool, bool, bool),
//     window_size: (usize, usize),
//     key_events: Rc<RefCell<Vec<KeyEvent>>>,
//     // todo: temp solution
//     key_backspace: KeyHelper,
//     key_delete: KeyHelper,
//     key_left: KeyHelper,
//     key_right: KeyHelper,
//     key_up: KeyHelper,
//     key_down: KeyHelper,
//     key_enter: KeyHelper,
//     key_control: KeyHelper,
//     key_control_right: KeyHelper,
//     key_shift_l: KeyHelper,
//     key_shift_r: KeyHelper,
//     key_alt: KeyHelper,
//     key_alt_r: KeyHelper,
//     key_escape: KeyHelper,
//     key_home: KeyHelper,
//     key_a: KeyHelper,
//     key_c: KeyHelper,
//     key_v: KeyHelper,
//     key_x: KeyHelper,
//     update: bool,
//     running: bool,
//     active: bool,
//     request_receiver: Receiver<ShellRequest>,
//     request_sender: Sender<ShellRequest>,
// }

// impl<A> Shell<A>
// where
//     A: WindowAdapter,
// {
//     /// Creates a new window shell with an adapter.
//     pub fn new(
//         window: minifb::Window,
//         adapter: A,
//         key_events: Rc<RefCell<Vec<KeyEvent>>>,
//     ) -> Shell<A> {
//         let size = window.get_size();
//         let render_context_2_d = RenderContext2D::new(size.0 as f64, size.1 as f64);
//         let (request_sender, request_receiver) = channel();

//         Shell {
//             window,
//             render_context_2_d,
//             adapter,
//             mouse_pos: (0.0, 0.0),
//             window_size: size,
//             button_down: (false, false, false),
//             key_events,
//             key_backspace: KeyHelper(false, minifb::Key::Backspace, Key::Backspace),
//             key_left: KeyHelper(false, minifb::Key::Left, Key::Left),
//             key_right: KeyHelper(false, minifb::Key::Right, Key::Right),
//             key_up: KeyHelper(false, minifb::Key::Up, Key::Up),
//             key_down: KeyHelper(false, minifb::Key::Down, Key::Down),
//             key_delete: KeyHelper(false, minifb::Key::Delete, Key::Delete),
//             key_enter: KeyHelper(false, minifb::Key::Enter, Key::Enter),
//             key_control: KeyHelper(false, minifb::Key::LeftCtrl, Key::Control),
//             key_control_right: KeyHelper(false, minifb::Key::RightCtrl, Key::Control),
//             key_shift_l: KeyHelper(false, minifb::Key::LeftShift, Key::ShiftL),
//             key_shift_r: KeyHelper(false, minifb::Key::RightShift, Key::ShiftR),
//             key_alt: KeyHelper(false, minifb::Key::LeftAlt, Key::Alt),
//             key_alt_r: KeyHelper(false, minifb::Key::RightAlt, Key::Alt),
//             key_escape: KeyHelper(false, minifb::Key::Escape, Key::Escape),
//             key_home: KeyHelper(false, minifb::Key::Home, Key::Home),
//             key_a: KeyHelper(false, minifb::Key::A, Key::A(false)),
//             key_c: KeyHelper(false, minifb::Key::C, Key::C(false)),
//             key_v: KeyHelper(false, minifb::Key::V, Key::V(false)),
//             key_x: KeyHelper(false, minifb::Key::X, Key::X(false)),
//             running: true,
//             update: true,
//             active: false,
//             request_receiver,
//             request_sender,
//         }
//     }

//     /// Gets if the shell is running.
//     pub fn running(&self) -> bool {
//         self.running
//     }

//     /// Gets a a new sender to send request to the window shell.
//     pub fn request_sender(&self) -> Sender<ShellRequest> {
//         self.request_sender.clone()
//     }

//     /// Sets running.
//     pub fn set_running(&mut self, running: bool) {
//         self.running = running;
//     }

//     /// Get if the shell should be updated.
//     pub fn update(&self) -> bool {
//         self.update
//     }

//     /// Sets update.
//     pub fn set_update(&mut self, update: bool) {
//         self.update = update;
//     }

//     /// Gets the shell adapter.
//     pub fn adapter(&mut self) -> &mut A {
//         &mut self.adapter
//     }

//     /// Gets the render ctx 2D.
//     pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
//         &mut self.render_context_2_d
//     }

//     fn drain_events(&mut self) {
//         // mouse move
//         if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
//             if (pos.0.floor(), pos.1.floor()) != self.mouse_pos {
//                 self.adapter.mouse(pos.0 as f64, pos.1 as f64);
//                 self.mouse_pos = (pos.0.floor(), pos.1.floor());
//             }
//         }

//         // mouse
//         let left_button_down = self.window.get_mouse_down(minifb::MouseButton::Left);
//         let middle_button_down = self.window.get_mouse_down(minifb::MouseButton::Middle);
//         let right_button_down = self.window.get_mouse_down(minifb::MouseButton::Right);

//         if self.active != self.window.is_active() {
//             self.adapter.active(self.window.is_active());
//             self.active = self.window.is_active();
//         }

//         if left_button_down != self.button_down.0 {
//             if left_button_down {
//                 self.push_mouse_event(true, MouseButton::Left);
//             } else {
//                 self.push_mouse_event(false, MouseButton::Left);
//             }
//             self.button_down.0 = left_button_down;
//         }

//         if middle_button_down != self.button_down.1 {
//             if middle_button_down {
//                 self.push_mouse_event(true, MouseButton::Middle);
//             } else {
//                 self.push_mouse_event(false, MouseButton::Middle);
//             }
//             self.button_down.1 = middle_button_down;
//         }

//         if right_button_down != self.button_down.2 {
//             if right_button_down {
//                 self.push_mouse_event(true, MouseButton::Right);
//             } else {
//                 self.push_mouse_event(false, MouseButton::Right);
//             }
//             self.button_down.2 = right_button_down;
//         }

//         // scroll
//         if let Some(delta) = self.window.get_scroll_wheel() {
//             self.adapter.scroll(delta.0 as f64, delta.1 as f64);
//         }

//         // key
//         while let Some(event) = self.key_events.borrow_mut().pop() {
//             self.adapter.key_event(event);
//         }

//         key_event_helper_down(&mut self.key_backspace, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_delete, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_left, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_right, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_up, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_down, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_enter, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_control, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_control_right, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_shift_l, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_shift_r, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_alt, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_alt_r, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_escape, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_home, &mut self.adapter, &self.window);

//         key_event_helper_up(&mut self.key_backspace, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_delete, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_left, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_right, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_up, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_down, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_enter, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_control, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_control_right, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_shift_l, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_shift_r, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_alt, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_alt_r, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_escape, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_home, &mut self.adapter, &self.window);

//         key_event_helper_down(&mut self.key_a, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_c, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_v, &mut self.adapter, &self.window);
//         key_event_helper_down(&mut self.key_x, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_a, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_c, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_v, &mut self.adapter, &self.window);
//         key_event_helper_up(&mut self.key_x, &mut self.adapter, &self.window);

//         // resize
//         if self.window_size != self.window.get_size() {
//             self.window_size = self.window.get_size();
//             self.render_context_2_d
//                 .resize(self.window_size.0 as f64, self.window_size.1 as f64);
//             self.adapter
//                 .resize(self.window_size.0 as f64, self.window_size.1 as f64);
//         }

//         // receive request
//         let mut update = self.update();

//         for request in self.request_receiver.try_iter() {
//             if update {
//                 break;
//             }

//             match request {
//                 ShellRequest::Update => {
//                     update = true;
//                 }
//                 _ => {}
//             }
//         }

//         self.set_update(update);
//     }

//     fn push_mouse_event(&mut self, pressed: bool, button: MouseButton) {
//         let state = if pressed {
//             ButtonState::Down
//         } else {
//             ButtonState::Up
//         };

//         self.adapter.mouse_event(MouseEvent {
//             x: self.mouse_pos.0 as f64,
//             y: self.mouse_pos.1 as f64,
//             button,
//             state,
//         });
//     }

//     pub fn flip(&mut self) -> bool {
//         if let Some(data) = self.render_context_2_d.data() {
//             let _ = self
//                 .window
//                 .update_with_buffer(data, self.window_size.0, self.window_size.1);
//             CONSOLE.time_end("render");
//             return true;
//         }

//         false
//     }

//     pub fn run(mut self) {
//         loop {
//             if !self.running() || !self.window.is_open() {
//                 break;
//             }

//             // CONSOLE.time("complete run");
//             self.adapter.run(&mut self.render_context_2_d);
//             if self.update() {
//                 self.set_update(false);
//             }

//             if !self.flip() {
//                 self.window.update();
//             }

//             self.drain_events();
//         }
//     }
// }

// impl<A> Drop for Shell<A>
// where
//     A: WindowAdapter,
// {
//     fn drop(&mut self) {}
// }

// /// Constructs the window shell.
// pub struct ShellBuilder<A>
// where
//     A: WindowAdapter,
// {
//     title: String,
//     resizeable: bool,
//     always_on_top: bool,
//     borderless: bool,
//     bounds: Rectangle,
//     adapter: A,
// }

// impl<A> ShellBuilder<A>
// where
//     A: WindowAdapter,
// {
//     /// Create a new window builder with the given adapter.
//     pub fn new(adapter: A) -> Self {
//         ShellBuilder {
//             adapter,
//             title: String::default(),
//             borderless: false,
//             resizeable: false,
//             always_on_top: false,
//             bounds: Rectangle::default(),
//         }
//     }

//     /// Sets the title.
//     pub fn title(mut self, title: impl Into<String>) -> Self {
//         self.title = title.into();
//         self
//     }

//     /// Sets borderless.
//     pub fn borderless(mut self, borderless: bool) -> Self {
//         self.borderless = borderless;
//         self
//     }

//     /// Sets resizeable.
//     pub fn resizeable(mut self, resizeable: bool) -> Self {
//         self.resizeable = resizeable;
//         self
//     }

//     /// Sets always_on_top.
//     pub fn always_on_top(mut self, always_on_top: bool) -> Self {
//         self.always_on_top = always_on_top;
//         self
//     }

//     /// Sets the bounds.
//     pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
//         self.bounds = bounds.into();
//         self
//     }

//     /// Builds the window shell.
//     pub fn build(self) -> Shell<A> {
//         let window_options = minifb::WindowOptions {
//             resize: self.resizeable,
//             topmost: self.always_on_top,
//             borderless: self.borderless,
//             title: !self.borderless,
//             scale_mode: minifb::ScaleMode::UpperLeft,
//             ..Default::default()
//         };

//         let mut window = minifb::Window::new(
//             self.title.as_str(),
//             self.bounds.width as usize,
//             self.bounds.height as usize,
//             window_options,
//         )
//         .unwrap_or_else(|e| {
//             panic!("{}", e);
//         });

//         // Limit to max ~60 fps update rate
//         window.limit_update_rate(Some(Duration::from_micros(64000)));

//         let key_events = Rc::new(RefCell::new(vec![]));

//         window.set_input_callback(Box::new(KeyInputCallBack {
//             key_events: key_events.clone(),
//         }));

//         window.set_position(self.bounds.x as isize, self.bounds.y as isize);

//         Shell::new(window, self.adapter, key_events)
//     }
// }

/// The `WindowBuilder` is used to construct a window shell for the minifb backend.
pub struct WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    shell: &'a mut Shell<A>,
    adapter: A,
    title: String,
    resizeable: bool,
    always_on_top: bool,
    borderless: bool,
    fonts: HashMap<String, &'static [u8]>,
    bounds: Rectangle,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Registers a new font with family key.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    pub fn build(self) {
        let window_options = minifb::WindowOptions {
            resize: self.resizeable,
            topmost: self.always_on_top,
            borderless: self.borderless,
            title: !self.borderless,
            scale_mode: minifb::ScaleMode::UpperLeft,
            ..Default::default()
        };

        let mut window = minifb::Window::new(
            self.title.as_str(),
            self.bounds.width as usize,
            self.bounds.height as usize,
            window_options,
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(Duration::from_micros(64000)));

        let key_events = Rc::new(RefCell::new(vec![]));

        window.set_input_callback(Box::new(KeyInputCallBack {
            key_events: key_events.clone(),
        }));

        window.set_position(self.bounds.x as isize, self.bounds.y as isize);

        let (request_sender, request_receiver) = channel();

        let mut render_context = RenderContext2D::new(self.bounds.width, self.bounds.height);

        for (family, font) in self.fonts {
            render_context.register_font(&family, font);
        }

        self.shell.window_shells.push(Window {
            window,
            adapter: self.adapter,
            render_context,
            request_receiver,
            request_sender,
            window_state: WindowState::default(),
            mouse: MouseState::default(),
            update: true,
            redraw: true,
            close: false,
            key_states: vec![
                KeyState::new(minifb::Key::Backspace, Key::Backspace),
                KeyState::new(minifb::Key::Left, Key::Left),
                KeyState::new(minifb::Key::Right, Key::Right),
                KeyState::new(minifb::Key::Up, Key::Up),
                KeyState::new(minifb::Key::Down, Key::Down),
                KeyState::new(minifb::Key::Delete, Key::Delete),
                KeyState::new(minifb::Key::Enter, Key::Enter),
                KeyState::new(minifb::Key::LeftCtrl, Key::Control),
                KeyState::new(minifb::Key::RightCtrl, Key::Control),
                KeyState::new(minifb::Key::LeftShift, Key::ShiftL),
                KeyState::new(minifb::Key::RightShift, Key::ShiftR),
                KeyState::new(minifb::Key::LeftAlt, Key::Alt),
                KeyState::new(minifb::Key::RightAlt, Key::Alt),
                KeyState::new(minifb::Key::Escape, Key::Escape),
                KeyState::new(minifb::Key::Home, Key::Home),
                KeyState::new(minifb::Key::A, Key::A(false)),
                KeyState::new(minifb::Key::C, Key::C(false)),
                KeyState::new(minifb::Key::V, Key::V(false)),
                KeyState::new(minifb::Key::X, Key::X(false)),
            ],
        });
    }
}


/// Represents a wrapper for a minifb window. It handles events, propagate them to 
/// the window adapter and handles the update and render pipeline.
struct Window<A>
where
    A: WindowAdapter,
{
    window: minifb::Window,
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Receiver<WindowRequest>,
    request_sender: Sender<WindowRequest>,
    window_state: WindowState,
    mouse: MouseState,
    update: bool,
    redraw: bool,
    close: bool,
    key_states: Vec<KeyState>,
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    fn is_open(&self) -> bool {
        self.window.is_open() && !self.close
    }

    fn push_mouse_event(&mut self, pressed: bool, button: MouseButton) {
        let state = if pressed {
            ButtonState::Down
        } else {
            ButtonState::Up
        };

        self.adapter.mouse_event(MouseEvent {
            x: self.mouse.mouse_pos.0 as f64,
            y: self.mouse.mouse_pos.1 as f64,
            button,
            state,
        });
    }

    fn push_key_down_event(&mut self, index: usize) {
        let key_repeat = match self.key_states.get(index).unwrap().minifb_key {
            minifb::Key::Left
            | minifb::Key::Right
            | minifb::Key::Up
            | minifb::Key::Down
            | minifb::Key::Backspace
            | minifb::Key::Delete => minifb::KeyRepeat::Yes,
            _ => minifb::KeyRepeat::No,
        };

        if self.window.is_key_pressed(self.key_states.get(index).unwrap().minifb_key, key_repeat) {
            self.adapter.key_event(KeyEvent {
                key: self.key_states.get(index).unwrap().key,
                state: ButtonState::Down,
                text: String::default(),
            });
        }
    }

    fn push_key_up_event(&mut self, index: usize) {
        if self.window.is_key_released(self.key_states.get(index).unwrap().minifb_key) {
            self.adapter.key_event(KeyEvent {
                key: self.key_states.get(index).unwrap().key,
                state: ButtonState::Up,
                text: String::default(),
            });
        }
    }

    fn drain_events(&mut self) {
        self.window.update();

        // mouse move
        if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
            if (pos.0.floor(), pos.1.floor()) != self.mouse.mouse_pos {
                self.adapter.mouse(pos.0 as f64, pos.1 as f64);
                self.mouse.mouse_pos = (pos.0.floor(), pos.1.floor());
                self.update = true;
            }
        }

        // mouse buttons
        let left_button_down = self.window.get_mouse_down(minifb::MouseButton::Left);
        let middle_button_down = self.window.get_mouse_down(minifb::MouseButton::Middle);
        let right_button_down = self.window.get_mouse_down(minifb::MouseButton::Right);

        if left_button_down != self.mouse.button_left {
            if left_button_down {
                self.push_mouse_event(true, MouseButton::Left);
            } else {
                self.push_mouse_event(false, MouseButton::Left);
            }
            self.mouse.button_left = left_button_down;
            self.update = true;
        }

        if middle_button_down != self.mouse.button_middle {
            if middle_button_down {
                self.push_mouse_event(true, MouseButton::Middle);
            } else {
                self.push_mouse_event(false, MouseButton::Middle);
            }
            self.mouse.button_middle = middle_button_down;
            self.update = true;
        }

        if right_button_down != self.mouse.button_right {
            if right_button_down {
                self.push_mouse_event(true, MouseButton::Right);
            } else {
                self.push_mouse_event(false, MouseButton::Right);
            }
            self.mouse.button_right = right_button_down;
            self.update = true;
        }

        // scroll
        if let Some(delta) = self.window.get_scroll_wheel() {
            self.adapter.scroll(delta.0 as f64, delta.1 as f64);
            self.update = true;
        }

        // resize
        if self.window_state.size != self.window.get_size() {
            self.window_state.size = self.window.get_size();
            self.render_context.resize(
                self.window_state.size.0 as f64,
                self.window_state.size.1 as f64,
            );
            self.adapter.resize(
                self.window_state.size.0 as f64,
                self.window_state.size.1 as f64,
            );
            self.update = true;
        }

        if self.window_state.active != self.window.is_active() {
            self.adapter.active(self.window.is_active());
            self.window_state.active = self.window.is_active();
        }

        // keys
        for i in 0..self.key_states.len() {
            self.push_key_down_event(i);
            self.push_key_up_event(i);
        }
    }

    fn receive_requests(&mut self) {
        for request in self.request_receiver.try_iter() {
            match request {
                WindowRequest::Redraw => {
                    self.update = true;
                    self.redraw = true;
                }
                WindowRequest::ChangeTitle(title) => {
                    self.window.set_title(&title);
                }
                WindowRequest::Close => {
                    self.close = true;
                }
            }
        }
    }

    fn update(&mut self) {
        if !self.update {
            return;
        }
        self.adapter.run(&mut self.render_context);
        self.update = false;
        self.redraw = true;
    }

    fn render(&mut self) {
        if self.redraw {
            if let Some(data) = self.render_context.data() {
                let _ = self.window.update_with_buffer(
                    data,
                    self.window_state.size.0 as usize,
                    self.window_state.size.1 as usize,
                );
                CONSOLE.time_end("render");
                self.redraw = false;
            }
        }
    }
}

/// Represents an application shell that could handle multiple windows. This implementation
/// is based on `minifb`.
pub struct Shell<A>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new() -> Self {
        Shell {
            window_shells: vec![],
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder {
            shell: self,
            adapter,
            title: String::default(),
            borderless: false,
            resizeable: false,
            always_on_top: false,
            bounds: Rectangle::new(0.0, 0.0, 100.0, 100.0),
            fonts: HashMap::new(),
        }
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(&mut self) {
        loop {
            if self.window_shells.is_empty() {
                return;
            }

            for i in 0..self.window_shells.len() {
                let mut remove = false;
                if let Some(window_shell) = self.window_shells.get_mut(i) {
                    window_shell.drain_events();
                    window_shell.receive_requests();
                    window_shell.update();
                    window_shell.render();

                    if !window_shell.is_open() {
                        remove = true;
                    }
                }

                if remove {
                    self.window_shells.remove(i);
                    break;
                }
            }
        }
    }
}
