use super::APU;

pub fn run(apu: &mut APU) {
    apu.channel1();
    apu.channel2();
}

impl APU {
    fn square_wave(&self) -> u8 {
        0
    }

    fn channel1(&mut self) {}

    fn channel2(&mut self) {
        //self.
    }
}
