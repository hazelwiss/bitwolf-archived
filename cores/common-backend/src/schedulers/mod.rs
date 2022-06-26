mod pqueu;

pub type Scheduler<Slot> = pqueu::PQueue<u64, Slot>;
