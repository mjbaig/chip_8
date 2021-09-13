use rand::distributions::uniform::SampleBorrow;
use std::fs;
use std::path::{PathBuf, Path};
use std::time::Duration;

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
            graphics_buffer: [0; 64 * 32],
            op_code: 0,
            rom: None,
            should_redraw: true,
        };

        emulator_state.set_font_data();

        emulator_state
    }

    pub fn register_keypress(&self) {}

    pub fn run(&mut self) {
        if self.rom.is_none() {
            panic!("rom was not loaded prior to running the emulator.")
        }

        self.fetch();

        self.decode_and_execute();
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) {
        let contents_result = fs::read(&path);

        let rom = match contents_result {
            Ok(T) => T,
            Err(E) => panic!("could not load file at path: {}. Full Error: {}",
                             &path.as_ref().to_str().unwrap(), E)
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


    /// Reads the instruction that the program counter is pointing to in memory.
    /// Each instruction is 2 bytes, so this reads two bytes in a row and combines them into a single instruction
    /// Program counter is incremented by two after reading both bytes.
    fn fetch(&mut self) {
        let memory = self.ram;
        let program_counter = self.program_counter;

        if (program_counter as usize + 1) < memory.len() {
            let first_byte = (memory[program_counter as usize] as u16) << 8;
            let second_byte = (memory[(program_counter as usize) + 1] as u16);

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
            0xD000 => println!("drawing to screen"),
            _ => panic!("{:#06x} has not been implemented yet", self.op_code)
        }
    }

    fn draw_screen(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.graphics_buffer[i] == 1 {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    /// 00EO - Clears the graphics buffer and tells the client to redraw the screen.
    fn clear_screen(&mut self) {
        println!("clear screen");
        self.graphics_buffer = [0; 64 * 32];
        self.should_redraw = true;
    }

    /// 00EE - This returns from the subroutine
    // TODO test this behavior cause I'm not sure this works properly atm
    fn return_from_subroutine(&mut self) {
        println!("return from subroutine");

        let new_program_counter = self.stack.pop();

        self.program_counter = match new_program_counter {
            Some(T) => T,
            None => panic!("Cannot return from subroutine. The stack was empty."),
        }
    }

    /// Jumps to an address represented by the last three nibbles of the given command (1NNN)
    fn jump(&mut self) {
        println!("jump");
        self.program_counter = self.op_code & 0x0FFF;
    }

    /// Calls subroutine represented by the last three nibbles of the given command (2NNN)
    fn call_subroutine(&mut self) {
        println!("call subroutine");
        self.stack.push(self.program_counter);
        self.program_counter = self.op_code & 0x0FFF;
    }

    fn set_register(&mut self) {
        println!("set register");
        let op_code = self.op_code;
        let x = (op_code & 0x0F00) >> 8;

        let value_nn = (op_code & 0x00FF) as u8;

        self.general_variable_registers[x as usize] = value_nn;
    }

    /// Adds the last to nibbles of the given command to the value register at index X (7XNN)
    fn add_value_to_register(&mut self) {
        println!("add value to register");
        let op_code = self.op_code;
        let x = (op_code & 0x0F00) >> 8;

        let value_nn = (op_code & 0x00FF) as u8;

        self.general_variable_registers[x as usize] =
            self.general_variable_registers[x as usize] + value_nn;
    }

    /// Sets the index register to the last 3 nibbles of the given command (ANNN)
    fn set_index_register(&mut self) {
        println!("set index register");
        self.index_register = self.op_code & 0x0FFF
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
    emulator_state.run();

    //this should be the opcode after a single cycle
    assert_eq!(0x00e0, emulator_state.op_code);
}

#[test]
fn sample_test() {
    let path = r"Z:\Documents\Dev\rust\chip_8\test_roms\IBM Logo.ch8";

    let mut emulator_state = EmulatorState::new();

    emulator_state.load_rom(path);

    for x in [1; 500] {
        emulator_state.run();
    }
}

#[test]
fn sample_test_2() {
    let op_code: u32 = 0x6971;

    let x = (op_code & 0x00FF) as u8;

    println!("{:#06x}", x)
}