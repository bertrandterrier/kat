use std::{
    fmt::Debug,
    cmp::PartialEq,
};
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid character `{0}`. Has to be /[a-z]/")]
    InvalLetter(char),
    #[error("Invalid character `{0}`. Has to be /[a-z0-9]/")]
    InvalMarkChar(char),
    #[error("Menge can never be empty.")]
    EmptyMenge,
    #[error("Unknown parsing error.")]
    SomeParseErr,
    #[error("`{0}` already exists in Katalog at `{1}`")]
    KatExists(String, usize),
    #[error("Invalid seperation marker `{0}`")]
    InvalSepMark(char),
    #[error("Invalid character for `Marke` `{0}`")]
    InvalMarke(char),
    #[error("Some syntax error in `{0}`")]
    Syntax(String),
    #[error("Cannot create actual Menge from optional Menge with nulll values: `{0}`")]
    NoNullmenge(String),
}
