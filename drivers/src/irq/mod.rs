//! External interrupt request and handle.

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        mod riscv_intc;
        mod riscv_plic;

        /// Implementation of risc-v interrupt controller.
        #[doc(cfg(any(target_arch = "riscv32", target_arch = "riscv64")))]
        pub mod riscv {
            pub use super::riscv_intc::{Intc, ScauseIntCode};
            pub use super::riscv_plic::Plic;
        }
    } else if #[cfg(target_arch = "aarch64")] {
        pub mod gic_400;
    }
}
