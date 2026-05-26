//! User context.

use crate::{MMUFlags, VirtAddr};
use core::fmt;
use trapframe::UserContext as UserContextInner;

pub use trapframe::GeneralRegs;

cfg_if! {
    if #[cfg(feature = "libos")] {
        pub use trapframe::syscall_fn_entry as syscall_entry;
    } else {
        pub use dummpy_syscall_entry as syscall_entry;
        pub fn dummpy_syscall_entry() {
            unreachable!("dummpy_syscall_entry")
        }
    }
}

/// For reading and writing fields in [`UserContext`].
#[derive(Debug)]
pub enum UserContextField {
    InstrPointer,
    StackPointer,
    ThreadPointer,
    ReturnValue,
}

/// Reason of the trap.
#[derive(Debug, PartialEq, Eq)]
pub enum TrapReason {
    Syscall,
    Interrupt(usize),
    PageFault(VirtAddr, MMUFlags),
    UndefinedInstruction,
    SoftwareBreakpoint,
    HardwareBreakpoint,
    UnalignedAccess,
    GernelFault(usize),
}

#[cfg(not(feature = "libos"))]
pub const TIMER_INTERRUPT_VEC: usize = crate::timer_interrupt_vector();

impl TrapReason {
    #[cfg(target_arch = "riscv64")]
    pub fn from(scause: riscv::register::scause::Scause) -> Self {
        use riscv::register::scause::{Exception, Trap};
        let stval = riscv::register::stval::read();
        match scause.cause() {
            Trap::Exception(Exception::UserEnvCall) => Self::Syscall,
            Trap::Exception(Exception::Breakpoint) => Self::SoftwareBreakpoint,
            Trap::Exception(Exception::IllegalInstruction) => Self::UndefinedInstruction,
            Trap::Exception(Exception::InstructionMisaligned)
            | Trap::Exception(Exception::StoreMisaligned) => Self::UnalignedAccess,
            Trap::Exception(Exception::LoadPageFault) => Self::PageFault(stval, MMUFlags::READ),
            Trap::Exception(Exception::StorePageFault) => Self::PageFault(stval, MMUFlags::WRITE),
            Trap::Exception(Exception::InstructionPageFault) => {
                Self::PageFault(stval, MMUFlags::EXECUTE)
            }
            Trap::Interrupt(_) => Self::Interrupt(scause.code()),
            _ => Self::GernelFault(scause.code()),
        }
    }

    #[cfg(target_arch = "aarch64")]
    pub fn from(esr: usize) -> Self {
        // TODO: check if is right
        use crate::{Fault, Info, Kind, Source, Syndrome};
        use cortex_a::registers::{ESR_EL1, FAR_EL1};
        use tock_registers::interfaces::Readable;

        let info = Info {
            source: Source::from(esr & 0xffff),
            kind: Kind::from((esr >> 16) & 0xffff),
        };
        let esr = ESR_EL1.get() as u32;
        match info.kind {
            Kind::Synchronous => match Syndrome::from(esr) {
                Syndrome::Breakpoint => Self::SoftwareBreakpoint,
                Syndrome::Svc(_) => Self::Syscall,
                Syndrome::DataAbort { kind: _, level: _ } => Self::PageFault(
                    FAR_EL1.get() as _,
                    MMUFlags::READ | MMUFlags::WRITE | MMUFlags::USER,
                ),
                Syndrome::InstructionAbort {
                    kind: Fault::Permission,
                    level: _,
                } => Self::PageFault(FAR_EL1.get() as _, MMUFlags::EXECUTE | MMUFlags::USER),
                Syndrome::PCAlignmentFault | Syndrome::SpAlignmentFault => Self::UnalignedAccess,
                _ => Self::GernelFault(esr as usize),
            },
            Kind::Irq => Self::Interrupt(
                #[cfg(not(feature = "libos"))]
                {
                    use crate::hal_fn::mem::phys_to_virt;
                    use crate::KCONFIG;
                    zcore_drivers::irq::gic_400::get_irq_num(
                        phys_to_virt(KCONFIG.gic_base + 0x1_0000),
                        phys_to_virt(KCONFIG.gic_base),
                    )
                },
                #[cfg(feature = "libos")]
                {
                    // TODO: interrupt in libOS
                    usize::MAX
                },
            ),
            _ => Self::GernelFault(esr as usize),
        }
    }
}

/// User context saved on trap.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UserContext(UserContextInner);

impl UserContext {
    /// Create an empty user context.
    pub fn new() -> Self {
        let context = UserContextInner::default();
        Self(context)
    }

    /// Initialize the context for entry into userspace.
    /// Note: if the number of args < 3, please fill with zeros
    /// Eg: ctx.setup_uspace(pc_, sp_, &[arg1, arg2, 0])
    pub fn setup_uspace(&mut self, pc: usize, sp: usize, args: &[usize; 3]) {
        cfg_if! {
            if #[cfg(target_arch = "aarch64")] {
                self.0.elr = pc;
                self.0.sp = sp;
                self.0.general.x0 = args[0];
                self.0.general.x1 = args[1];
                self.0.general.x2 = args[2];
                // Mask SError exceptions (currently unhandled).
                // TODO
                self.0.spsr = 1 << 8;
            } else if #[cfg(target_arch = "riscv64")] {
                self.0.sepc = pc;
                self.0.general.sp = sp;
                self.0.general.a0 = args[0];
                self.0.general.a1 = args[1];
                self.0.general.a2 = args[2];
                // SUM = 1, FS = 0b11, SPIE = 1
                self.0.sstatus = 1 << 18 | 0b11 << 13 | 1 << 5;
            }
        }
    }

    /// Setup return addr
    pub fn set_ra(&mut self, _ra: usize) {
        cfg_if! {
            if #[cfg(target_arch = "riscv64")] {
                self.0.general.ra = _ra;
            } else if #[cfg(target_arch = "aarch64")] {
                self.0.general.x30 = _ra;
            } else {
                unimplemented!("Unsupported arch!");
            }
        }
    }

    /// Switch to user mode.
    pub fn enter_uspace(&mut self) {
        cfg_if! {
            if #[cfg(feature = "libos")] {
                self.0.run_fncall()
            } else {
                self.0.run()
            }
        }
    }

    /// Returns [`TrapReason`] according to the context.
    pub fn trap_reason(&self) -> TrapReason {
        cfg_if! {
            if #[cfg(target_arch = "aarch64")] {
                TrapReason::from(self.0.trap_num)
            } else if #[cfg(target_arch = "riscv64")] {
                TrapReason::from(riscv::register::scause::read())
            } else {
                unimplemented!()
            }
        }
    }
    /// Returns a `usize` representing the trap reason. (i.e., IDT vector for x86, `scause` for RISC-V)
    pub fn raw_trap_reason(&self) -> usize {
        cfg_if! {
            if #[cfg(target_arch = "aarch64")] {
                unimplemented!() // ESR_EL1
            } else if #[cfg(target_arch = "riscv64")] {
                riscv::register::scause::read().bits()
            } else {
                unimplemented!()
            }
        }
    }

    /// Returns the reference of general registers.
    pub fn general(&self) -> &GeneralRegs {
        &self.0.general
    }

    /// Returns the mutable reference of general registers.
    pub fn general_mut(&mut self) -> &mut GeneralRegs {
        &mut self.0.general
    }

    fn field_ref(&mut self, which: UserContextField) -> &mut usize {
        cfg_if! {
            if #[cfg(target_arch = "aarch64")] {
                match which {
                    UserContextField::InstrPointer => &mut self.0.elr,
                    UserContextField::StackPointer => &mut self.0.sp,
                    UserContextField::ThreadPointer => &mut self.0.tpidr,
                    UserContextField::ReturnValue => &mut self.0.general.x0,
                }
            } else if #[cfg(target_arch = "riscv64")] {
                match which {
                    UserContextField::InstrPointer => &mut self.0.sepc,
                    UserContextField::StackPointer => &mut self.0.general.sp,
                    UserContextField::ThreadPointer => &mut self.0.general.tp,
                    UserContextField::ReturnValue => &mut self.0.general.a0,
                }
            } else {
                unimplemented!()
            }
        }
    }

    /// Read a field of the context.
    pub fn get_field(&mut self, which: UserContextField) -> usize {
        *self.field_ref(which)
    }

    /// Write a field of the context.
    pub fn set_field(&mut self, which: UserContextField, value: usize) {
        *self.field_ref(which) = value;
    }

    /// Advance the instruction pointer in trap handler on some architecture.
    pub fn advance_pc(&mut self, reason: TrapReason) {
        cfg_if! {
            if #[cfg(target_arch = "riscv64")] {
                if let TrapReason::Syscall = reason { self.0.sepc += 4 }
            } else {
                let _ = reason;
            }
        }
    }
}

impl Default for UserContext {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for UserContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


