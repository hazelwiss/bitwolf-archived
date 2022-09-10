use proc_bitfield::UnsafeInto;
use util::dumpable::UnsafeDumpString;

#[repr(C, packed)]
#[derive(UnsafeDumpString, Clone, Copy)]
pub struct CartridgeHeader {
    #[dump(unsafe_cast_str)]
    pub game_title: [u8; 12],
    pub game_code: u32,
    pub maker_code: u16,
    pub unit_code: u8,
    pub encryption_seed_select: u8,
    pub device_capacity: u8,
    #[dump(ignore)]
    __0: [u8; 8],
    pub nds_region: u8,
    pub rom_version: u8,
    pub auto_start: u8,
    pub arm9_rom_offset: u32,
    pub arm9_entry_address: u32,
    pub arm9_ram_address: u32,
    pub arm9_size: u32,
    pub arm7_rom_offset: u32,
    pub arm7_entry_address: u32,
    pub arm7_ram_address: u32,
    pub arm7_size: u32,
    pub file_name_table_fnt_offset: u32,
    pub file_name_table_fnt_size: u32,
    pub file_allocation_table_fat_offset: u32,
    pub file_allocation_table_fat_size: u32,
    pub file_arm9_overlay_offset: u32,
    pub file_arm9_overlay_size: u32,
    pub file_arm7_overlay_offset: u32,
    pub file_arm7_overlay_size: u32,
    pub port_40001a4h_setting_normal_commands: u32,
    pub port_40001a4h_setting_for_key1_commands: u32,
    pub icon_title_offset: u32,
    pub secure_area_checksum_ctc_16: u16,
    pub secure_area_delay: u16,
    pub arm9_auto_load_list_hook_ram_address: u32,
    pub arm7_auto_load_list_hook_ram_address: u32,
    pub secure_area_disable: u64,
    pub total_used_rom_size: u32,
    pub rom_header_size: u32,
    #[dump(ignore)]
    __1: [u8; 12],
    pub nand_end_of_rom_area: u16,
    pub nand_start_of_rw_area: u16,
    #[dump(ignore)]
    __2: [u8; 0x28],
    #[dump(ignore)]
    pub nintendo_logo: [u8; 0x9C],
    pub nintendo_logo_checksum: u16,
    pub header_checksum: u16,
    pub debug_rom_offset: u32,
    pub debug_size: u32,
    pub debug_ram_address: u32,
    #[dump(ignore)]
    __3: [u8; 0xE94],
}

impl Default for CartridgeHeader {
    fn default() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(UnsafeDumpString)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    #[dump(ignore)]
    pub main_data: alloc::vec::Vec<u8>,
}

impl Cartridge {
    pub fn from_rom(rom: alloc::vec::Vec<u8>) -> Self {
        parse_rom(&rom)
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self {
            header: Default::default(),
            main_data: Default::default(),
        }
    }
}

fn parse_cartridge_header(rom: &[u8]) -> CartridgeHeader {
    assert!(core::mem::size_of::<CartridgeHeader>() == 0x1000);
    unsafe {
        let mut header: CartridgeHeader = core::mem::MaybeUninit::zeroed().assume_init();
        let src = &rom[0] as *const u8;
        let dst = &mut header as *mut _ as *mut u8;
        src.copy_to(dst, 0x1000);
        header
    }
}

pub fn parse_rom(rom: &alloc::vec::Vec<u8>) -> Cartridge {
    assert!(rom.len() >= 0x7FFF);
    let header = parse_cartridge_header(rom);
    let main_data = rom[0x8000..rom.len()].to_vec();
    Cartridge { header, main_data }
}
