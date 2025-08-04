#[macro_export]
macro_rules! define_module {
    (license: $license:expr) => {
        #[cfg(not(test))]
        #[no_mangle]
        pub static _MODULE_LICENSE: &[u8] = $license.as_bytes();
    };

    (license: $license:expr, author: $author:expr) => {
        $crate::define_module!(license: $license);

        #[cfg(not(test))]
        #[no_mangle]
        pub static _MODULE_AUTHOR: &[u8] = $author.as_bytes();
    };
}