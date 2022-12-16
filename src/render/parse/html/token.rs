#[derive(Debug, PartialEq, Clone)]
pub enum HTMLToken {
    Lt,
    Gt,
    Slash,
    Eq,
    DoubleQuot,
    SingleQuot,
    Exclamation,
    Newline,
    Space,
    Tab,
    Word(String),
    EOF,
}
