extern crate sdl2;

use std::fs::File;

use std::fmt;
use std::convert::From;
use std::rc::Rc;

use sdl2::{Sdl, VideoSubsystem, EventPump, IntegerOrSdlError};
use sdl2::event::{Event, EventType, WindowEvent};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::render::{Canvas};
use sdl2::video::{Window, WindowBuildError};

#[macro_use]
mod err_enum;
mod framerate;
mod scene;
mod input;
mod camera;

use scene::Scene;
use input::InputState;
use camera::Camera;

// Create an enum wrapper over possible init err types
ErrorEnum!(
    AppInitErr,
    (String,
     IntegerOrSdlError,
     WindowBuildError)
);

pub struct AppContext {
    sdl_context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    events: EventPump,
    is_running: bool,
    scene: Scene,
    input: InputState,
}

fn init_app() -> Result<AppContext, AppInitErr> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsystem.window("raytracer", 300, 300)
    .position_centered()
    .build()?;

    let canvas = window.into_canvas()
    .accelerated()
    .build()?;

    if !canvas.render_target_supported() {
        eprintln!("Render target not supported.");
        std::process::exit(1);
    }

    event_pump.enable_event(EventType::Quit);
    event_pump.enable_event(EventType::Window);

    let canvas_size = canvas.output_size().unwrap();
    let camera = Camera::new(
        (0.5, 0.0, 0.5),
        (0.0, -0.5, 0.0),
        canvas_size.0,
        canvas_size.1,
        70.0);

    let input = InputState::new(camera);
    let scene = scene::Scene::new(Rc::downgrade(&input.state.camera));

    Ok(AppContext {
        sdl_context: sdl_context,
        video: video_subsystem,
        canvas: canvas,
        events: event_pump,
        is_running: true,
        scene: scene,
        input: input,
    })
}

fn main() {
    let mut context = match init_app() {
        Ok(c) => c,
        Err(e) => {
            let estring = format!("SDL2 init error: {}", e);
            panic!(estring);
        }
    };

    let mut light_delta = 0.1;
    let mut framerate_regulator = framerate::FramerateRegulator::new(30);
    while context.is_running {
        context.scene.render(&mut context.canvas);
        context.canvas.present();

        if context.scene.lights[0].position.y > 3.0 {
            light_delta = -0.1;
        }
        else if context.scene.lights[0].position.y < -3.0 {
            light_delta = 0.1;
        }
        context.scene.lights[0].position.y += light_delta;

        poll_events(&mut context);
        update_context(&mut context);

        framerate_regulator.delay();
    }
}

fn update_context(context: &mut AppContext) {
    let events = &mut context.input.events;
    while !events.is_empty() {
        let (event_handler, args) = events.pop().unwrap();
        event_handler(&mut context.input.state, args);
    }
}

fn poll_events(context: &mut AppContext) {

    for event in context.events.poll_iter() {
        match event {
            Event::Quit{timestamp} => {
                eprintln!("SIGINT received at timestamp: {}", timestamp);
                eprintln!("Aborting...");
                std::process::abort();
            },
            Event::Window{
                timestamp,
                window_id,
                win_event,
            } => {
                match win_event {
                    WindowEvent::Close => {
                        context.is_running = false;
                        break;
                    },
                    _ => {
                        //println!("{:#?}", event);
                    }
                }
            }
            Event::MouseButtonDown {
                timestamp, window_id, which,
                mouse_btn, clicks, x, y,
            } => {
                context.input.events.push((
                    input::callbacks::enable_rotation,
                    None));
            },
            Event::MouseButtonUp {
                timestamp, window_id, which,
                mouse_btn, clicks, x, y,
            } => {
                context.input.events.push((
                    input::callbacks::disable_rotation,
                    None));
            },
            Event::MouseMotion {
                timestamp, window_id, which,
                mousestate, x, y, xrel, yrel
            } => {
                context.input.events.push((
                    input::callbacks::rotate,
                    Some(Box::new(input::CameraRotationEvent {
                        yaw: xrel,
                        pitch: -yrel,
                        roll:0 as i32})),
                ));
            },
            Event::KeyDown {
                timestamp, window_id,
                keycode, scancode, keymod, repeat
            } => {
                // context.input_state.events.push((
                //     ::enable_key,
                //     Some(Box::new((keycode)))));
            },
            Event::KeyUp {
                timestamp, window_id,
                keycode, scancode, keymod, repeat
            } => {
                // context.input_state.release_keyboard(keycode);
            },
            _ => {
                //println!("{:#?}", event);
            },
        }
    }
}
