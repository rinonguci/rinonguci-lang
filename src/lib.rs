#[macro_export]
macro_rules! new_error {
    () => {
        Object::Error(crate::object::Error {
            message: "".to_string(),
        })
    };
    ($($arg:tt)*) => {{
       Object::Error( crate::object::Error {
            message: format!($($arg)*),
        })
    }};
}
