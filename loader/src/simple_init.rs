//! Simple init process loader - loads our own userspace program directly
//! without depending on prebuilt Fuchsia binaries.

use alloc::{boxed::Box, sync::Arc};


use kernel_hal::context::{TrapReason, UserContext, UserContextField};
use kernel_hal::{MMUFlags, PAGE_SIZE};
use zircon_object::kcounter;
use zircon_object::object::KernelObject;
use zircon_object::task::{CurrentThread, ExceptionType, Job, Process, Thread, ThreadState};
use zircon_object::util::elf_loader::{ElfExt, VmarExt};
use zircon_object::vm::{VmObject, VmarFlags};

kcounter!(EXCEPTIONS_USER, "exceptions.user");
kcounter!(EXCEPTIONS_IRQ, "exceptions.irq");
kcounter!(EXCEPTIONS_PGFAULT, "exceptions.pgfault");

/// Run a simple init program from an ELF binary.
/// This bypasses the entire Fuchsia boot stack (userboot, ZBI, vDSO, etc.)
pub fn run_simple_init(elf_data: &[u8], name: &str) -> Arc<Process> {
    use xmas_elf::ElfFile;

    info!(
        "run_simple_init: loading '{}' ({} bytes)",
        name,
        elf_data.len()
    );

    let elf = ElfFile::new(elf_data).unwrap();
    let entry = elf.header.pt2.entry_point();

    let job = Job::root();
    let proc = Process::create(&job, name).unwrap();
    let thread = Thread::create(&proc, name).unwrap();
    let vmar = proc.vmar();

    // Load ELF segments into process address space
    let size = elf.load_segment_size();
    let proc_vmar = vmar
        .allocate(None, size, VmarFlags::CAN_MAP_RXW, PAGE_SIZE)
        .unwrap();
    proc_vmar.load_from_elf(&elf).unwrap();

    let entry_addr = proc_vmar.addr() + entry as usize;
    info!(
        "run_simple_init: entry={:#x}, vmar_base={:#x}, size={:#x}",
        entry_addr,
        proc_vmar.addr(),
        size
    );

    // Allocate stack
    const STACK_PAGES: usize = 8;
    let stack_vmo = VmObject::new_paged(STACK_PAGES);
    let flags = MMUFlags::READ | MMUFlags::WRITE | MMUFlags::USER;
    let stack_bottom = vmar
        .map(None, stack_vmo.clone(), 0, stack_vmo.len(), flags)
        .unwrap();
    let sp = stack_bottom + stack_vmo.len();

    info!(
        "run_simple_init: stack at {:#x}, sp={:#x}",
        stack_bottom, sp
    );

    // Start the process
    proc.start(&thread, entry_addr, sp, None, 0, thread_fn)
        .expect("failed to start init process");

    proc
}

fn thread_fn(
    thread: CurrentThread,
) -> core::pin::Pin<alloc::boxed::Box<dyn core::future::Future<Output = ()> + Send + 'static>> {
    Box::pin(run_user(thread))
}

async fn run_user(thread: CurrentThread) {
    kernel_hal::thread::set_current_thread(Some(thread.inner()));
    if thread.is_first_thread() {
        thread
            .handle_exception(ExceptionType::ProcessStarting)
            .await;
    }
    thread.handle_exception(ExceptionType::ThreadStarting).await;

    loop {
        let mut ctx = thread.wait_for_run().await;
        if thread.state() == ThreadState::Dying {
            break;
        }

        trace!("switch to {}|{}", thread.proc().name(), thread.name());
        let tmp_time = kernel_hal::timer::timer_now().as_nanos();

        ctx.enter_uspace();

        let time = kernel_hal::timer::timer_now().as_nanos() - tmp_time;
        thread.time_add(time);
        trace!("back from user: {:#x?}", ctx);
        EXCEPTIONS_USER.add(1);

        if let Err(e) = handler_user_trap(&thread, ctx).await {
            if let ExceptionType::ThreadExiting = e {
                break;
            }
            thread.handle_exception(e).await;
        }
    }
    thread.handle_exception(ExceptionType::ThreadExiting).await;
}

async fn handler_user_trap(
    thread: &CurrentThread,
    mut ctx: Box<UserContext>,
) -> Result<(), ExceptionType> {
    let reason = ctx.trap_reason();

    if let TrapReason::Syscall = reason {
        let num = syscall_num(&ctx);
        let args = syscall_args(&ctx);
        ctx.advance_pc(reason);
        thread.put_context(ctx);
        let mut syscall = zircon_syscall::Syscall { thread, thread_fn };
        let ret = syscall.syscall(num as u32, args).await as usize;
        thread
            .with_context(|ctx| ctx.set_field(UserContextField::ReturnValue, ret))
            .map_err(|_| ExceptionType::ThreadExiting)?;
        return Ok(());
    }

    thread.put_context(ctx);
    match reason {
        TrapReason::Interrupt(vector) => {
            EXCEPTIONS_IRQ.add(1);
            kernel_hal::interrupt::handle_irq(vector);
            kernel_hal::thread::yield_now().await;
            Ok(())
        }
        TrapReason::PageFault(vaddr, flags) => {
            EXCEPTIONS_PGFAULT.add(1);
            info!("page fault from user mode @ {:#x}({:?})", vaddr, flags);
            let vmar = thread.proc().vmar();
            vmar.handle_page_fault(vaddr, flags).map_err(|err| {
                error!(
                    "FATAL: failed to handle page fault @ {:#x}({:?}): {:?}",
                    vaddr, flags, err
                );
                ExceptionType::FatalPageFault
            })
        }
        TrapReason::SoftwareBreakpoint => {
            info!("breakpoint from user mode");
            thread
                .with_context(|ctx| ctx.advance_pc(reason))
                .map_err(|_| ExceptionType::ThreadExiting)?;
            Ok(())
        }
        r => {
            error!("unsupported trap from user mode: {:?}", r);
            Err(ExceptionType::General)
        }
    }
}

fn syscall_num(ctx: &UserContext) -> usize {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            ctx.general().x16 as usize
        } else {
            let _ = ctx;
            unimplemented!("syscall_num: unsupported architecture")
        }
    }
}

fn syscall_args(ctx: &UserContext) -> [usize; 8] {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            let r = ctx.general();
            [
                r.x0 as usize,
                r.x1 as usize,
                r.x2 as usize,
                r.x3 as usize,
                r.x4 as usize,
                r.x5 as usize,
                r.x6 as usize,
                r.x7 as usize,
            ]
        } else {
            let _ = ctx;
            unimplemented!("syscall_args: unsupported architecture")
        }
    }
}
