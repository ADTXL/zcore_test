// zCore Shell - aarch64 userspace
// Syscalls used:
//   96 = debug_write(buf_ptr, len)  -> x0=ptr, x1=len, x16=96, svc #0
//   95 = debug_read(handle, buf_ptr, len_ptr) -> x0=handle, x1=buf, x2=len_ptr, x16=95, svc #0
//   8  = nanosleep(deadline)        -> x0=deadline, x16=8, svc #0

.section .text
.global _start

_start:
    // Print welcome banner
    adrp    x0, banner
    add     x0, x0, :lo12:banner
    mov     x1, banner_len
    bl      write

    // Print prompt
    bl      print_prompt

main_loop:
    // Read a line from serial
    bl      read_line

    // Check if we got anything
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    ldrb    w1, [x0]
    cbz     w1, main_loop       // empty line, just reprint prompt

    // Check for built-in commands
    bl      try_echo
    cbnz    w0, cmd_done

    bl      try_help
    cbnz    w0, cmd_done

    bl      try_hello
    cbnz    w0, cmd_done

    bl      try_info
    cbnz    w0, cmd_done

    // Unknown command
    adrp    x0, msg_unknown
    add     x0, x0, :lo12:msg_unknown
    mov     x1, msg_unknown_len
    bl      write

cmd_done:
    bl      print_prompt
    b       main_loop

// ---- Subroutines ----

// write: x0=buf, x1=len
write:
    // x16=96 (debug_write), x0=buf, x1=len
    mov     x16, #96
    svc     #0
    ret

// print_prompt
print_prompt:
    stp     x29, x30, [sp, #-16]!
    adrp    x0, prompt
    add     x0, x0, :lo12:prompt
    mov     x1, prompt_len
    bl      write
    ldp     x29, x30, [sp], #16
    ret

// read_line: reads into line_buf, null-terminates
read_line:
    stp     x29, x30, [sp, #-16]!
    stp     x19, x20, [sp, #-16]!

    adrp    x19, line_buf
    add     x19, x19, :lo12:line_buf
    mov     w20, #0             // current position

.read_char:
    // debug_read(handle=0, buf=stack_byte, len_ptr=stack_len)
    sub     sp, sp, #16
    mov     x0, #0              // handle = 0 (skip resource check)
    add     x1, sp, #8          // buf = sp+8 (1 byte)
    mov     w2, #1
    str     w2, [sp]            // len = 1
    mov     x2, sp              // len_ptr
    mov     x16, #95            // debug_read
    svc     #0
    ldrb    w0, [sp, #8]        // read the character
    add     sp, sp, #16

    // Check for newline (end of line)
    cmp     w0, #'\n'
    b.eq    .line_done

    // Check for backspace/delete
    cmp     w0, #0x7f           // DEL
    b.eq    .do_backspace
    cmp     w0, #0x08           // BS
    b.eq    .do_backspace

    // Store character
    cmp     w20, #126           // max line length
    b.ge    .read_char          // ignore if buffer full
    add     x3, x19, x20
    strb    w0, [x3]
    add     w20, w20, #1

    // Echo character back
    sub     sp, sp, #16
    strb    w0, [sp]
    mov     x0, sp
    mov     x1, #1
    bl      write
    add     sp, sp, #16

    b       .read_char

.do_backspace:
    cbz     w20, .read_char     // nothing to delete
    sub     w20, w20, #1
    // Echo backspace + space + backspace
    sub     sp, sp, #16
    mov     w0, #0x08
    strb    w0, [sp]
    mov     w0, #' '
    strb    w0, [sp, #1]
    mov     w0, #0x08
    strb    w0, [sp, #2]
    mov     x0, sp
    mov     x1, #3
    bl      write
    add     sp, sp, #16
    b       .read_char

.line_done:
    // Null-terminate the line
    add     x3, x19, x20
    strb    wzr, [x3]
    // Print newline
    sub     sp, sp, #16
    mov     w0, #'\n'
    strb    w0, [sp]
    mov     x0, sp
    mov     x1, #1
    bl      write
    add     sp, sp, #16

    ldp     x19, x20, [sp], #16
    ldp     x29, x30, [sp], #16
    ret

// try_echo: handles "echo ..." command
// Returns w0=1 if handled, 0 if not
try_echo:
    stp     x29, x30, [sp, #-16]!
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    adrp    x1, cmd_echo
    add     x1, x1, :lo12:cmd_echo
    mov     w2, #5              // len("echo ")
    bl      strneq
    cbz     w0, .not_echo

    // Print the argument after "echo "
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    add     x0, x0, #5         // skip "echo "
    // Calculate remaining length
    bl      strlen
    mov     x1, x0
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    add     x0, x0, #5
    bl      write
    // Print newline
    sub     sp, sp, #16
    mov     w0, #'\n'
    strb    w0, [sp]
    mov     x0, sp
    mov     x1, #1
    bl      write
    add     sp, sp, #16
    mov     w0, #1
    ldp     x29, x30, [sp], #16
    ret

.not_echo:
    mov     w0, #0
    ldp     x29, x30, [sp], #16
    ret

// try_help: handles "help" command
try_help:
    stp     x29, x30, [sp, #-16]!
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    adrp    x1, cmd_help
    add     x1, x1, :lo12:cmd_help
    mov     w2, #4              // len("help")
    bl      strneq
    cbz     w0, .not_help

    adrp    x0, msg_help
    add     x0, x0, :lo12:msg_help
    mov     x1, msg_help_len
    bl      write
    mov     w0, #1
    ldp     x29, x30, [sp], #16
    ret

.not_help:
    mov     w0, #0
    ldp     x29, x30, [sp], #16
    ret

// try_hello: handles "hello" command
try_hello:
    stp     x29, x30, [sp, #-16]!
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    adrp    x1, cmd_hello
    add     x1, x1, :lo12:cmd_hello
    mov     w2, #5              // len("hello")
    bl      strneq
    cbz     w0, .not_hello

    adrp    x0, msg_hello_reply
    add     x0, x0, :lo12:msg_hello_reply
    mov     x1, msg_hello_reply_len
    bl      write
    mov     w0, #1
    ldp     x29, x30, [sp], #16
    ret

.not_hello:
    mov     w0, #0
    ldp     x29, x30, [sp], #16
    ret

// try_info: handles "info" command
try_info:
    stp     x29, x30, [sp, #-16]!
    adrp    x0, line_buf
    add     x0, x0, :lo12:line_buf
    adrp    x1, cmd_info
    add     x1, x1, :lo12:cmd_info
    mov     w2, #4              // len("info")
    bl      strneq
    cbz     w0, .not_info

    adrp    x0, msg_info
    add     x0, x0, :lo12:msg_info
    mov     x1, msg_info_len
    bl      write
    mov     w0, #1
    ldp     x29, x30, [sp], #16
    ret

.not_info:
    mov     w0, #0
    ldp     x29, x30, [sp], #16
    ret

// strneq: compare first w2 chars of x0 and x1
// Returns w0=1 if equal, 0 if not
strneq:
    cbz     w2, .strneq_eq
.strneq_loop:
    ldrb    w3, [x0], #1
    ldrb    w4, [x1], #1
    cmp     w3, w4
    b.ne    .strneq_ne
    sub     w2, w2, #1
    cbnz    w2, .strneq_loop
.strneq_eq:
    mov     w0, #1
    ret
.strneq_ne:
    mov     w0, #0
    ret

// strlen: x0=string, returns length in x0
strlen:
    mov     x1, x0
.strlen_loop:
    ldrb    w2, [x0], #1
    cbnz    w2, .strlen_loop
    sub     x0, x0, x1
    sub     x0, x0, #1         // don't count null
    ret

// ---- Data ----
.section .rodata

banner:
    .ascii "\n"
    .ascii "============================\n"
    .ascii "  zCore Shell v0.1\n"
    .ascii "  Custom userspace on zCore\n"
    .ascii "============================\n"
    .ascii "\n"
banner_len = . - banner

prompt:
    .ascii "zcore> "
prompt_len = . - prompt

cmd_echo:
    .ascii "echo "
cmd_help:
    .ascii "help"
cmd_hello:
    .ascii "hello"
cmd_info:
    .ascii "info"

msg_unknown:
    .ascii "Unknown command. Type 'help' for available commands.\n"
msg_unknown_len = . - msg_unknown

msg_help:
    .ascii "Available commands:\n"
    .ascii "  help        - Show this help\n"
    .ascii "  hello       - Say hello\n"
    .ascii "  info        - Show system info\n"
    .ascii "  echo <text> - Echo text back\n"
msg_help_len = . - msg_help

msg_hello_reply:
    .ascii "Hello! Welcome to zCore userspace!\n"
msg_hello_reply_len = . - msg_hello_reply

msg_info:
    .ascii "zCore OS - aarch64\n"
    .ascii "  Kernel: zCore (Rust)\n"
    .ascii "  Arch:   AArch64\n"
    .ascii "  Userspace: Custom ELF (no prebuilt)\n"
    .ascii "  Syscalls: debug_write, debug_read, nanosleep\n"
msg_info_len = . - msg_info

.section .data
line_buf:
    .space 128, 0
