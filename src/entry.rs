use crate::{cpu::CPU, emu_window, rom::ROM, utils::{log, reset_logs}};

const TETRIS_ROM_PATH: &str = "tests/tetris.gb";
const BAKERY_ROM_PATH: &str = "tests/bakery_v1.0.3.gb";
const TEST_ROM_PATH: &str = "tests/cpu_instrs/cpu_instrs.gb";


pub fn setup() -> () {
    reset_logs();

    let mut cpu = CPU::new();
    let rom: ROM = ROM::read_rom(TEST_ROM_PATH);

    cpu.pc = 0x100; // TODO: don't always skip the bootrom!

    log(&format!(
        "\
        ROM Name: \"{}\"\n\
        ROM Size: {}KB\n\
        ROM Type: \"{}\"\n\
        ROM Region: \"{}\"\n\
        ROM Version: \"{}\"\n\
        Calculated Header Checksum: {:04X}\n\
        Original Header Checksum: {:04X}\n\
        \n\
        CPU PC: 0x{:04X} | {}\n\
        ",

        rom.name,
        rom.size,
        ROM::get_cartridge_type_name(rom.cartridge_type),
        rom.region, rom.version,
        rom.header_checksum,
        rom.data[0x14D],
        cpu.pc, cpu.pc
    ));

    log("--------------------\n");

    cpu.mem_bus.memory = rom.data.clone().try_into().expect("Couldn't convert ROM bytes vector to an array");
    cpu.rom_size = rom.size;

    emu_window::window_life(cpu);
}

#[allow(unused)]
pub fn process(mut cpu: CPU, rom_file: ROM) -> () {
    loop {
        cpu.step();

        if cpu.pc >= rom_file.size as u16 {
            break;
        }
    }
}