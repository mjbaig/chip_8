use rand::distributions::uniform::SampleBorrow;
use std::fs;
use std::path::PathBuf;

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
    rom: Option<Vec<u8>>
}

impl Default for EmulatorState {
    fn default() -> Self {
        EmulatorState {
            ram: [0; 4096],
            program_counter: 0x200,
            index_register: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            general_variable_registers: [0; 16],
            graphics_buffer: [0; 64 * 32],
            op_code: 0,
            rom: None
        }
    }
}

impl EmulatorState {

    pub fn new() -> Self {
        let mut emulator_state = Self::default();

        emulator_state.set_font_data();

        emulator_state
    }

    pub fn register_keypress(&self) {

    }

    pub fn load_rom(&mut self, path: PathBuf) {
        let contents_result = fs::read(&path);

        let rom = if contents_result.is_ok() {
            contents_result.unwrap()
        } else {
            panic!("could not load file at path: {}", &path.to_str().unwrap())
        };

        self.rom = Some(rom)

    }

    fn set_font_data(&mut self) {
        for (index, value) in FONT_DATA.iter().enumerate() {
            self.ram[index] = value.to_owned()
        }
    }

    /**
        Reads the instruction that the program counter is pointing to in memory.
        Each instruction is 2 bytes, so this reads two bytes in a row and combines them into a single instruction
        Program counter is incremented by two after reading both bytes.
    */
    fn fetch(&self) {

    }

    /**
        This decodes the instruction (op_code) found in fetch to find out what needs to be done next.
    */
    fn decode(&self) {

    }

    fn execute(&self) {

    }

    fn draw_screen(&self, frame: &mut [u8]) {

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let rgba = if self.graphics_buffer[i] == 1 {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba)

        }

    }

}

#[test]
fn emulator_creation_test() {

    let emulator_state = EmulatorState::new();

    println!("{:?}", emulator_state.ram)

}

#[test]
fn rom_load_test() {

    let mut path_buf = PathBuf::new();

    path_buf.push(r"Z:\Documents\Dev\rust\chip_8\test_roms\IBM Logo.ch8");

    let mut emulator_state = EmulatorState::new();

    emulator_state.load_rom(path_buf);

    assert!(&emulator_state.rom.as_ref().is_some());

    assert_eq!(&132, &emulator_state.rom.as_ref().unwrap().len());

    let rom = emulator_state.rom.as_ref().unwrap();

    assert_eq!(0, rom[0]);
    assert_eq!(0xE0, rom[1]);
    assert_eq!(0xE0, rom[rom.len() - 1]);
}

