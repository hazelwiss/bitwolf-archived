use crate::config::Config;
use crate::core::Core___;

pub struct ProgramState<C: Core___> {
    pub config: Config,
    pub core: C,
}
