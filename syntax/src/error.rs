pub type LowerResult<T> = Result<T, LowerError>;

#[derive(Debug)]
pub struct LowerError {
    pub message: String,
}

#[macro_export]
macro_rules! errors {
    ($($arg:tt)*) => {
        $crate::LowerError {
            message: format!($($arg)*),
        }
    };
}

#[macro_export]
macro_rules! lower_todo {
    () => {
        return Err($crate::errors!("Not implemented"))
    };
}
