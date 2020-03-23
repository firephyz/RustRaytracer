use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::camera::Camera;

pub enum InputEventArgs {
    None,
    CameraRotation {
        yaw: i32,
        pitch: i32,
        roll: i32,
    },
    CameraTranslation {
        forback: i32,
        leftright: i32,
        updown: i32,
    },
}

// pub trait InputEventArgs {
//     type RetType;
//     fn unwrap(&self) -> Self::RetType;
// }
//
// #[derive(Clone, Copy)]
// pub struct CameraRotationEvent {
//     yaw: i32,
//     pitch: i32,
//     roll: i32,
// }
//
// impl InputEventArgs for &CameraRotationEvent {
//     type RetType = CameraRotationEvent;
//     fn unwrap(&self) -> Self::RetType { *self }
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
// Rather than storing a single fn type, store a fn type that implements a call trait so
// second argument isn't always required.
pub struct InputState_t {
    pub camera: Rc<Camera>,
    pub enable_rotation: bool,
}

pub struct InputState {
    pub events: Vec<(fn(&mut InputState_t, InputEventArgs), InputEventArgs)>,
    pub state: InputState_t,
}

impl InputState {
    pub fn new(camera: Camera) -> Self {
        InputState {
            events: Vec::new(),
            state: InputState_t {
                camera: Rc::new(camera),
                enable_rotation: false,
            },
        }
    }
}

pub mod callbacks {
    use std::rc::Rc;
    use super::{InputState_t, InputEventArgs};

    pub fn enable_rotation(input: &mut InputState_t, args: InputEventArgs) {
        input.enable_rotation = true;
    }

    pub fn disable_rotation(input: &mut InputState_t, args: InputEventArgs) {
        input.enable_rotation = false;
    }

    pub fn rotate(input: &mut InputState_t, args: InputEventArgs) {
        if input.enable_rotation {
            match args {
                InputEventArgs::CameraRotation {
                    yaw: yaw,
                    pitch: pitch,
                    roll: roll,
                } => {
                    unsafe {
                        Rc::get_mut_unchecked(&mut input.camera).move_rotate(yaw, pitch, roll);
                    }
                    //Rc::get_mut(&mut input.camera).unwrap().move_rotate(yaw, pitch, roll);
                },
                _ => std::unreachable!(),
            }
        }
    }
}
