cfg_if! {
    if #[cfg(feature = "libos")] {
        #[path = "libos/mod.rs"]
        mod arch;
    } else if #[cfg(target_arch = "aarch64")] {
        #[path = "aarch64/mod.rs"]
        mod arch;
    }
}

pub use arch::consts::*;
