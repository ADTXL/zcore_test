# zCore 自定义用户态开发文档

## 当前状态：✅ 交互式 Shell 运行中

```
============================
  zCore Shell v0.1
  Custom userspace on zCore
============================

zcore> help
Available commands:
  help        - Show this help
  hello       - Say hello
  info        - Show system info
  echo <text> - Echo text back
zcore> hello
Hello! Welcome to zCore userspace!
```

## 启动方式

```bash
cargo qemu --arch aarch64
```

## 架构

```
UEFI (QEMU_EFI.fd)
    ↓
rayboot (bootaa64.efi)
    ↓
zCore Kernel (Rust, aarch64)
    ├── kernel-hal: GIC-400, PL011 UART, Sv39 页表
    ├── zircon-object: Process/Thread/VMAR
    ├── zircon-syscall: debug_write(96), debug_read(95), nanosleep(8)
    └── simple_init: 加载用户态 ELF
    ↓ (svc #0)
shell.elf (aarch64 汇编)
```

## 已实现的系统调用

| Syscall # | 名称 | 功能 | 参数 |
|-----------|------|------|------|
| 8 | nanosleep | 休眠 | x0=deadline(ns), x16=8 |
| 95 | debug_read | 串口读取 | x0=handle(0), x1=buf, x2=len_ptr, x16=95 |
| 96 | debug_write | 串口写入 | x0=buf, x1=len, x16=96 |

## 用户态程序

### user/shell.S - 交互式 Shell
- aarch64 汇编实现
- 内置命令: help, hello, info, echo
- 支持退格键编辑
- 通过 svc #0 直接调用 syscall

### user/hello.S - 最小示例
- 输出 "Hello from zCore userspace!" 后休眠

### 构建
```bash
cd user && make
```

## 内核改动

### loader/src/simple_init.rs
- 自定义 ELF 加载器
- 替代 prebuilt 的 userboot/ZBI/vDSO
- 直接加载 ELF 到进程地址空间

### zircon-syscall/src/debug.rs
- sys_debug_read: handle=0 时跳过 Resource 验证
- 允许简单用户态程序直接读取串口

### kernel-hal/src/bare/arch/aarch64/drivers.rs
- virtio 块设备初始化改为可选（失败只 warn 不 panic）

## 项目结构

```
zcore_test/
├── user/              # 用户态程序
│   ├── shell.S        # 交互式 Shell
│   ├── hello.S        # 最小示例
│   ├── linker.ld      # 链接脚本
│   └── Makefile
├── loader/src/
│   ├── simple_init.rs # ELF 加载器
│   └── lib.rs
├── zircon-syscall/src/
│   └── debug.rs       # debug_read/write
├── kernel-hal/src/bare/arch/aarch64/
│   ├── drivers.rs     # 设备驱动初始化
│   ├── trap.rs        # 中断处理
│   └── vm.rs          # 虚拟内存
├── zCore/src/
│   ├── main.rs        # 内核入口
│   └── fs.rs
├── docs/
│   └── userspace-dev.md  # 本文档
└── config/
    └── machine-features.toml
```

## 平台支持

- ✅ 仅支持 aarch64
- ❌ 已移除 x86_64 和 riscv64

## 已删除的内容

- prebuilt/ (Fuchsia 预构建二进制, 60MB)
- zircon/src/zircon.rs (旧的 userboot 加载器)
- rboot/ (x86 UEFI bootloader)
- riscv/ 平台代码
- 各种板级文档 (D1, fu740, visionfive)

## 待完成

- [ ] process_create syscall (多进程)
- [ ] 更多内置命令 (ps, meminfo)
- [ ] Rust 编写的用户态程序
- [ ] 文件系统操作 syscall
