#[macro_export]
macro_rules! kb {
    ($v:literal) => {
        ($v * 1024)
    };
}

#[macro_export]
macro_rules! mb {
    ($v:literal) => {
        ($v * 1024 * 1024)
    };
}
