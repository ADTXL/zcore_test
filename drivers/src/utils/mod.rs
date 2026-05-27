//! Event handler and device tree.

#[allow(dead_code)]
mod event_listener;
#[allow(dead_code)]
mod id_allocator;
#[allow(dead_code)]
mod irq_manager;

#[cfg(feature = "graphic")]
mod graphic_console;

pub mod devicetree;

#[allow(unused_imports)]
pub(super) use id_allocator::IdAllocator;
#[allow(unused_imports)]
pub(super) use irq_manager::IrqManager;

pub use event_listener::{EventHandler, EventListener};
