use std::error::Error;
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

fn is_wall((x, y): (i32, i32)) -> bool {
    let idx = x / 64;
    let idy = y / 64;
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

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
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
                if pos.x() % 64 == 0 || pos.y() % 64 == 0 {
                    rgba = [0x66, 0x66, 0x66, 0xff];
                }

                // player
                if pos.x() > me.pos().x() - 4
                && pos.x() < me.pos().x() + 4
                && pos.y() > me.pos().y() - 4
                && pos.y() < me.pos().y() + 4 {
                    rgba = [0x00, 0x00, 0x00, 0x00];
                } else if me.pos().dist(pos) < 10.0 {
                    // direction
                    if me.ray(100).ang(pos.add(me.pos().scale(-1))) < 5 {
                        rgba = [0x00, 0xff, 0x0f, 0xff];
                    }
                }

                pixel.copy_from_slice(&rgba);
            }
            if let Err(err) = pixels.render() {
                println!("Exiting due to error: {}", err.to_string());
                return;
            }
        }

        if input.update(&event) {
            // Query keypresses this update
            if input.key_pressed_os(VirtualKeyCode::W) {
                me.step(5);
                println!("{:?}", me);
            }

            if input.key_pressed_os(VirtualKeyCode::A) {
                me.rotate(-5);
                println!("{:?}", me);
            }

            if input.key_pressed_os(VirtualKeyCode::S) {
                me.step(-5);
                println!("{:?}", me);
            }

            if input.key_pressed_os(VirtualKeyCode::D) {
                me.rotate(5);
                println!("{:?}", me);
            }

            if input.key_released(VirtualKeyCode::Q) || input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}

fn one_to_two(i: usize) -> V2d {
    let x = i % WIDTH as usize;
    let y = i / HEIGHT as usize;
    (x as i32, y as i32)
}

