use util::dumpable::UnsafeDumpString;

#[repr(C, packed)]
#[derive(UnsafeDumpString)]
pub struct CartridgeHeader {
    #[dump(unsafe_cast_str)]
    game_title: [u8; 12],
    game_code: u32,
    maker_code: u16,
    unit_code: u8,
    encryption_seed_select: u8,
    device_capacity: u8,
    #[dump(ignore)]
    __0: [u8; 8],
    nds_region: u8,
    rom_version: u8,
    auto_start: u8,
    arm9_rom_offset: u32,
    arm9_entry_address: u32,
    arm9_ram_address: u32,
    arm9_size: u32,
    arm7_rom_offset: u32,
    arm7_entry_address: u32,
    arm7_ram_address: u32,
    arm7_size: u32,
    file_name_table_fnt_offset: u32,
    file_name_table_fnt_size: u32,
    file_allocation_table_fat_offset: u32,
    file_allocation_table_fat_size: u32,
    file_arm9_overlay_offset: u32,
    file_arm9_overlay_size: u32,
    file_arm7_overlay_offset: u32,
    file_arm7_overlay_size: u32,
    port_40001a4h_setting_normal_commands: u32,
    port_40001a4h_setting_for_key1_commands: u32,
    icon_title_offset: u32,
    secure_area_checksum_ctc_16: u16,
    secure_area_delay: u16,
    arm9_auto_load_list_hook_ram_address: u32,
    arm7_auto_load_list_hook_ram_address: u32,
    secure_area_disable: u64,
    total_used_rom_size: u32,
    rom_header_size: u32,
    #[dump(ignore)]
    __1: [u8; 12],
    nand_end_of_rom_area: u16,
    nand_start_of_rw_area: u16,
    #[dump(ignore)]
    __2: [u8; 0x28],
    #[dump(ignore)]
    nintendo_logo: [u8; 0x9C],
    nintendo_logo_checksum: u16,
    header_checksum: u16,
    debug_rom_offset: u32,
    debug_size: u32,
    debug_ram_address: u32,
    #[dump(ignore)]
    __3: [u8; 0xE94],
}

pub fn parse_rom(rom: &Vec<u8>) -> CartridgeHeader {
    assert!(rom.len() >= 0x1000);
    assert!(std::mem::size_of::<CartridgeHeader>() == 0x1000);

    unsafe {
        let mut header: CartridgeHeader = std::mem::MaybeUninit::uninit().assume_init();
        let src = &rom[0] as *const u8;
        let dst = &mut header as *mut _ as *mut u8;
        src.copy_to(dst, 0x1000);
        header
    }
}
