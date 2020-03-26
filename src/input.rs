use std::rc::Rc;
use std::cell::RefCell;

use sdl2::keyboard::Keycode;

use crate::camera::Camera;

#[derive(Debug)]
pub enum KeyboardKey {
    Q,
    E,
    W,
    A,
    S,
    D,
}

#[derive(Debug)]
pub enum InputEvent {
    MouseMotion {
        xrel: i32,
        yrel: i32,
    },
    KeyChange(KeyboardKey),
}

struct KeyboardState {
    q: bool,
    e: bool,
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl KeyboardState {
    fn new() -> Self {
        KeyboardState {
            q: false,
            e: false,
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }
}

pub struct InputState {
    pub events: Vec<InputEvent>,
    camera: Rc<RefCell<Camera>>,
    drag_camera: bool,
    keyboard: KeyboardState,
}

impl InputState {
    pub fn new(camera: Camera) -> Self {
        InputState {
            events: Vec::new(),
            camera: Rc::new(RefCell::new(camera)),
            drag_camera: false,
            keyboard: KeyboardState::new(),
        }
    }

    pub fn get_camera(&self) -> Rc<RefCell<Camera>> {
        self.camera.clone()
    }

    pub fn toggle_drag_camera(&mut self) {
        self.drag_camera = !self.drag_camera;
    }

    pub fn update(&mut self) {
        for event in self.events.iter() {
            match event {
                InputEvent::MouseMotion {xrel, yrel} => {
                    if self.drag_camera {
                        (*self.camera).borrow_mut().move_rotate(*xrel, -*yrel, 0);
                    }
                },
                InputEvent::KeyChange(key) => {
                    match key {
                        KeyboardKey::Q => self.keyboard.q = !self.keyboard.q,
                        KeyboardKey::E => self.keyboard.e = !self.keyboard.e,
                        KeyboardKey::W => self.keyboard.w = !self.keyboard.w,
                        KeyboardKey::A => self.keyboard.a = !self.keyboard.a,
                        KeyboardKey::S => self.keyboard.s = !self.keyboard.s,
                        KeyboardKey::D => self.keyboard.d = !self.keyboard.d,
                    }
                },
            }
        }
        self.events.clear();

        if self.keyboard.q ^ self.keyboard.e {
            let value = {
                if self.keyboard.q { -3 }
                else { 3 }
            };
            self.camera.borrow_mut().move_rotate(0, 0, -value);
        }

        if self.keyboard.w ^ self.keyboard.s {
            if self.keyboard.w { self.camera.borrow_mut().move_translate(1, 0); }
            else { self.camera.borrow_mut().move_translate(-1, 0); }
        }

        if self.keyboard.a ^ self.keyboard.d {
            if self.keyboard.a { self.camera.borrow_mut().move_translate(0, -1); }
            else { self.camera.borrow_mut().move_translate(0, 1); }
        }
    }
}
