extern crate sdl2;

use std::fmt;
use std::convert::From;

use sdl2::{Sdl, VideoSubsystem, EventPump, IntegerOrSdlError};
use sdl2::event::{Event, EventType, WindowEvent};
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::render::{Canvas};
use sdl2::video::{Window, WindowBuildError};
use sdl2::rect::{Rect};

#[macro_use]
mod err_enum;
mod framerate;

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

    let mut is_running = true;
    let mut framerate_regulator = framerate::FramerateRegulator::new(60);
    while is_running {
        context.canvas.with_texture_canvas(&mut texture_a, |t_canvas| {
            render_scene(t_canvas, camera, scene);
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

    let window = video_subsystem.window("raytracer", 800, 600)
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

fn render_scene(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();
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
