use thiserror;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value of a skill must be between 0 and 100, but {0} is {1}")]
    SkillValueBound(String, u8),
}


pub type Result<T> = std::result::Result<T, Error>;