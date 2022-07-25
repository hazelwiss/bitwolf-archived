pub(in crate::core) enum Cycles {
    T(u32),
    M(u32),
}

impl Cycles {
    pub fn as_t(&self) -> u32 {
        match self {
            Cycles::T(t) => *t,
            Cycles::M(m) => m * 4,
        }
    }
}
