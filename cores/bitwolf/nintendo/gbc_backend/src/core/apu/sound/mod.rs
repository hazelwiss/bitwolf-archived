mod ch1_ch2;
mod ch3;
mod ch4;

use super::APU;

static mut TEST: f32 = 0.0;
static mut cycles: u64 = 0;

impl APU {
    pub fn tick(&mut self, t_cycles: u32) {
        //ch1_ch2::
        unsafe {
            for _ in 0..t_cycles {
                self.audio_interface.handle_sample(TEST.sin());
                TEST += 0.0001;
            }
            cycles += 1;
        }
    }
}
