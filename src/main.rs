use std::{error::Error, time::SystemTime, f64::consts::PI};
use pixels::{SurfaceTexture, Pixels};
use winit_input_helper::WinitInputHelper;
use winit::{dpi::LogicalSize, event::{VirtualKeyCode, Event}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

const BLOCK_S: i32 = 64;
const MAP_WIDTH: i32 = 8 * BLOCK_S;
const MAP_HEIGHT: i32 = 8 * BLOCK_S;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const SLICES: i32 = 256;
const SLICE_W: i32 = SCREEN_WIDTH / SLICES;

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
use crate::player::{Player, Ray};

mod vectors;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("My Raycaster")
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut me = Player::new(MAP);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?
    };

    let mut last_frame_time = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Update the player
            me.update();

            let frame = pixels.frame_mut();
            for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
                // background
                let rgba = [0x55, 0x55, 0x55, 0xff];
                pixel.copy_from_slice(&rgba);
            }

            for i in 0..SLICES {

                let offs = ((i + 1 as i32 - (SLICES / 2)) as f64 / 100.0).atan();
                let hit = me.ray_cast(to_deg(offs), &MAP);
                let dist = hit.to_dist();
                let height = (20.0 * SCREEN_HEIGHT as f64 / dist).min(SCREEN_HEIGHT as f64);
                let mut col = (0xff, 0xff, 0xff);
                if let Ray::H(_, _) = hit {
                    col = (0xaa, 0xaa, 0xaa);
                }
                draw_slice(frame, i as usize, height as i32, col);
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

fn draw_slice(frame: &mut [u8], i: usize, height: i32, rgb: (u8, u8, u8)) {
    let middle = SCREEN_HEIGHT / 2;
    let bottom = middle - (height / 2);
    let top = middle + (height / 2);
    for y in bottom..top {
        for mut x in 0..SLICE_W {
            x = i as i32 * SLICE_W + x;
            pixel_at(frame, x as usize, y as usize, rgb);
        }
    }
}

fn pixel_at(frame: &mut [u8], x: usize, y: usize, rgb: (u8, u8, u8)) {
    let idx = y * SCREEN_WIDTH as usize + x;
    frame[idx * 4] = rgb.0;
    frame[idx * 4 + 1] = rgb.1;
    frame[idx * 4 + 2] = rgb.2;
    frame[idx * 4 + 3] = 0xff;
}

fn to_deg(i: f64) -> f64 {
    i * (180.0 / PI)
}

