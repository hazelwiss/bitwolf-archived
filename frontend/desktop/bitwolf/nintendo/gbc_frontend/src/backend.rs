use gbc_backend::{Builder, Core, Interpreter};
use util::colour::BGRA;

pub fn run(builder: Builder) {
    //let mut backend = Core::<Interpreter>::new(builder);
    let fb = builder.fb;
    let mut extra_x = 0.0 as f32;
    let mut extra_y = 0.0 as f32;
    loop {
        let writer = fb.get();
        let write = writer.write();
        for x in 0..160 {
            for y in 0..144 {
                let xf = (x as f32 + extra_x) / 160.0;
                let yf = (y as f32 + extra_y) / 144.0;
                write.text[y][x] = BGRA(
                    (256.0 * xf) as u8,
                    (256.0 * xf) as u8,
                    (256.0 * xf * yf) as u8,
                    0xFF,
                );
            }
        }
        extra_x += 0.0001;
        extra_y += 0.0001;
    }
}
