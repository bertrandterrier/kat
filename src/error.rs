#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid character `{0}`. Has to be /[a-z]/")]
    InvalLetter(char),
    #[error("Invalid character `{0}`. Has to be /[a-z0-9]/")]
    InvalMarkChar(char),
    #[error("Menge can never be empty.")]
    EmptyMenge,
}

#[derive(Debug, thiserror::Error)]
pub enum CatError {
    #[error("`{0}` already exists.")]
    Exists(String),
}
