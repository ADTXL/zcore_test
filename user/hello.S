// Minimal zCore userspace program for aarch64
// Prints "Hello from zCore userspace!" to debug console, then loops

.section .text
.global _start

_start:
    // x16 = syscall number (96 = debug_write)
    // x0 = buffer pointer
    // x1 = length
    adrp    x0, msg
    add     x0, x0, :lo12:msg
    mov     x1, #27         // length of message
    mov     x16, #96        // ZX_SYS_debug_write
    svc     #0

    // Sleep forever: x16 = 8 (nanosleep), x0 = deadline = i64::MAX
    mov     x0, #-1         // deadline = -1 (forever)
    mov     x16, #8         // ZX_SYS_nanosleep
    svc     #0

    // Should never reach here, but just in case: loop
1:  b       1b

.section .rodata
msg:
    .ascii "Hello from zCore userspace!\n"
