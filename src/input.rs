use std::any::Any;
use std::rc::Rc;

use sdl2::keyboard::Keycode;

use crate::camera::Camera;

pub trait InputEventArgs {
    type RetType;
    fn unwrap(&self) -> Self::RetType;
}

#[derive(Clone, Copy)]
pub struct CameraRotationEvent {
    yaw: i32,
    pitch: i32,
    roll: i32,
}

impl InputEventArgs for &CameraRotationEvent {
    type RetType = CameraRotationEvent;
    fn unwrap(&self) -> Self::RetType { *self }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Rather than storing a single fn type, store a fn type that implements a call trait so
// second argument isn't always required.
pub struct InputState_t {
    pub camera: Rc<Camera>,
    pub enable_rotation: bool,
}

type EvArgT = Box<dyn InputEventArgs<RetType=Any>>;
pub struct InputState {
    pub events: Vec<(fn(&mut InputState_t, Option<EvArgT>), Option<EvArgT>)>,
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
    use std::any::Any;
    use crate::AppContext;
    use super::{InputState, InputState_t, InputEventArgs};
    use super::{CameraRotationEvent};

    type NoArgs = Option<Box<dyn InputEventArgs<RetType=Any>>>;

    pub fn enable_rotation(input: &mut InputState_t, args: NoArgs) {
        input.enable_rotation = true;
    }

    pub fn disable_rotation(input: &mut InputState_t, args: NoArgs) {
        input.enable_rotation = false;
    }

    pub fn rotate(input: &mut InputState_t, args: Option<Box<dyn InputEventArgs<RetType=Any>>>) {
        if input.enable_rotation {
            let args : () = args.unwrap().as_ref().unwrap();
            Rc::get_mut(&mut input.camera).unwrap().move_rotate(args.yaw, args.pitch, args.roll);
        }
    }
}
