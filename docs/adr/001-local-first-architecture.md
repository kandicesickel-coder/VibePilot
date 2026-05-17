# ADR-001: 本地优先架构

## 状态
**Accepted** | 2026-05-18

## 背景
VibePilot 面向个人开发者和小团队，需要在没有云端依赖的情况下运行所有核心功能。

## 决策

**VibePilot 采用本地优先架构，所有数据存储在用户本地。**

- 桌面端数据存储在 `~/.local/share/VibePilot/data.db`（Linux/macOS）或 `~/Library/Application Support/VibePilot/data.db`（macOS）
- 数据库使用 SQLite + WAL 模式，支持读写并发
- 加密：MVP 暂不上，商业版可选 SQLCipher（AES-256）

## 替代方案考虑

| 方案 | 优势 | 劣势 | 结论 |
|------|------|------|------|
| 云端 PostgreSQL | 团队共享 | 需要网络、隐私风险 | MVP 阶段不考虑 |
| 本地 SQLite | 简单、隐私、无依赖 | 无团队同步 | **选择** |
| IndexedDB（浏览器）| 跨平台 | 不适合 Rust 侧数据 | 移动端 WebView 可用 |

## 后果

**正面**：
- 用户完全掌控数据，无隐私风险
- 离线可用，无网络依赖
- 部署简单（单文件安装包）

**负面**：
- 多设备同步需要额外方案（未来 Team 版）
- 数据在设备损坏时可能丢失（无云备份）

## 实施

- Rust 侧使用 `rusqlite` + `dirs` crate 获取 `data_local_dir()`
- React 前端通过 Tauri commands 访问 DB，不直接操作 SQLite