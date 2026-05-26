#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
pub mod arch;
#[cfg(target_arch = "aarch64")]
pub use self::arch::timer_interrupt_vector;

pub mod boot;
pub mod mem;
pub mod net;
pub mod thread;
pub mod timer;

pub use self::arch::{config, cpu, interrupt, vm};
pub use super::hal_fn::{rand, vdso};

hal_fn_impl_default!(rand, vdso);
