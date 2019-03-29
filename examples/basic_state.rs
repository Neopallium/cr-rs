
#[derive(Debug)]
pub struct BasicState {
    pub counter: i32,
}

pub type BasicPlugin = cr::Plugin<BasicState>;

