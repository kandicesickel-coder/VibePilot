# ADR-003: Context Pack 组装设计

## 状态
**Accepted** | 2026-05-18

## 背景
Context Pack 是发给 Agent 的上下文包。需要：
1. Token 预算可控（不超过模型上下文窗口）
2. 利于 Prompt Caching（Anthropic/OpenAI API 支持）
3. 每次只注入与当前任务相关的记忆，不重复喂全量

## 决策

**Context Pack 采用"规则前缀固定 + 动态内容后置"的结构。**

```
┌─────────────────────────────────────────────────────┐
│ [PART 1: 固定规则前缀] — 每次相同，利于 Prompt Caching │
│ - 项目规则（AGENTS.md / CLAUDE.md 内容）             │
│ - 技术栈约束（TypeScript strict / Rust no unsafe）   │
│ - 工作流阶段定义（spec → plan → build → ...）       │
│                                                       │
│ [PART 2: 动态内容] — 每次变化，按需注入                │
│ - 相关 Learning Cards（检索匹配当前任务）            │
│ - Active Tasks（pending / in_progress）             │
│ - Repo Map（符号级代码地图，token 预算裁剪）        │
│ - Diff（当前变更文件，仅在 build 阶段注入）          │
└─────────────────────────────────────────────────────┘
```

**Token 估算**（每个组件独立估算，总和不超过 `max_context_tokens` 的 80%）：
- 1 token ≈ 4 characters（英文）；中文 2 characters
- Rules：按实际字符数 / 4
- Learning Cards：每张卡片 `body.len() / 4`
- Tasks：JSON 序列化后 `len() / 4`
- Repo Map：符号表 `symbols_json.len() / 4`

**Prompt Caching 优化**：
- Anthropic（Claude）：`cache_control` 可控制缓存边界
- OpenAI（Codex）：Prompt Caching 支持固定前缀
- VibePilot 将 Rules 部分作为缓存候选（`components[0]` = rules）

## 替代方案考虑

| 方案 | 劣势 | 结论 |
|------|------|------|
| 全量喂入（无 Context Pack）| 每次都重复喂全部项目历史，token 浪费 60%+ | 放弃 |
| 仅靠向量检索压缩上下文 | 需要外部向量库，架构复杂度高 | MVP 简化处理 |
| 按文件粒度注入上下文 | 需要语言解析，复杂度高 | P2 再评估 |

## 实施

- `packages/core/src/context.ts` → `assembleContextPack()` 函数
- 每个组件有 `tokenEstimate` 和 `contentPreview`
- 组装前检查总 token 预算（默认 8K tokens）
- 超预算时：优先保留 rules 和 Learning Cards，裁剪 repo_map