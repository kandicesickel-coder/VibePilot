# VibePilot

> **本地优先的 Agent 研发记忆与工作流层** — 让任何 Agent 进入你的项目时，都像一个记得上次踩坑的老同事。

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

---

## 定位

VibePilot **不做"又一个 AI 编程聊天工具"**，而做：

> **面向 Claude Code / Codex / Cursor / Windsurf / Cline 等 Agent 的本地项目记忆与工程纪律层。**

真正的产品资产是：
- **Learning Cards**（失败路径 / 成功路径 / 根因 / 验证证据）
- **Context Packs**（最小上下文包，按 token 预算压缩）
- **Workflow Gates**（Spec → Plan → Build → Test → Review → Ship 工程纪律）

---

## 核心功能（MVP）

1. **项目扫描** — 识别 repo 结构、命令、规则文件健康度
2. **Agent 适配** — Claude Code + Codex 双后端，捕获 session/transcript/tool call
3. **Learning Cards** — 会话结束自动生成经验卡，用户确认后入库
4. **Context Packs** — 规则前缀固定 + 动态内容后置，利于 Prompt Caching
5. **MCP Server** — 暴露 `search_memory` / `get_context_pack` / `record_outcome` / `get_project_rules`

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面端 | Tauri 2 + React 18 + TypeScript |
| 移动端 | Capacitor + React（远程控制） |
| 数据库 | SQLite（Rust 侧 rusqlite/sqlx）|
| AI 接入 | MCP Server（对外）+ Agent Adapter（内部）|
| UI | Tailwind + shadcn/ui |

---

## 快速开始

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm dev

# 构建生产版本
pnpm build
```

---

## 项目结构

```
VibePilot/
├── apps/
│   ├── desktop/          # Tauri 2 + React（主应用）
│   ├── mobile/           # Capacitor + React（远程控制）
│   └── web-console/      # 可选：Team/远程控制
├── packages/
│   ├── ui/               # 共享 React 组件
│   ├── core/             # 类型 / Agent 抽象 / Context Pack
│   ├── prompts/          # Skills / Workflow Templates
│   ├── mcp-server/       # 对外 MCP Server
│   └── runner-protocol/   # Desktop ↔ Mobile 通信协议
└── docs/
    ├── spec/             # 项目规格
    ├── adr/               # 架构决策记录
    └── skills/            # Agent Skills（23 个）
```

---

## 市场差异化

| 竞品 | 差异 |
|------|------|
| Cursor / Windsurf | 记忆服务自己；VibePilot 服务所有 Agent |
| Augment / Copilot Cloud | 本地优先，隐私敏感场景更适合 |
| Pieces | 宽泛记忆；VibePilot 专注"失败路径 + 验证证据" |
| Aider | 学习 repo map 思路；VibePilot 加跨工具经验沉淀 |

---

## 参考研究

- AGENTS.md 与 -28.64% 中位运行时间和 -16.58% 输出 token 相关（[arXiv:2601.20404](https://arxiv.org/abs/2601.20404)）
- 不同 Agent 在不同任务类型表现不同（[arXiv:2602.08915](https://arxiv.org/abs/2602.08915)）

---

## License

MIT