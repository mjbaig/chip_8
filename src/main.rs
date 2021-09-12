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

lazy_static! {
    static ref EMULATOR_STATE: Mutex<EmulatorState> = Mutex::new(EmulatorState::new());
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();

    let mut input = WinitInputHelper::new();

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

//This struct is temporary. I made it so that I could see that the screen is displaying pixels.
struct ViewPort {}

impl ViewPort {
    fn new() -> Self {
        Self {}
    }

    fn draw(&self, frame: &mut [u8]) {
        println!("{}", frame.len());
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let num = rand::thread_rng().gen_range(1..10);

            let random_red = rand::thread_rng().gen_range(0x00..0xFF);
            let random_green = rand::thread_rng().gen_range(0x00..0xFF);
            let random_blue = rand::thread_rng().gen_range(0x00..0xFF);

            let rgba = if (i % num) == 0 {
                [random_red, random_green, random_blue, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };

            //edits a single pixel by copying the value from rgba.
            pixel.copy_from_slice(&rgba)
        }
    }
}