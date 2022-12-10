#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    AddX(i64),
    Noop,
}
