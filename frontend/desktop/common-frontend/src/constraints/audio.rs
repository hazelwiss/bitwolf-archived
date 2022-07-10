pub trait SampleType {
    type Type;
}

pub struct Sample<T>(T);

impl SampleType for Sample<f32> {
    type Type = f32;
}

pub trait Audio {}
