use std::{error::Error, time::SystemTime};
use pixels::{SurfaceTexture, Pixels};
use vectors::{V2d, Vector};
use winit_input_helper::WinitInputHelper;
use winit::{dpi::LogicalSize, event::{VirtualKeyCode, Event}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

const BLOCK_S: i32 = 64;
const WIDTH: i32 = 8 * BLOCK_S;
const HEIGHT: i32 = 8 * BLOCK_S;

const MAP: [u8;64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 1, 1, 0, 1,
    1, 0, 0, 0, 0, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

mod player;
use crate::player::Player;

mod vectors;

fn is_wall((x, y): V2d) -> bool {
    let idx = x as i32 / 64;
    let idy = y as i32 / 64;
    let id = 8 * idy + idx;
    if MAP[id as usize] == 1 {
        true
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("My Raycaster")
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut me = Player::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let mut last_frame_time = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Update the player
            me.update();
            println!("{:?}", me);

            let frame = pixels.frame_mut();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let pos = one_to_two(i);

                // background
                let mut rgba = [0x5e, 0x48, 0xe8, 0xff];

                // walls
                if is_wall(pos) {
                    rgba = [0x40, 0x00, 0x40, 0xff];
                }

                // lines
                if pos.xi() % 64 == 0 || pos.yi() % 64 == 0 {
                    rgba = [0x66, 0x66, 0x66, 0xff];
                }

                // player
                if pos.dist(me.pos()) < 4.0 {
                    rgba = [0x00, 0x00, 0x00, 0x00];
                } else if me.pos().dist(pos) < 10.0 {
                    // direction
                    if me.ray(100.0).ang(pos.add(me.pos().scale(-1.0))) < 4.0 {
                        rgba = [0x00, 0xff, 0x0f, 0xff];
                    }
                }

                pixel.copy_from_slice(&rgba);
            }

            for i in 1..60 {
                let offs = (i - 30) as f64;
                let h = me.ray_cast(offs, &MAP);
                pixel_at(frame, h.xi() as usize, h.yi() as usize, (0xff, 0xff, 0xff))
            }

            if let Err(err) = pixels.render() {
                println!("Exiting due to error: {}", err.to_string());
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::W) {
                me.keys.w = true;
            }
            if input.key_released(VirtualKeyCode::W) {
                me.keys.w = false;
            }
            if input.key_pressed(VirtualKeyCode::A) {
                me.keys.a = true;
            }
            if input.key_released(VirtualKeyCode::A) {
                me.keys.a = false;
            }
            if input.key_pressed(VirtualKeyCode::S) {
                me.keys.s = true;
            }
            if input.key_released(VirtualKeyCode::S) {
                me.keys.s = false;
            }
            if input.key_pressed(VirtualKeyCode::D) {
                me.keys.d = true;
            }
            if input.key_released(VirtualKeyCode::D) {
                me.keys.d = false;
            }

            if input.key_released(VirtualKeyCode::Q) || input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            match last_frame_time.elapsed() {
                Ok(t) => {
                    if t.as_millis() > (1000 / 25) {
                        window.request_redraw();
                        last_frame_time = SystemTime::now();
                    }
                },
                Err(err) => {
                    println!("Exiting due to timing error: {}", err);
                    return;
                }
            }
        }
    });
}

fn pixel_at(frame: &mut [u8], x: usize, y: usize, rgb: (u8, u8, u8)) {
    let idx = y * WIDTH as usize + x;
    frame[idx * 4] = rgb.0;
    frame[idx * 4 + 1] = rgb.1;
    frame[idx * 4 + 2] = rgb.2;
    frame[idx * 4 + 3] = 0xff;
}

fn one_to_two(i: usize) -> V2d {
    let x = i % WIDTH as usize;
    let y = i / HEIGHT as usize;
    (x as f64, y as f64)
}

