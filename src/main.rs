extern crate sdl2;

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

    // Render to texture for accelerated rendering
    let texture_creator = context.canvas.texture_creator();
    let texture_size = context.canvas.output_size().unwrap();
    let mut texture_a = texture_creator.create_texture_target(
        PixelFormatEnum::RGB888,
        texture_size.0,
        texture_size.1).unwrap();

    let scene = scene::Scene::new(
        Camera::new(
            (0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            texture_size.0,
            texture_size.1,
            70.0));

    let mut is_running = true;
    let mut framerate_regulator = framerate::FramerateRegulator::new(60);
    while is_running {
        context.canvas.with_texture_canvas(&mut texture_a, |t_canvas| {
            scene.render(t_canvas);
        }).unwrap();

        context.canvas.copy(&texture_a, None, None).unwrap();
        context.canvas.present();

        poll_events(&mut context.events, &mut is_running);

        framerate_regulator.delay();
    }
}

fn init_app() -> Result<AppContext, AppInitErr> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    let window = video_subsystem.window("raytracer", 200, 150)
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
