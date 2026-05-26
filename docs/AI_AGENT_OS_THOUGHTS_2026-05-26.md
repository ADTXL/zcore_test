# AI 时代的 OS 应该是什么样的：AgentOS 设计思考

> 记录时间：2026-05-26 19:47 GMT+8  
> 背景：围绕 `ADTXL/zcore_test` / zCore 项目，讨论如何在 AI 时代开发一个适配 AI Agent 的 OS。

## 核心判断

AI 时代的 OS 不应该只是“能跑 AI 应用的 OS”，而应该变成一种新的 **Agent 运行与监管底座**。

传统 OS 的核心问题是：

> 怎么让人写的程序安全、高效地使用 CPU、内存、文件、网络和设备？

AI 时代 OS 的核心问题会变成：

> 怎么让具有一定自主性的 Agent，在可控、可审计、可协作的边界内使用计算资源、工具、数据和外部世界？

这两个问题不一样。

## 1. Agent 应该成为 OS 的一等公民

传统 OS 的一等公民是：

```plain text
Process / Thread / File / Socket / Device
```

AI 时代 OS 里，应该多一个一等公民：

```plain text
Agent
```

Agent 不只是普通进程。它应该有：

```plain text
AgentId
身份 / 角色
任务目标
权限集合
上下文状态
预算限制
工具能力
风险等级
审计轨迹
人类监督策略
```

比如一个 Agent 不只是简单的 `/bin/task_worker`，而是：

```plain text
Agent {
    name: "代码修改助手",
    goal: "修复驱动初始化 bug",
    allowed_tools: ["read_file", "edit_file", "run_test"],
    denied_tools: ["git_push", "send_message", "delete_project"],
    budget: {
        cpu_time: ...,
        tokens: ...,
        file_write_scope: "drivers/net/",
    },
    audit: enabled,
    human_approval: ["external_send", "delete", "privilege_escalation"]
}
```

也就是说，OS 不只知道“这个进程是谁”，还知道：

> 这个 Agent 打算做什么、能做什么、不能做什么、做过什么。

这是传统 OS 没有的。

## 2. 权限模型要从 UID/GID 走向 Capability

Linux 的权限模型本质上还是：

```plain text
用户是谁？
属于哪个组？
有没有 root？
```

但 AI Agent 的风险不是“它属于哪个用户”，而是：

```plain text
它能不能读这个文件？
能不能发网络请求？
能不能执行 shell？
能不能调用摄像头？
能不能发消息？
能不能改系统配置？
能不能创建子 Agent？
```

所以 AI OS 应该更像 Zircon/Fuchsia 这一类 capability OS。

权限不是全局身份推导出来的，而是明确授予：

```plain text
你拿到了某个 capability handle，才可以做某件事。
```

例如：

```plain text
FileReadCapability("/project/src")
FileWriteCapability("/project/src/drivers")
NetworkCapability("api.openai.com")
ShellCapability(false)
MessageSendCapability(requires_human_approval)
DeviceCapability("camera", denied)
```

这样 Agent 天然被关在一个能力笼子里。

这也是 `zcore_test` 项目有潜力的地方：Zircon-style handle / rights 模型比 Linux 传统权限模型更适合 AgentOS。

## 3. OS 要内建审计，而不是事后看日志

AI Agent 的问题不是“会不会犯错”，而是：

> 它犯错时，我们能不能知道它为什么这么做、哪一步开始偏了、它到底动了什么？

所以 AI OS 需要原生审计链路。

传统日志通常是应用自己打的：

```plain text
app.log
system.log
dmesg
```

但 AgentOS 里的 audit 应该是 OS 级别的：

```plain text
Agent A received task X
Agent A requested capability Y
Policy allowed/denied
Agent A called Tool Z
Tool Z touched file F
Agent A spawned Agent B
Agent B sent message M
Human approved action H
```

这不是普通 log，而是行为账本。

理想情况下，每个重要动作都有结构化审计事件：

```plain text
AuditEvent {
    agent_id,
    parent_agent_id,
    action,
    object,
    capability_used,
    policy_result,
    risk_level,
    timestamp,
    causal_chain_id,
}
```

这样出了问题可以回放：

```plain text
任务目标 → 推理计划 → 工具调用 → 文件修改 → 外部动作
```

这对 AI 时代很关键。

## 4. IPC 要从 byte stream 变成语义消息

传统 OS 的 IPC 很多还是：

```plain text
pipe
socket
shared memory
signal
```

底层强大，但语义贫乏。

Agent 之间通信时，OS 至少应该能理解一些元信息：

```plain text
谁发的？
发给谁？
意图是什么？
风险等级？
附带了哪些 capability？
是否请求外部动作？
是否需要人类确认？
```

所以 AI OS 里的 IPC 应该有一种结构化 Agent Message：

```plain text
AgentMessage {
    from,
    to,
    intent,
    risk,
    payload,
    attached_capabilities,
    audit_tag,
}
```

例如：

```plain text
PlannerAgent -> CodeAgent:
{
    intent: "modify_code",
    target: "drivers/virtio/net.rs",
    constraints: ["do_not_change_public_api"],
    attached_capabilities: ["read_project", "write_drivers"],
}
```

OS 不需要理解自然语言内容，但应该理解 **权限、来源、目标、风险、capability 传递**。

这会让 Agent 协作更安全。

## 5. Tool 调用应该被 OS 管起来

现在大多数 Agent 框架的问题是：工具调用基本是应用层自己管。

比如：

```plain text
read_file()
write_file()
run_shell()
send_email()
browser_open()
```

这些工具一旦暴露给 Agent，危险边界就很模糊。

AI OS 应该把 Tool Service 做成系统级能力：

```plain text
Agent
  → Tool Service
      → Policy Check
      → Capability Check
      → Audit Log
      → Real Action
```

比如 Agent 想执行 shell：

```plain text
Agent requests: run_shell("rm -rf build/")
```

OS 应该能判断：

```plain text
这个 Agent 有没有 ShellCapability？
这个命令是否 destructive？
是否限定 working directory？
是否需要 human approval？
是否记录完整 stdout/stderr？
```

最后决定：

```plain text
allow / deny / ask-human / sandbox-run
```

这才像 AI 时代的 OS。

## 6. 人类监督应该是内核/系统能力，不是 UI 补丁

AI Agent 的危险动作不能只靠“应用弹窗”。

人类监督应该进入系统设计：

```plain text
Policy result = AskHuman
```

例如：

```plain text
Agent wants to:
- send message to external contact
- delete project directory
- push code to remote
- access private file
- make payment
- change system config
```

OS 可以暂停这个 action，把请求送到 Supervisor：

```plain text
PendingApproval {
    agent,
    action,
    reason,
    risk,
    diff,
    rollback_plan,
}
```

用户批准后，capability 才临时发放。

这比现在 Agent 框架里的 “Are you sure?” 更系统化。

## 7. 资源调度要包含 token、上下文和注意力

传统 OS 调度：

```plain text
CPU time
memory
IO
network bandwidth
priority
```

AI 时代还要调度：

```plain text
token budget
context window
model quota
tool-call budget
外部动作次数
人类注意力预算
```

比如：

```plain text
Agent A 每小时最多 100k tokens
Agent B 只能调用模型 10 次
Agent C 不能发起网络搜索
Agent D 的上下文只能访问项目 A
```

这其实是新的资源管理问题。

未来 OS 的 scheduler 可能不只是调度线程，还要调度：

```plain text
Agent task graph
LLM inference requests
Tool call queues
Human approval queue
```

也就是说，AI OS 的资源模型会比传统 OS 更高层。

## 8. 上下文成为系统资源

传统 OS 管理内存页、文件、fd。AI OS 还要管理 context。

Agent 的上下文不是随便一堆 prompt，它应该是系统资源：

```plain text
ContextObject
MemoryObject
ConversationObject
KnowledgeObject
TraceObject
```

这些对象要有：

```plain text
owner
access rights
lifetime
sensitivity level
retention policy
summary policy
share policy
```

例如，一个 Agent 可以读取“项目源码上下文”，但不能读取“用户私人聊天上下文”。

这不是应用层随手拼 prompt 能解决的。

## 9. 隔离模型要面向“不可信自主行为”

传统 OS 主要防：

```plain text
恶意程序
buggy 程序
越权用户
外部攻击
```

AI OS 还要防：

```plain text
目标误解
prompt injection
工具滥用
长链路失控
Agent 之间互相诱导
数据越界传播
能力扩散
```

所以隔离边界要更细。

例如：

```plain text
Planner Agent 只能规划，不能执行。
Code Agent 只能改指定目录。
Test Agent 只能运行测试，不能写源码。
Messenger Agent 只能草拟消息，不能直接发送。
Supervisor Agent 可以审批，但不能绕过审计。
```

这就像把 Agent 拆成不同安全域。

## 10. 一版 AI OS 架构草图

```plain text
+--------------------------------------------------+
| Human Supervisor / Console                       |
+--------------------------------------------------+
| Agent Manager                                    |
| - Agent lifecycle                                |
| - Task graph                                     |
| - Parent/child agent relation                    |
+--------------------------------------------------+
| Policy Engine        | Audit Engine              |
| - allow/deny/ask     | - trace/replay            |
| - risk classification| - causality chain         |
+--------------------------------------------------+
| Capability Manager                              |
| - file/net/tool/device/model capabilities        |
| - delegation / revocation                        |
+--------------------------------------------------+
| Agent IPC Layer                                  |
| - structured messages                            |
| - capability passing                             |
| - context VMO/shared memory                      |
+--------------------------------------------------+
| Model Runtime Service | Tool Services            |
| - LLM request queue   | - file/shell/net/device  |
| - token budget        | - browser/message/etc    |
+--------------------------------------------------+
| Microkernel / HAL                                |
| - process/thread/job                             |
| - memory/VMO                                    |
| - channel/port                                  |
| - interrupt/timer/device                         |
+--------------------------------------------------+
| Hardware / Virtual Machine                       |
+--------------------------------------------------+
```

在 `zcore_test` 上，可以映射成：

```plain text
zircon-object     → Agent/Capability/Policy 的基础对象
zircon-syscall    → capability enforcement / audit hook
Channel/Port      → Agent IPC
Job/Process       → Agent 生命周期
VMO/VMAR          → Context / shared memory
kernel-hal        → ARM64 bare-metal 支撑
zCore             → 启动、调度、系统 glue
```

## 11. 最关键的一句话

AI 时代 OS 的核心不是“更快地跑 AI”，而是：

> **让具有自主性的智能体，在可限制、可解释、可审计、可撤销的系统边界内行动。**

传统 OS 管的是程序。  
AI OS 管的是 **行动主体**。

这个差别很大。

## 12. 对 `zcore_test` 项目的建议定位

建议不要一开始直接叫它：

```plain text
AI Operating System
```

一开始这个词太大。

可以先定位成：

```plain text
Agent-native Microkernel Runtime
```

或者中文：

```plain text
面向 AI Agent 的微内核运行环境
```

第一阶段 slogan 可以是：

> 基于 Zircon-style capability 的 AgentOS 原型：让 Agent 的权限、通信、工具调用和审计成为 OS 原生能力。

这个方向比“在 Linux 上套一层 Agent 框架”更有研究价值，也更匹配 `zcore_test` 当前技术路线。
