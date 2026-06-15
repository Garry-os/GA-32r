#[derive(Debug)]
pub enum EmuError {
    InvalidRegister,
    ReadOnlyRegister,
    AttemptToAccessImm,
    InvalidOpcode
}

#[derive(Default, Debug)]
pub enum EmuStatus {
    Error,
    Halt,
    #[default]
    Running
}
