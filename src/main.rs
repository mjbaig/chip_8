#![forbid(unsafe_code)]

#[path = "emulator/emulator.rs"] mod emulator;

use lazy_static::lazy_static;
use winit::event_loop::{EventLoop, ControlFlow};
use winit_input_helper::WinitInputHelper;
use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use pixels::{SurfaceTexture, Pixels, Error};
use winit::event::{Event, VirtualKeyCode};
use log::error;
use std::sync::Mutex;
use rand::Rng;
use emulator::EmulatorState;
use std::borrow::Borrow;

lazy_static! {
    static ref EMULATOR_STATE: Mutex<EmulatorState> = Mutex::new(EmulatorState::new());
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const BOX_SIZE: i16 = 64;

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();

    let mut input = WinitInputHelper::new();

    println!("{:?}", EMULATOR_STATE.lock().unwrap().ram);

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("chip 8 emulator?")
            .with_max_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop).unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let view_port = ViewPort::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            view_port.draw(pixels.get_frame());

            if pixels.render().map_err(|e| error!("pixels.render() failed {}", e)).is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}

struct ViewPort {}

impl ViewPort {
    fn new() -> Self {
        Self {}
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let num = rand::thread_rng().gen_range(1..10);

            let rgba = if (i % num) == 0 {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };

            //edits a single pixel by copying the value from rgba.
            pixel.copy_from_slice(&rgba)
        }
    }
}