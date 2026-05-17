# ADR-004: Agent Adapter 多后端架构

## 状态
**Accepted** | 2026-05-18

## 背景
VibePilot 不绑定单一 AI 后端。用户可能同时使用 Claude Code 和 Codex，或在不同项目选择不同模型。需要统一接口，同时保留各后端的特殊能力。

## 决策

**Agent Adapter 采用"统一接口 + 后端特化"的适配器模式。**

```
┌─────────────────────────────────────────────┐
│           VibePilot Orchestrator            │
│     (Session lifecycle, cost tracking)      │
└────────────────┬────────────────────────────┘
                 │
     ┌──────────┴──────────┐
     ▼                      ▼
┌─────────────┐    ┌─────────────────┐
│ ClaudeCode   │    │     Codex       │
│  Adapter    │    │    Adapter     │
└─────────────┘    └─────────────────┘
     │                      │
     ▼                      ▼
 Claude Code CLI/SDK   Codex CLI/Responses API
 Anthropic API         OpenAI API
```

**AgentBackend 接口（`packages/core/src/agent.ts`）：**
```typescript
interface AgentBackend {
  id: AgentBackendId;  // 'claude-code' | 'codex' | 'gemini' | 'ollama'

  capabilities(): Promise<AgentCapabilities>;
  startSession(input: StartSessionInput): AsyncIterable<AgentEvent>;
  sendTurn(input: AgentTurnInput): AsyncIterable<AgentEvent>;
  cancel(sessionId: string): Promise<void>;
}
```

**后端优先级（MVP）：**
1. **Claude Code CLI** — 用户最熟悉，hooks/skills 生态完整
2. **Codex CLI** — OpenAI 官方，本地 CLI + Responses API

**P1 扩展：**
- Gemini（Google AI API）
- Ollama（本地模型，私有部署）

## 替代方案考虑

| 方案 | 劣势 | 结论 |
|------|------|------|
| 单一后端（只支持 Claude Code）| 绑定风险，用户无选择 | MVP 扩展到双后端 |
| 统一 API 适配层 | Claude/Codex API 差异大，维护成本高 | 适配器隔离 |
| 直接调用 API（无 Adapter）| 无法捕获 session/transcript/tool call | 放弃 |

## 实施

- `packages/core/src/adapters/` 目录存放各后端适配器实现
- Adapter 在 `src-tauri/orchestrator/` 实例化，由 `SessionOrchestrator` 管理生命周期
- Token 消耗通过 stdout 解析或 API 响应 header 提取，写入 `token_usage` 表