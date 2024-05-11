use thiserror::Error;

#[derive(Error, Debug)]
pub enum MorphError {}

pub type Result<T, E = MorphError> = std::result::Result<T, E>;
