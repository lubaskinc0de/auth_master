use std::fmt::Debug;

pub(crate) trait AppError: Debug {
    fn get_error_code(&self) -> String;
    fn get_error_description(&self) -> String;
}

impl<E: AppError> From<ErrorKind<std::convert::Infallible>> for ErrorKind<E> {
    fn from(value: ErrorKind<std::convert::Infallible>) -> Self {
        ErrorKind::Unexpected(value.into())
    }
}

impl AppError for ErrorKind<std::convert::Infallible> {
    fn get_error_code(&self) -> String {
        match self {
            ErrorKind::Expected(_) => unreachable!(),
            ErrorKind::Unexpected(_) => "INTERNAL_ERROR".to_string(),
        }
    }
    fn get_error_description(&self) -> String {
        match self {
            ErrorKind::Expected(_) => unreachable!(),
            ErrorKind::Unexpected(_) => "internal error".to_string(),
        }
    }
}

impl<E: AppError> AppError for ErrorKind<E> {
    fn get_error_code(&self) -> String {
        match &self {
            ErrorKind::Expected(e) => e.get_error_code(),
            ErrorKind::Unexpected(_) => "INTERNAL_ERROR".to_string(),
        }
    }

    fn get_error_description(&self) -> String {
        match &self {
            ErrorKind::Expected(e) => e.get_error_description(),
            ErrorKind::Unexpected(_) => "internal error".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorKind<E> {
    #[error(transparent)]
    Expected(E),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub(crate) type VoidResult<E: AppError> = Result<(), ErrorKind<E>>;
pub(crate) type Failable<T, E: AppError> = Result<T, ErrorKind<E>>;
pub(crate) type Infallible<T> = Result<T, ErrorKind<std::convert::Infallible>>;
pub(crate) type InfallibleVoid = Result<(), ErrorKind<std::convert::Infallible>>;

#[macro_export]
macro_rules! unexpected_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                return Err($crate::entities::errors::base::ErrorKind::Unexpected(
                    e.into(),
                ))
            }
        }
    };
}
