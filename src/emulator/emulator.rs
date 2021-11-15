use std::fs;
use std::path::{Path};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F
];

pub struct EmulatorState {
    /* 4kilobytes of ram */
    ram: [u8; 4096],
    /* PC: points to the current instruction in memory */
    program_counter: u16,
    /* Index register "I" is used to point to locations in memory */
    index_register: u16,
    /* A stack for 16bit addresses. used to call functions and return from them */
    stack: Vec<u16>,
    /* Delay timer. Decremented at a rate of 60hz (60 times per second) until it reaches 0 */
    delay_timer: u8,
    /* Sound timer acts like delay timer but also gives off beeping sound at long as its not 0 */
    sound_timer: u8,
    /* 16 8bit general purpose variable registers. 0-F in hex. Can be called V0 to VF */
    general_variable_registers: [u8; 16],
    /* Operation code for cpu and whatever */
    op_code: u16,
    /* Graphics buffer to hold pixel data */
    pub graphics_buffer: [u8; 64 * 32],
    /* rom file to be loaded */
    rom: Option<Vec<u8>>,
    /* Informs the client that screen must be redrawn */
    should_redraw: bool,
}

impl EmulatorState {
    pub fn new() -> Self {
        let mut emulator_state = EmulatorState {
            ram: [0; 4096],
            program_counter: 0x200,
            index_register: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            general_variable_registers: [0; 16],
            graphics_buffer: [0; (WIDTH * HEIGHT) as usize],
            op_code: 0,
            rom: None,
            should_redraw: true,
        };

        emulator_state.set_font_data();

        emulator_state
    }

    pub fn screen_height(&self) -> u32 {
        return HEIGHT;
    }

    pub fn screen_width(&self) -> u32 {
        return WIDTH;
    }

    pub fn register_keypress(&self) {}

    pub fn tick(&mut self) {
        if self.rom.is_none() {
            panic!("rom was not loaded prior to running the emulator.")
        }

        self.fetch();

        self.decode_and_execute();
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) {
        let contents_result = fs::read(&path);

        let rom = match contents_result {
            Ok(t) => t,
            Err(e) => panic!("could not load file at path: {}. Full Error: {}",
                             &path.as_ref().to_str().unwrap(), e)
        };

        // Load rom into memory
        for (i, val) in rom.iter().enumerate() {
            self.ram[i + 0x200] = val.to_owned();
        }

        self.rom = Some(rom)
    }

    fn set_font_data(&mut self) {
        for (index, value) in FONT_DATA.iter().enumerate() {
            self.ram[index] = value.to_owned();
        }
    }

    pub fn draw_screen(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.graphics_buffer[i] == 1 {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    /// Reads the instruction that the program counter is pointing to in memory.
    /// Each instruction is 2 bytes, so this reads two bytes in a row and combines them into a single instruction
    /// Program counter is incremented by two after reading both bytes.
    fn fetch(&mut self) {
        let memory = self.ram;
        let program_counter = self.program_counter;

        if (program_counter as usize + 1) < memory.len() {
            let first_byte = (memory[program_counter as usize] as u16) << 8;
            let second_byte = memory[(program_counter as usize) + 1] as u16;

            self.op_code = first_byte | second_byte;
            self.program_counter = self.program_counter + 2;
        } else {
            panic!("program counter is pointing outside of available ram")
        }
    }

    /// This decodes the instruction (op_code) found in fetch to find out what needs to be done next and then execute the command
    fn decode_and_execute(&mut self) {
        let op_code = self.op_code;

        match op_code & 0xF000 {
            0x0000 => {
                match op_code & 0x000F {
                    0x0000 => self.clear_screen(),
                    0x000E => self.return_from_subroutine(),
                    _ => panic!("{:#06x} has not been implemented yet", self.op_code)
                }
            }
            0x1000 => self.jump(),
            0x2000 => self.call_subroutine(),
            0x6000 => self.set_register(),
            0x7000 => self.add_value_to_register(),
            0xA000 => self.set_index_register(),
            0xD000 => self.draw(),
            _ => panic!("{:#06x} has not been implemented yet", self.op_code)
        }
    }

    /// 00EO - Clears the graphics buffer and tells the client to redraw the screen.
    fn clear_screen(&mut self) {
        println!("clearing screen");
        self.graphics_buffer = [0; 64 * 32];
        self.should_redraw = true;
    }

    /// 00EE - This returns from the subroutine
    // TODO test this behavior cause I'm not sure this works properly atm
    fn return_from_subroutine(&mut self) {
        let new_program_counter = self.stack.pop();

        println!("returning from subroutine");

        self.program_counter = match new_program_counter {
            Some(t) => t,
            None => panic!("Cannot return from subroutine. The stack was empty."),
        }
    }

    /// Jumps to an address represented by the last three nibbles of the given command (1NNN)
    fn jump(&mut self) {
        self.program_counter = self.op_code & 0x0FFF;
    }

    /// Calls subroutine represented by the last three nibbles of the given command (2NNN)
    fn call_subroutine(&mut self) {

        println!("calling from subroutine");

        self.stack.push(self.program_counter);
        self.program_counter = self.op_code & 0x0FFF;
    }

    /// Sets the last two nibbles of the given command to the value register at index X (6XNN)
    fn set_register(&mut self) {
        let op_code = self.op_code;
        let x = (op_code & 0x0F00) >> 8;

        let value_nn = (op_code & 0x00FF) as u8;

        println!("setting {:#06x} to register at {:#06x}", &value_nn, &x);

        self.general_variable_registers[x as usize] = value_nn;
    }

    /// Adds the last two nibbles of the given command to the value register at index X (7XNN)
    fn add_value_to_register(&mut self) {
        let op_code = self.op_code;
        let x = (op_code & 0x0F00) >> 8;

        let value_nn = (op_code & 0x00FF) as u8;

        println!("adding {:#06x} to register at {:#06x}", &value_nn, &x);

        self.general_variable_registers[x as usize] += value_nn;
    }

    /// Sets the index register to the last 3 nibbles of the given command (ANNN)
    fn set_index_register(&mut self) {
        self.index_register = self.op_code & 0x0FFF;
        println!("Set index register to {:#06x}", self.index_register);
    }

    /// Draws a pixel on the buffer
    fn set_pixel(&mut self, x_coordinate: u32, y_coordinate: u32) -> u8 {

        let mut adjusted_x = x_coordinate;

        let mut adjusted_y = y_coordinate;

        // Wrap the value around if X is greater than the width
        if adjusted_x > WIDTH {
            adjusted_x -= WIDTH;
        } else if adjusted_x < 0 {
            adjusted_x += WIDTH;
        }

        if adjusted_y > HEIGHT {
            adjusted_y -= HEIGHT;
        } else if adjusted_y < 0 {
            adjusted_y += HEIGHT
        }

        let location = adjusted_x + (adjusted_y * WIDTH);

        self.graphics_buffer[location as usize] ^= 1;

        return self.graphics_buffer[location as usize];

    }

    /// Draws a pixel (DXYN)
    fn draw(&mut self) {
        let op_code = self.op_code;

        let ram = self.ram;

        let x_index = ((op_code & 0x0F00) >> 8) as usize;
        let x_coordinate = self.general_variable_registers[x_index] % 64 /* mod 64 so that the value can wrap */;

        let y_index = ((op_code & 0x00F0) >> 4) as usize;
        let y_coordinate = self.general_variable_registers[y_index] % 32 /* mod 32 so that the value can wrap */;

        let index_register = self.index_register;

        let height = op_code & 0x000F;

        self.general_variable_registers[0xF] = 0; /* TODO find out why I set V0 to 0 */

        for n in 0..height as u8 {

            let mut sprite_data = ram[(index_register + (n as u16)) as usize];

            for bit in 0..8 as u8 {

                let pixel = sprite_data & (0x80);

                if pixel > 0 {
                    if self.set_pixel((x_coordinate + bit) as u32, (y_coordinate + n) as u32) == 1 {
                        self.general_variable_registers[0xF] = 1;
                        println!("x: {} y:{} op:{:#06x} index:{:#06x} pixel:{}", (x_index as u8 + bit), (y_index as u8 + n), op_code, index_register, pixel);
                    }
                }
                sprite_data <<= 1;
            }

        }

        self.should_redraw = true;

    }
}

#[test]
fn emulator_creation_test() {
    let emulator_state = EmulatorState::new();

    println!("{:?}", emulator_state.ram)
}

#[test]
fn rom_load_test() {
    let path = r"Z:\Documents\Dev\rust\chip_8\test_roms\IBM Logo.ch8";

    let mut emulator_state = EmulatorState::new();

    emulator_state.load_rom(path);

    assert!(&emulator_state.rom.as_ref().is_some());

    assert_eq!(&132, &emulator_state.rom.as_ref().unwrap().len());

    let rom = emulator_state.rom.as_ref().unwrap();

    assert_eq!(0, rom[0]);
    assert_eq!(0xE0, rom[1]);
    assert_eq!(0xE0, rom[rom.len() - 1]);
}

#[test]
fn run_cycle_test() {
    let path = r"Z:\Documents\Dev\rust\chip_8\test_roms\IBM Logo.ch8";

    let mut emulator_state = EmulatorState::new();

    emulator_state.load_rom(path);

    //runs a single cycle
    emulator_state.tick();

    //this should be the opcode after a single cycle
    assert_eq!(0x00e0, emulator_state.op_code);
}

#[test]
fn sample_test() {
    let path = r"Z:\Documents\Dev\rust\chip_8\test_roms\IBM Logo.ch8";

    let mut emulator_state = EmulatorState::new();

    emulator_state.load_rom(path);

    for x in [1; 500] {
        emulator_state.tick();
    }
}

#[test]
fn sample_test_2() {
    let x = 1;

    println!("{:#06x}", x ^ 0)
}