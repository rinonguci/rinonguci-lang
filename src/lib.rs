#[macro_export]
macro_rules! new_error {
    () => {
        Object::Error(Error {
            message: "".to_string(),
        })
    };
    ($($arg:tt)*) => {{
        Object::Error(Error {
            message: format!($($arg)*),
        })
    }};
}
