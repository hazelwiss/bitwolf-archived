macro_rules! header {
    (1) => {
        u8
    };
    (2) => {
        u16
    };
    (4) => {
        u32
    };
    (8) => {
        u64
    };
    ($($name:ident, $base:literal, $size:tt);* $(;)?) => {
        #[derive(Debug, Clone, Default)]
        pub struct Header {
            $(
                $name: header!($size)
            ),*
        }

        impl Header{
            pub fn from_rom(rom: &[u8]) -> Self{
                assert!(rom.len() > 0x200);
                unsafe { Self{
                    $(
                        $name: (rom[$base..$base+$size].as_ptr() as *const header!($size)).read()
                    ),*
                }}
            }

            $(
                pub fn $name (&self) -> header!($size){
                    self.$name
                }
            )*
        }
    };
}

header! {
    arm9_rom_offset, 0x020, 4;
    arm9_entry_address, 0x024, 4;
    arm9_ram_address, 0x028, 4;
    arm9_size, 0x02C, 4;
}
