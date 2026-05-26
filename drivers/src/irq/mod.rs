//! External interrupt request and handle.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub mod gic_400;
    }
}
