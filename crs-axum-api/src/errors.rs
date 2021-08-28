use thiserror::Error;

#[derive(Error, Debug)]
pub enum OError {
    #[error("database error: {0}")]
    RBError(#[from] rbatis::Error),

    #[error("custom error: {1}")]
    CustomedError(u16, &'static str),

    #[error("internal error: {0}")]
    InternalError(&'static str),
}

pub type OResult<T> = Result<T, OError>;
