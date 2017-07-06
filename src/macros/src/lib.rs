#[macro_export]
macro_rules! get_lock {
    ($l:expr, $m:ident) => {
        match $l.$m() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Got poisoned rwlock on {}", stringify!($l));
                poisoned.into_inner()
            }
        }
    }
}

#[macro_export]
macro_rules! get_read_lock {
    ($l:expr) => {
        get_lock!($l, read)
    }
}

#[macro_export]
macro_rules! get_write_lock {
    ($l:expr) => {
        get_lock!($l, write)
    }
}