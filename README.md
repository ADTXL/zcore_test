# zCore (aarch64)

基于 Zircon 微内核的 Rust 实现，专注于 aarch64 平台。

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/ADTXL/zcore_test.git
cd zcore_test

# 编译并运行
cargo qemu --arch aarch64
```

启动后进入交互式 Shell：

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
```

## 架构

```
┌─────────────────────────────────────┐
│          QEMU aarch64               │
│  ┌───────────────────────────────┐  │
│  │    UEFI → rayboot             │  │
│  │         ↓                     │  │
│  │    zCore Kernel (Rust)        │  │
│  │    ├── kernel-hal             │  │
│  │    ├── zircon-object          │  │
│  │    ├── zircon-syscall         │  │
│  │    └── simple_init            │  │
│  │         ↓ (svc #0)            │  │
│  │    shell.elf (用户态)          │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

## 系统调用

| # | 名称 | 功能 |
|---|------|------|
| 8 | nanosleep | 休眠 |
| 95 | debug_read | 串口读取 |
| 96 | debug_write | 串口写入 |

## 项目结构

```
zcore_test/
├── user/              # 用户态程序 (shell.S, hello.S)
├── loader/            # ELF 加载器 (simple_init.rs)
├── zircon-syscall/    # 系统调用实现
├── zircon-object/     # 内核对象 (Process, Thread, VMAR)
├── kernel-hal/        # aarch64 硬件抽象层
├── drivers/           # 设备驱动 (GIC, PL011, virtio)
├── zCore/             # 内核主体
├── docs/              # 文档
└── config/            # 配置文件
```

## 开发

### 构建用户态程序
```bash
cd user && make
```

### 构建内核
```bash
cargo +nightly-2022-08-05 build --package zcore \
  --no-default-features --features "zircon" \
  --target zCore/aarch64.json \
  -Z build-std=core,alloc \
  -Z build-std-features=compiler-builtins-mem \
  --release
```

### 运行
```bash
cargo qemu --arch aarch64
```

## 文档

- [用户态开发文档](docs/userspace-dev.md)

## 平台支持

- ✅ aarch64 (AArch64)
- ❌ x86_64 (已移除)
- ❌ riscv64 (已移除)

## 许可证

MIT License
