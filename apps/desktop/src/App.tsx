import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Project {
  id: string;
  name: string;
  path: string;
  created_at: string;
  total_token_cost_usd: number;
}

interface TauriInfo {
  version: string;
  tauri_version: string;
}

function App() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [tauriInfo, setTauriInfo] = useState<string>('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      invoke<string>('get_tauri_info'),
      invoke<Project[]>('list_projects'),
    ]).then(([info, projs]) => {
      setTauriInfo(info);
      setProjects(projs);
      setLoading(false);
    }).catch((err) => {
      console.error('Failed to load:', err);
      setLoading(false);
    });
  }, []);

  return (
    <div className="min-h-screen bg-[#001a2e] text-white">
      {/* Header */}
      <header className="border-b border-cyan-900/50 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <span className="text-2xl font-bold text-cyan-400">X</span>
            <div>
              <h1 className="text-lg font-semibold text-white">VibePilot</h1>
              <p className="text-xs text-cyan-600">{tauriInfo || 'Loading...'}</p>
            </div>
          </div>
        </div>
      </header>

      {/* Main */}
      <main className="p-8">
        <div className="max-w-4xl mx-auto">
          {/* Hero */}
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold mb-4">
              让任何 Agent 进入你的项目时，
              <br />
              <span className="text-cyan-400">都像一个记得上次踩坑的老同事</span>
            </h2>
            <p className="text-gray-400 text-sm">
              本地优先的 Agent 研发记忆与工作流层 — VibePilot
            </p>
          </div>

          {/* Quick Actions */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
            <ActionCard
              title="创建项目"
              description="扫描本地目录，生成项目画像和 Repo Map"
              icon="+"
            />
            <ActionCard
              title="启动 Agent"
              description="选择 Claude Code 或 Codex，开始编程"
              icon=">"
            />
          </div>

          {/* Projects List */}
          <div className="border border-cyan-900/30 rounded-lg p-6">
            <h3 className="text-sm font-semibold text-cyan-400 mb-4">项目 ({projects.length})</h3>
            {loading ? (
              <p className="text-gray-500 text-sm">加载中...</p>
            ) : projects.length === 0 ? (
              <p className="text-gray-600 text-sm">暂无项目，请创建第一个项目</p>
            ) : (
              <div className="space-y-3">
                {projects.map((p) => (
                  <ProjectRow key={p.id} project={p} />
                ))}
              </div>
            )}
          </div>

          {/* MCP Server Status */}
          <div className="mt-8 border border-cyan-900/30 rounded-lg p-6">
            <h3 className="text-sm font-semibold text-cyan-400 mb-3">MCP Server 工具</h3>
            <div className="grid grid-cols-2 gap-3 text-xs">
              <ToolChip name="search_memory" desc="检索项目记忆" />
              <ToolChip name="get_context_pack" desc="获取当前上下文包" />
              <ToolChip name="record_outcome" desc="记录任务结果" />
              <ToolChip name="get_project_rules" desc="获取项目规则" />
            </div>
            <p className="text-gray-600 text-xs mt-3">在 Claude Code / Codex 中配置 MCP 后即可使用</p>
          </div>
        </div>
      </main>
    </div>
  );
}

function ActionCard({ title, description, icon }: { title: string; description: string; icon: string }) {
  return (
    <button className="group text-left border border-cyan-900/40 rounded-lg p-6 hover:border-cyan-500/60 hover:bg-cyan-900/10 transition-all">
      <div className="flex items-start gap-4">
        <span className="text-2xl text-cyan-400 group-hover:scale-110 transition-transform">{icon}</span>
        <div>
          <h3 className="font-semibold text-white mb-1">{title}</h3>
          <p className="text-xs text-gray-400">{description}</p>
        </div>
      </div>
    </button>
  );
}

function ProjectRow({ project }: { project: Project }) {
  return (
    <div className="flex items-center justify-between py-3 border-b border-cyan-900/20 last:border-0">
      <div>
        <p className="text-sm font-medium text-white">{project.name}</p>
        <p className="text-xs text-gray-500">{project.path}</p>
      </div>
      <div className="text-right">
        <p className="text-xs text-cyan-400">${project.total_token_cost_usd.toFixed(4)}</p>
        <p className="text-xs text-gray-600">{project.created_at.slice(0, 10)}</p>
      </div>
    </div>
  );
}

function ToolChip({ name, desc }: { name: string; desc: string }) {
  return (
    <div className="bg-cyan-900/20 border border-cyan-900/30 rounded px-3 py-2">
      <p className="text-cyan-400 font-mono text-xs">{name}</p>
      <p className="text-gray-500 text-xs">{desc}</p>
    </div>
  );
}

export default App;