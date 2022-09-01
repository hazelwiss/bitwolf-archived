pub trait Engine {
    type ARM9Data;
    type ARM7Data;
    type GlobalData;

    fn into_data() -> (Self::ARM9Data, Self::ARM7Data, Self::GlobalData);
}
