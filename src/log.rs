#[macro_export]
macro_rules! pr_log {
    ($level:expr, $($arg:tt)*) => ({
    });
}

#[macro_export]
macro_rules! pr_info {
    ($($arg:tt)*) => ($crate::pr_log!("INFO", $($arg)*))
}

#[macro_export]
macro_rules! pr_warn {
    ($($arg:tt)*) => ($crate::pr_log!("WARN", $($arg)*))
}

#[macro_export]
macro_rules! pr_err {
    ($($arg:tt)*) => ($crate::pr_log!("ERR", $($arg)*))
}

#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => ($crate::pr_log!("DEBUG", $($arg)*))
}