# zCore 自定义用户态开发进展

## 当前状态：✅ 交互式 Shell 已运行

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

## 架构概览

```
┌─────────────────────────────────────────────────┐
│                  QEMU aarch64                    │
│  ┌───────────────────────────────────────────┐  │
│  │           UEFI (QEMU_EFI.fd)              │  │
│  │              ↓                            │  │
│  │         rayboot (bootaa64.efi)            │  │
│  │              ↓                            │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │       zCore Kernel (Rust)           │  │  │
│  │  │  ┌───────────────────────────────┐  │  │  │
│  │  │  │  kernel-hal (aarch64 HAL)     │  │  │  │
│  │  │  │  - GIC-400 中断控制器         │  │  │  │
│  │  │  │  - PL011 UART               │  │  │  │
│  │  │  │  - Sv39/Sv48 页表           │  │  │  │
│  │  │  │  - 调度器 / 进程管理         │  │  │  │
│  │  │  └───────────────────────────────┘  │  │  │
│  │  │  ┌───────────────────────────────┐  │  │  │
│  │  │  │  zircon-object / syscall      │  │  │  │
│  │  │  │  - Process / Thread / VMAR    │  │  │  │
│  │  │  │  - Channel / Handle           │  │  │  │
│  │  │  │  - 调试/时间 syscall          │  │  │  │
│  │  │  └───────────────────────────────┘  │  │  │
│  │  │  ┌───────────────────────────────┐  │  │  │
│  │  │  │  simple_init loader           │  │  │  │
│  │  │  │  直接加载 ELF，不依赖 prebuilt│  │  │  │
│  │  │  └───────────────────────────────┘  │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │              ↓ (svc #0)                   │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  用户态程序 (shell.elf)             │  │  │
│  │  │  - aarch64 汇编实现                │  │  │
│  │  │  - 直接使用 syscall (svc #0)       │  │  │
│  │  │  - 无 prebuilt 依赖                │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## 已实现的系统调用

| Syscall # | 名称 | 功能 | 状态 |
|-----------|------|------|------|
| 8 | nanosleep | 休眠 | ✅ |
| 95 | debug_read | 串口读取 | ✅ |
| 96 | debug_write | 串口写入 | ✅ |

## 已完成的工作

### 平台清理
- ✅ 移除 x86/x86_64 平台代码
- ✅ 移除 riscv/riscv64 平台代码
- ✅ 项目仅保留 aarch64 支持

### 用户态开发
- ✅ 创建 `loader/src/simple_init.rs` - 自定义 ELF 加载器
- ✅ 创建 `user/hello.S` - 最小用户态程序
- ✅ 创建 `user/shell.S` - 交互式 Shell
- ✅ 修改 `sys_debug_read` 支持 handle=0 直接读取
- ✅ virtio 块设备初始化改为可选

### 验证结果
- ✅ QEMU 启动成功
- ✅ Shell 显示 banner 和提示符
- ✅ help/hello/info/echo 命令正常工作
- ✅ 退格键编辑正常

## 待完成工作

### 短期（下一步）
- [ ] 实现 `process_create` syscall 支持多进程
- [ ] 实现 `handle_close` / `handle_duplicate` syscall
- [ ] 添加更多内置命令（ps, meminfo 等）

### 中期
- [ ] 用 Rust 编写用户态程序（需要 no_std 环境）
- [ ] 实现基本的文件系统操作 syscall
- [ ] 支持运行独立的 ELF 程序

### 长期
- [ ] 实现完整的 Zircon syscall 子集
- [ ] 支持 POSIX 兼容的 Linux 程序
- [ ] 网络支持

## 构建和运行

```bash
# 构建用户态程序
cd user && make

# 构建内核
cargo +nightly-2022-08-05 build --package zcore \
  --no-default-features --features "zircon" \
  --target zCore/aarch64.json \
  -Z build-std=core,alloc \
  -Z build-std-features=compiler-builtins-mem \
  --release

# 复制内核到启动盘
cp target/aarch64/release/zcore zCore/disk/os

# 运行
qemu-system-aarch64 \
  -m 1G \
  -kernel target/aarch64/release/zcore.bin \
  -append "LOG=warn" \
  -display none -no-reboot -nographic \
  -cpu cortex-a72 \
  -bios prebuilt/firmware/aarch64/QEMU_EFI.fd \
  -machine virt \
  -hda fat:rw:zCore/disk
```

## 文件结构

```
user/
├── hello.S          # 最小用户态程序
├── shell.S          # 交互式 Shell（当前使用）
├── linker.ld        # 链接脚本
└── Makefile         # 构建脚本

loader/src/
└── simple_init.rs   # 自定义 ELF 加载器

zircon-syscall/src/
└── debug.rs         # debug_read/write syscall 实现
```
