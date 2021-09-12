use rand::distributions::uniform::SampleBorrow;

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
    pub graphics_buffer: [u8; 64 * 32]
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
            op_code: 0
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

