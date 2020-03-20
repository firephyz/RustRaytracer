extern crate sdl2;

use std::fs::File;

#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::convert::From;

use sdl2::{Sdl, VideoSubsystem, EventPump, IntegerOrSdlError};
use sdl2::event::{Event, EventType, WindowEvent};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::render::{Canvas};
use sdl2::video::{Window, WindowBuildError};

#[macro_use]
mod err_enum;
mod framerate;
mod scene;

use scene::camera::Camera;

// Create an enum wrapper over possible init err types
ErrorEnum!(
    AppInitErr,
    (String,
     IntegerOrSdlError,
     WindowBuildError)
);

struct AppContext {
    sdl_context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    events: EventPump,
}

fn main() {
    let mut context = match init_app() {
        Ok(c) => c,
        Err(e) => {
            let estring = format!("SDL2 init error: {}", e);
            panic!(estring);
        }
    };

    context.events.enable_event(EventType::Quit);
    context.events.enable_event(EventType::Window);

    let canvas_size = context.canvas.output_size().unwrap();
    let mut scene = scene::Scene::new(
        Camera::new(
            (0.5, 0.0, 0.5),
            (0.0, -0.5, 0.0),
            canvas_size.0,
            canvas_size.1,
            70.0));

    let mut is_running = true;
    let mut framerate_regulator = framerate::FramerateRegulator::new(30);
    while is_running {
        scene.render(&mut context.canvas);
        context.canvas.present();

        scene.lights[0].position.y += 0.1;

        poll_events(&mut context.events, &mut is_running);

        framerate_regulator.delay();
    }
}

fn init_app() -> Result<AppContext, AppInitErr> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

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

    Ok(AppContext {
        sdl_context: sdl_context,
        video: video_subsystem,
        canvas: canvas,
        events: event_pump,
    })
}

fn poll_events(events: &mut EventPump, is_running: &mut bool) {
    for event in events.poll_iter() {
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
                        *is_running = false;
                        break;
                    },
                    _ => {
                        //println!("{:#?}", event);
                    }
                }
            }
            _ => {
                //println!("{:#?}", event);
            },
        }
    }
}
