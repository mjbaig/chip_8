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
    pub ram: [u8; 4096],
    /* PC: points to the current instruction in memory */
    pub program_counter: u16,
    /* Index register "I" is used to point to locations in memory */
    pub index_register: u16,
    /* A stack for 16bit addresses. used to call functions and return from them */
    pub stack: Vec<u16>,
    /* Delay timer. Decremented at a rate of 60hz (60 times per second) until it reaches 0 */
    pub delay_timer: u8,
    /* Sound timer acts like delay timer but also gives off beeping sound at long as its not 0 */
    pub sound_timer: u8,
    /* 16 8bit general purpose variable registers. 0-F in hex. Can be called V0 to VF */
    pub general_variable_registers: [u8; 16]
}

impl Default for EmulatorState {
    fn default() -> Self {
        EmulatorState {
            ram: [0; 4096],
            program_counter: 0,
            index_register: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            general_variable_registers: [0; 16]
        }
    }
}

impl EmulatorState {

    pub fn new() -> Self {
        let mut emulator_state = Self::default();

        emulator_state.set_font_data();

        emulator_state
    }

    fn set_font_data(&mut self) {
        for (index, value) in FONT_DATA.iter().enumerate() {
            self.ram[index] = value.to_owned()
        }
    }

}
