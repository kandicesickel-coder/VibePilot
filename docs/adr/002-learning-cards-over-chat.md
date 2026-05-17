# ADR-002: Learning Cards 作为核心记忆单元

## 状态
**Accepted** | 2026-05-18

## 背景
市场已有工具（Pieces、Cursor Memories）偏向"事实/规则/聊天摘要"存储。但 VibePilot 的核心差异化是：保存"失败尝试 + 成功路径 + 验证证据"，让 Agent 下次不重蹈覆辙。

## 决策

**VibePilot 的核心记忆单元是 Learning Card，不是聊天历史。**

每个 Learning Card 包含：
- `type`: `failed_attempt` | `success_path` | `root_cause` | `verification_evidence`
- `title`: 简短标题
- `trigger`: 触发条件（什么场景触发了这个经验）
- `body`: 结构化经验内容（JSON）
- `confirmed_at`: NULL = pending review， 有值 = 已确认入库
- `token_cost_usd`: 本次消耗（用于成本分析）

**不是聊天记录的原因：**
1. 聊天记录大小随会话线性增长 → token 成本高
2. 聊天记录包含大量上下文填充 → 检索效率低
3. 聊天记录不结构化 → 无法按类型/触发条件过滤

**Learning Card 的优势：**
1. 结构化 + 可检索 → 每次只注入相关经验，token 成本低
2. 经验固化 → 成功路径可跨 Agent 复用
3. pending review 机制 → 用户决定什么值得记住

## 替代方案考虑

| 方案 | 劣势 | 结论 |
|------|------|------|
| 聊天历史摘要 | 大小线性增长、不结构化 | 放弃 |
| 向量嵌入语义搜索 | 需要外部向量数据库、延迟高 | MVP 不上，P1 再评估 |
| 规则文件（AGENTS.md）| 用户需要手动维护、无法自动生成 | 补充非替代 |

## 实施

- Learning Card 通过 `record_outcome` MCP tool 自动生成
- 用户确认后写入 `learning_cards` 表
- Context Pack 组装时检索相关 Learning Cards，注入 Agent prompt
- 搜索用 SQLite FTS5 全文索引（`title LIKE ? OR trigger LIKE ? OR body LIKE ?`），不做向量检索