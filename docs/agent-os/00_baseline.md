# AgentOS Phase 0：工程基线复现

> 目标：先确认当前 `zcore_test` 可以稳定构建、启动、调试，然后再开始 AgentOS 原型开发。

## 1. 当前目标

第一步不急着改内核逻辑，先完成三件事：

1. 固化开发环境；
2. 跑通 ARM64 QEMU 启动链路；
3. 记录现有 zCore/Zircon 启动、对象、syscall、IPC 能力，为 AgentOS 设计找落点。

## 2. 为什么第一步要做基线复现？

AgentOS 后面会涉及：

- Agent 生命周期；
- Job / Process / Thread 扩展；
- Channel IPC 协议；
- Capability / Policy / Audit；
- Tool Service；
- LLM Runtime Service。

这些都要落在当前系统之上。如果现在启动链路、构建环境、QEMU 参数不稳定，后面会很难判断问题来自 AgentOS 新代码，还是来自旧工程基线。

## 3. Phase 0 Checklist

### 3.1 环境检查

- [ ] 确认当前分支；
- [ ] 确认 Rust toolchain；
- [ ] 确认 QEMU 版本；
- [ ] 确认 cargo alias 可用；
- [ ] 确认 docker 环境是否可用，可选。

### 3.2 构建检查

- [ ] `cargo check` 或项目推荐检查命令；
- [ ] `cargo qemu --arch aarch64` 能启动；
- [ ] `cargo qemu --arch aarch64 --firmware atf` 状态明确；
- [ ] 记录成功/失败日志。

### 3.3 启动链路记录

- [ ] UEFI / ATF 入口；
- [ ] zCore kernel entry；
- [ ] memory init；
- [ ] kernel-hal init；
- [ ] Zircon userboot；
- [ ] console 输出。

### 3.4 AgentOS 落点调查

- [ ] Job / Process / Thread 当前实现位置；
- [ ] Channel / Port IPC 当前实现位置；
- [ ] JobPolicy 当前能力；
- [ ] syscall dispatch 入口；
- [ ] audit hook 可能插入点。

## 4. 第一条可执行命令

建议从最小环境确认开始：

```bash
cd /root/.openclaw/workspace/zcore_test
git status --short
git branch --show-current
rustc --version
cargo --version
qemu-system-aarch64 --version
```

然后再跑：

```bash
cargo qemu --arch aarch64
```

如果 QEMU 可以进入 console，Phase 0 的核心基线就成立。

## 5. Phase 0 完成标准

满足以下条件，就可以进入 Phase 1：

- 当前分支、commit、工具链、QEMU 版本已记录；
- 至少一种 ARM64 启动路径可复现；
- 启动失败时有明确日志和下一步定位方向；
- 已明确 AgentOS 第一批代码落点：Agent metadata、Channel message、Policy/Audit hook。
