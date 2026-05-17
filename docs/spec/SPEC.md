# VibePilot — 本地优先 Agent 研发记忆与工作流层

> **定位**：面向 Claude Code / Codex / Cursor / Windsurf / Cline 等 Agent 的本地项目记忆与工程纪律层。

---

## 状态

- **版本**：v3.0（市场定位精炼版）
- **制定日期**：2026-05-18
- **下一步**：T1 Monorepo 初始化

---

## 核心定位（一句话）

> **让任何 Agent 进入你的项目时，都像一个记得上次踩坑的老同事。**

---

## ASSUMPTIONS（最终确认）

```
ASSUMPTIONS I'M MAKING（v3 最终确认）：

1. 桌面端：Tauri 2 + React，Rust 侧处理 SQLite/文件/git/进程
2. 移动端：Capacitor + React，远程控制桌面 daemon，不本地执行
3. AI 底层：Claude Code + Codex（双选二），MCP Server 对外暴露
4. 记忆核心：Learning Cards（失败路径/成功路径/根因/验证证据），非聊天记录
5. MVP：桌面单用户，不做 Team，不做移动端本地执行
6. 数据库：Rust 侧 SQLite + WAL + FTS5（MVP）
7. 差异化：跨工具记忆 + 工程证据链 + 成本可解释
→ 如有改动，立即更新此文件。
```

---

## 市场定位（竞品对比）

### 核心洞察

| 洞察 | 说明 |
|------|------|
| **方向已市场验证** | Cursor/Windsurf 做 Memories/Rules；Claude Code/Codex 支持项目指令；Augment/Pieces 强调上下文和记忆 |
| **记忆仍偏"事实/规则/摘要"** | 大多数工具没有做到：保存失败尝试、成功路径、验证证据、成本数据、下次规避策略 |
| **AGENTS.md 有实证价值** | 2026 年研究：AGENTS.md 与 -28.64% 中位运行时间 和 -16.58% 输出 token 相关（arXiv:2601.20404） |
| **无单个 Agent 通吃所有任务** | 不同 Agent 在不同任务类型表现不同：Claude Code 文档/功能强，Cursor fix 任务强，Codex 整体稳定（arXiv:2602.08915） |
| **MCP 是生态入口** | 但有安全边界：敏感工具保留 human-in-the-loop、展示输入、审计、超时（MCP Tools spec 2025-06-18） |

### 竞品对比表

| 项目 | 类别 | 强项 | 短板 | VibePilot 差异 |
|------|------|------|------|---------------|
| [Cursor](https://docs.cursor.com/chat/codebase) | AI IDE | 体验成熟、代码库索引、Rules、Memories、Background Agents | 强绑定 Cursor IDE；背景 Agent 依赖云环境 | Cursor 是写代码入口，VibePilot 做跨工具编排和项目记忆中枢 |
| [Windsurf](https://docs.windsurf.com/windsurf/cascade/memories) | AI IDE | Cascade、Rules、Workflows、Skills、Memories | 仍是 IDE 内能力；重要知识应写入 Rules/AGENTS.md | Windsurf 证明"记忆+规则+工作流"方向正确，VibePilot 更工具无关 |
| [Claude Code](https://code.claude.com/docs/en/hooks) | CLI Agent | Hooks、MCP、Skills、Subagents、CLAUDE.md 生态 | 主要是执行器，不是跨 Agent 项目记忆平台 | VibePilot 捕获它的过程、证据、失败路径 |
| [OpenAI Codex](https://developers.openai.com/codex/cloud) | CLI/Cloud Agent | 本地 CLI + 云任务 + GitHub PR 工作流 | 记忆和跨工具经验沉淀不是核心产品形态 | VibePilot 成为 Codex 的外部 context/memory/quality layer |
| [Copilot Cloud Agent](https://docs.github.com/en/copilot/concepts/agents/cloud-agent/about-cloud-agent) | GitHub 原生 | Actions 环境、分支、PR、团队协作天然强 | 本地项目过程、跨模型成本控制弱 | 它解决"从 issue 到 PR"；VibePilot 解决"开发过程如何不重复犯错" |
| [Aider](https://aider.chat/docs/repomap.html) | CLI coding assistant | Repo map 很优秀：符号+函数签名+图排序压缩上下文 | CLI 体验偏工程师；长期项目记忆不是核心 | 直接学习它的 repo map 思路 |
| [Continue](https://docs.continue.dev/customize/custom-providers) | 开源 IDE Agent 框架 | Context Providers、MCP、repo-map、diff 很灵活 | 更多是 IDE 插件/框架，不是产品级记忆操作系统 | 借鉴"上下文来源插件化" |
| [Cline](https://docs.cline.bot/cline-overview) | 开源/商业 Agent | Plan & Act、Checkpoints、MCP、Memory Bank、人工审批 | 重心仍在单个 Agent 执行；记忆多偏文档方法论 | 把 Memory Bank 产品化、结构化、可检索 |
| [Plandex](https://docs.plandex.ai/core-concepts/context-management/) | CLI 长任务 Agent | Context versioning、plan 分支、rewind、project map 很强 | CLI 心智重；商业体验和跨工具生态弱 | 学习"计划版本控制"和"上下文版本控制" |
| [Augment Context Engine](https://docs.augmentcode.com/context-services/mcp/overview) | 上下文引擎 | 实时索引、跨 repo、commit history，通过 MCP 接入各工具 | 商业闭源，按查询消耗 credits；不是完整研发工作流 | VibePilot 在"上下文引擎"上的直接强竞品 |
| [Pieces](https://pieces.app/use-cases/software-engineers) | OS 级长期记忆 | 本地优先、跨 IDE/浏览器/终端、长期记忆、MCP 接入 | 范围宽，不专注"代码任务闭环、失败路径、验证证据" | VibePilot 在"长期记忆"上的最重要参考 |

---

## VibePilot 的真正差异化

### 比竞品强的地方

1. **跨工具**：Cursor/Windsurf 的记忆主要服务自己；VibePilot 服务所有 Agent
2. **本地优先**：比 Augment/Copilot Cloud 更适合隐私敏感个人和小团队
3. **经验卡而非聊天记录**：比 Pieces 的宽泛记忆更适合代码研发
4. **工程证据链**：每个成功路径必须绑定 diff、命令、测试结果、失败尝试
5. **成本可解释**：显示本次 Context Pack 每一块为什么进入 prompt

### 比竞品弱的地方

1. 没有 IDE 入口优势，Cursor/Windsurf 更容易高频触达用户
2. 没有大模型原生控制权，Claude/Codex 的底层能力变化会影响你
3. 早期 repo map、索引、检索质量很难超过 Aider/Augment
4. 用户要理解"编排层"价值，比理解"AI 帮我写代码"更难
5. 若移动端也想完整执行，会被系统权限、依赖安装、文件系统、长任务拖垮

---

## Tech Stack

| 层级 | 技术选型 | 说明 |
|------|---------|------|
| 桌面端 | Tauri 2.x + React 18 + TypeScript | Rust 侧处理 SQLite/文件/git/进程 |
| 移动端 | Capacitor + React | 远程控制 desktop daemon，不本地执行 |
| UI | React + Tailwind + shadcn/ui | 跨平台共享 |
| 数据库 | SQLite + rusqlite/sqlx（Drizzle ORM）| Rust 原生绑定，无 Node runtime 依赖 |
| 加密 | SQLCipher（AES-256，商用）| MVP 暂不上 |
| 向量检索 | MVP 不上 | P1 评估 sqlite-vec / LanceDB |
| AI 接入 | MCP Server（对外）+ Agent Adapter（内部）| Claude Code SDK/CLI、Codex CLI、Responses API |

---

## Project Structure（Monorepo）

```
VibePilot/
├── apps/
│   ├── desktop/              # Tauri 2 + React（主应用）
│   │   └── src-tauri/        # Rust 侧
│   │       ├── commands/     # 文件/git/进程/SQLite/系统权限的 Tauri commands
│   │       ├── orchestrator/ # Agent session 编排
│   │       ├── storage/      # SQLite migrations + repository layer
│   │       └── sandbox/      # 命令白名单、权限、审计
│   ├── mobile/               # Capacitor + React（远程控制模式）
│   └── web-console/          # 可选：Team/远程控制
├── packages/
│   ├── ui/                   # 共享 React 组件
│   ├── core/                 # 任务/记忆/上下文/成本/Agent 抽象类型
│   ├── prompts/              # skills/workflow templates/context pack templates
│   ├── mcp-server/           # 对外暴露 VibePilot memory/repo/tools 的 MCP Server
│   └── runner-protocol/       # desktop ↔ mobile 通信协议
```

---

## Core Modules

| 模块 | 职责 | 优先级 |
|------|------|--------|
| **Project Scanner** | 识别 repo、包管理器、测试命令、AGENTS.md/CLAUDE.md、依赖、目录结构 | P0 |
| **Repo Map Engine** | 用 tree-sitter/语言解析提取符号/签名/依赖图，按 token 预算裁剪 | P0 |
| **Context Pack Engine** | 把静态规则+相关记忆+repo map+diff+目标文件组合成最小上下文包 | P0 |
| **Agent Adapter** | Claude Code SDK/CLI、Codex CLI/Responses API 的统一接口 | P0 |
| **Memory Engine** | 保存 Project Facts / Failed Attempts / Successful Paths / ADR / 验证证据（Learning Cards）| P0 |
| **Workflow Gate** | Spec → Plan → Build → Test → Review → Ship，每步有验收标准 | P0 |
| **Cost Engine** | 记录 input/output/reasoning/cached tokens、模型、耗时、缓存命中率 | P0 |
| **MCP Server** | 让 Claude/Codex 可调用 VibePilot 的记忆/任务/上下文/验证工具 | P1 |
| **Sandbox** | 命令白名单、权限、审计（Tauri shell/capability 模型）| P1 |

---

## MVP 功能（v3.0）

### 第一版只做 5 件事

1. **扫描项目，生成 repo map、命令清单、规则文件健康度**
2. **对接 Claude Code 和 Codex，捕获 session、prompt、tool call、diff、测试结果**
3. **每次任务结束生成 Learning Card：问题、失败路径、成功路径、验证证据**
4. **下次任务前生成 Context Pack：相关经验卡、相关文件、规则、验证命令**
5. **暴露 MCP 工具：search_memory、get_context_pack、record_outcome、get_project_rules**

### MCP Server 暴露的工具

```typescript
// VibePilot MCP Server 暴露的工具
interface VibePilotMCPTools {
  // 检索项目记忆
  search_memory(query: string, project_id: string): Promise<LearningCard[]>;

  // 获取当前上下文包
  get_context_pack(project_id: string, task_id?: string): Promise<ContextPack>;

  // 记录任务结果（用于生成 Learning Card）
  record_outcome(
    project_id: string,
    task_id: string,
    outcome: 'success' | 'failed' | 'partial',
    details: string
  ): Promise<void>;

  // 获取项目规则（AGENTS.md / CLAUDE.md 内容）
  get_project_rules(project_id: string): Promise<ProjectRules>;
}
```

---

## 数据库 Schema（SQLite + Drizzle ORM）

```sql
-- projects: 一个项目对应一个 git repo 或本地目录
CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  path TEXT NOT NULL UNIQUE,
  repo_url TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  last_session_at TEXT,
  total_token_cost_usd REAL DEFAULT 0
);

-- project_facts: 项目级知识（静态规则、架构决策）
CREATE TABLE project_facts (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id),
  category TEXT NOT NULL,
  content TEXT NOT NULL,
  source TEXT NOT NULL,
  created_at TEXT NOT NULL
);

-- learning_cards: 经验卡（核心差异化资产）
CREATE TABLE learning_cards (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id),
  type TEXT NOT NULL,
  title TEXT NOT NULL,
  trigger TEXT NOT NULL,
  body TEXT NOT NULL,
  token_cost_usd REAL,
  confirmed_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- sessions: Agent 会话
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id),
  backend_id TEXT NOT NULL,
  model TEXT NOT NULL,
  workflow_stage TEXT NOT NULL,
  started_at TEXT NOT NULL,
  finished_at TEXT,
  total_token_input INTEGER DEFAULT 0,
  total_token_output INTEGER DEFAULT 0,
  total_token_cached INTEGER DEFAULT 0,
  total_cost_usd REAL DEFAULT 0
);

-- token_usage: 每次 token 消耗明细
CREATE TABLE token_usage (
  id TEXT PRIMARY KEY,
  session_id TEXT REFERENCES sessions(id),
  project_id TEXT NOT NULL REFERENCES projects(id),
  model TEXT NOT NULL,
  input_tokens INTEGER NOT NULL,
  output_tokens INTEGER NOT NULL,
  cached_tokens INTEGER DEFAULT 0,
  reasoning_tokens INTEGER DEFAULT 0,
  cost_usd REAL NOT NULL,
  created_at TEXT NOT NULL
);

-- tasks: 任务
CREATE TABLE tasks (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id),
  session_id TEXT REFERENCES sessions(id),
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  acceptance_criteria TEXT NOT NULL,
  verification_method TEXT NOT NULL,
  priority TEXT NOT NULL DEFAULT 'medium',
  dependencies TEXT,
  phase TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  completed_at TEXT
);

-- verifications: 验证证据（绑定到 task）
CREATE TABLE verifications (
  id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(id),
  type TEXT NOT NULL,
  passed BOOLEAN NOT NULL,
  output TEXT NOT NULL,
  duration_ms INTEGER,
  created_at TEXT NOT NULL
);

-- repo_maps: 项目符号地图缓存
CREATE TABLE repo_maps (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(id),
  hash TEXT NOT NULL,
  symbols_json TEXT NOT NULL,
  dependencies_json TEXT NOT NULL,
  created_at TEXT NOT NULL
);

-- context_packs: 上下文包历史
CREATE TABLE context_packs (
  id TEXT PRIMARY KEY,
  session_id TEXT NOT NULL REFERENCES sessions(id),
  components_json TEXT NOT NULL,
  total_tokens INTEGER NOT NULL,
  cached_tokens INTEGER DEFAULT 0,
  cost_usd REAL NOT NULL,
  created_at TEXT NOT NULL
);
```

---

## Commands

```bash
# 桌面端开发
pnpm tauri dev           # 开发模式
pnpm tauri build         # 生产构建

# 移动端开发
pnpm cap sync            # Capacitor 同步
pnpm cap open ios        # 打开 iOS 项目

# 代码质量
pnpm lint                # ESLint + Prettier
pnpm typecheck           # TypeScript 类型检查

# 数据库（Rust 侧）
cargo run -- migrate     # 执行 SQLite 迁移

# 测试
pnpm test                # Vitest 单元测试
pnpm test:e2e            # Playwright E2E
```

---

## Code Style

TypeScript strict mode + React functional components only

```typescript
// 命名规范
const ComponentName = ({ props }) => {...}  // PascalCase 组件
const useHookName = () => {...}              // use 前缀 hooks
const CONSTANT_NAME = 'value'               // 全大写常量
const functionName = () => {...}            // camelCase 函数

// 分层规则
components/  → 仅做 UI 渲染和 props 透传，禁止业务逻辑
hooks/       → 业务逻辑，组合 useCase
lib/         → 纯工具函数，无 React 依赖
packages/core/ → 跨应用共享类型和接口

// 禁止
// - 任意 any
// - 在 components/ 下写业务逻辑
// - 直接 import Rust 侧模块（通过 Tauri invoke）
```

---

## Testing Strategy

| 层级 | 工具 | 覆盖率目标 |
|------|------|-----------|
| 单元测试 | Vitest | ≥80%（core/lib/hooks）|
| 组件测试 | React Testing Library | ≥60%（UI 组件）|
| E2E 测试 | Playwright | 关键路径 |

---

## Boundaries

**Always:**
- 每个非平凡改动前先写/更新 `docs/spec/*.md` 和 `docs/adr/*.md`
- 所有数据变更走 Drizzle ORM 迁移（Rust 侧）
- Token 消耗超阈值（$0.05/session）主动警告
- 每次 Agent 会话结束自动生成 Learning Card（pending review）
- 所有 Tauri commands 必须有 capability 声明

**Ask first:**
- 数据库 schema 变更
- 新增 AI 适配器
- 命令白名单变更
- 跨平台兼容决策

**Never:**
- 硬编码 API key（用 Tauri secure storage 或环境变量）
- 在 Tauri 主路径依赖 Node.js 原生包
- 直接操作 SQLite（绕过 Rust repository layer）
- 在非 docs/ 目录存储规格/ADR 文档
- 把聊天历史当核心资产（核心是 Learning Cards）

---

## Success Criteria（MVP）

### 项目扫描
- [ ] 用户选择本地目录 → 自动扫描 repo（语言/包管理/测试命令）
- [ ] 生成项目画像 + repo map + 命令清单 + 规则文件健康度

### Agent 会话
- [ ] 支持 Claude Code CLI（通过 SDK/CLI 捕获 session/transcript）
- [ ] 支持 Codex CLI（CLI 优先，Responses API 作为可计费增强路径）
- [ ] 捕获 prompt/tool call/diff/测试结果

### Learning Card 生成
- [ ] 会话结束自动生成 Learning Card（pending review）
- [ ] 包含：问题/失败路径/成功路径/验证证据/token 成本
- [ ] 用户确认后入库，下次相关任务优先检索

### Context Pack 生成
- [ ] 每次发给 Agent 前显示"本次 Context Pack 包含内容 + token 统计"
- [ ] 规则前缀固定 + 动态内容后置（利于 Anthropic/OpenAI Prompt Caching）

### MCP Server 暴露
- [ ] `search_memory` / `get_context_pack` / `record_outcome` / `get_project_rules` 四个工具可用
- [ ] Claude Code / Codex 可通过 MCP 调用 VibePilot 的项目记忆

---

## Open Questions（待确认）

| # | 问题 | 建议方案 |
|---|------|---------|
| O1 | 移动端远程控制协议 | WebSocket？gRPC？SSH 隧道？先专注桌面端 |
| O2 | Team 版数据同步 | 商业版 SQLite 加密 + 云端密钥管理；MVP 暂不考虑 |
| O3 | 定价模型精确边界 | Free（1 项目）/ Pro（无限项目 + 多模型）/ Team（协作） |

---

## 参考启发（已融入架构）

| 项目 | 启发点 | 来源 |
|------|--------|------|
| Aider repo map | 符号级代码地图 + 图排序裁剪 | https://aider.chat/docs/repomap.html |
| Continue context providers | 上下文拆成文件/代码/搜索/repo map/问题/终端等多来源 | https://docs.continue.dev |
| Plandex context versioning | 计划版本控制 + 上下文版本控制 | https://docs.plandex.ai |
| Augment Context Engine | 实时索引 + 跨 repo + MCP 接入 | https://docs.augmentcode.com |
| Pieces OS 级长期记忆 | 本地优先 + 跨工具 + 长期记忆 | https://pieces.app |
| Claude Code hooks/skills | 捕获和注入层 | https://code.claude.com/docs/en/hooks |
| Tauri capability model | 命令白名单 + 审计 | https://v2.tauri.app |
| AGENTS.md 研究（arXiv:2601.20404）| AGENTS.md 与 -28.64% 运行时间 和 -16.58% token 相关 | — |
| Agent 任务类型研究（arXiv:2602.08915）| 不同 Agent 在不同任务表现不同 | — |

---

## 文档结构

```
docs/
├── spec/                           # 项目规格
│   └── v3-mvp.md                   # 本文件
├── adr/                            # 架构决策记录
│   ├── 001-local-first-architecture.md
│   ├── 002-learning-cards-over-chat.md
│   ├── 003-context-pack-design.md
│   └── 004-multi-agent-adapter.md
└── skills/                          # Agent Skills（永久生效）
    └── ...                          # 23 个 skills 完整内容
```

---

## 版本历史

| 版本 | 日期 | 变更 |
|------|------|------|
| v1 | 2026-05-18 | 定位为"上下文编排器" |
| v2 | 2026-05-18 | 重新定位为"本地优先 Agent 研发操作系统"；明确移动端远程控制；修正 Rust 侧 SQLite |
| v3 | 2026-05-18 | 市场定位精炼：跨工具记忆中枢 + 经验卡差异化 + 5 个 MVP 功能 + MCP Server 暴露 + 竞品对比表 |