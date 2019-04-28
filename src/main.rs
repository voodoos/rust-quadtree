extern crate sdl2;

mod lib;
mod vals;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use rand::Rng;

use lib::geometry::AABB;
use lib::traits::{Drawable, Dynamic};
use vals::TestVal;

fn main() -> Result<(), String> {
    let dim = 256;
    let mut qt = lib::QuadTree::<TestVal>::default();
    qt.insert(TestVal {
        bbox: AABB {
            x: 1,
            y: 1,
            w: 10,
            h: 10,
        },
    });

    qt.insert(TestVal {
        bbox: AABB {
            x: 50,
            y: 50,
            w: 10,
            h: 10,
        },
    });

    qt.insert(TestVal {
        bbox: AABB {
            x: 150,
            y: 150,
            w: 10,
            h: 10,
        },
    });

    qt.insert(TestVal {
        bbox: AABB {
            x: 240,
            y: 240,
            w: 10,
            h: 10,
        },
    });
    println!("{:#?}", qt);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Quadtree demo", dim, dim)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut rng = rand::thread_rng();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        /*qt.insert(TestVal {
            bbox: AABB {
                x: rng.gen_range(1, dim) as i32,
                y: rng.gen_range(1, dim) as i32,
                w: 10,
                h: 10,
            },
        });*/

        qt.update(&Duration::new(0, 0));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        qt.draw(&mut canvas)?;
        canvas.present();
    }
    Ok(())
}
