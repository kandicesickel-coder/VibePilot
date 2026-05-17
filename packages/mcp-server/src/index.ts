// packages/mcp-server/src/index.ts
// VibePilot MCP Server — exposes memory/context/tools to Claude/Codex via Model Context Protocol

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from '@modelcontextprotocol/sdk/types.js';
import { assembleContextPack } from '@vibepilot/core';
import type { LearningCard, ContextPack } from '@vibepilot/core';

// ── MCP Tools ────────────────────────────────────────────────────────────────

const VIBEPILOT_TOOLS = [
  {
    name: 'search_memory',
    description: 'Search VibePilot project memory for relevant Learning Cards',
    inputSchema: {
      type: 'object',
      properties: {
        project_id: { type: 'string', description: 'Project ID' },
        query: { type: 'string', description: 'Search query (matches title, trigger, body)' },
      },
      required: ['project_id', 'query'],
    },
  },
  {
    name: 'get_context_pack',
    description: 'Get the current context pack for a VibePilot project — rules, learning cards, active tasks',
    inputSchema: {
      type: 'object',
      properties: {
        project_id: { type: 'string', description: 'Project ID' },
        task_id: { type: 'string', description: 'Optional: specific task ID to focus context on' },
      },
      required: ['project_id'],
    },
  },
  {
    name: 'record_outcome',
    description: 'Record a task outcome (success/failed/partial) — auto-generates a Learning Card',
    inputSchema: {
      type: 'object',
      properties: {
        project_id: { type: 'string', description: 'Project ID' },
        task_id: { type: 'string', description: 'Task ID' },
        outcome: { type: 'string', enum: ['success', 'failed', 'partial'], description: 'Task outcome' },
        details: { type: 'string', description: 'What happened, what was the resolution' },
      },
      required: ['project_id', 'task_id', 'outcome', 'details'],
    },
  },
  {
    name: 'get_project_rules',
    description: 'Get project rules (AGENTS.md / CLAUDE.md content)',
    inputSchema: {
      type: 'object',
      properties: {
        project_id: { type: 'string', description: 'Project ID' },
      },
      required: ['project_id'],
    },
  },
];

// ── MCP Server ───────────────────────────────────────────────────────────────

class VibePilotMCPServer {
  private server: Server;

  constructor() {
    this.server = new Server(
      {
        name: 'VibePilot MCP Server',
        version: '0.1.0',
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.server.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: VIBEPILOT_TOOLS,
    }));

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;
      try {
        const result = await this.handleTool(name, args as Record<string, unknown>);
        return {
          content: [
            {
              type: 'text' as const,
              text: JSON.stringify(result, null, 2),
            },
          ],
        };
      } catch (error) {
        return {
          content: [
            {
              type: 'text' as const,
              text: `Error: ${error instanceof Error ? error.message : String(error)}`,
            },
          ],
          isError: true,
        };
      }
    });
  }

  private async handleTool(name: string, args: Record<string, unknown>): Promise<unknown> {
    switch (name) {
      case 'search_memory': {
        // In a real implementation, this would call Tauri commands or HTTP API
        // For now, return a stub response showing the interface
        return {
          status: 'ok',
          tool: 'search_memory',
          project_id: args.project_id,
          query: args.query,
          cards: [] as LearningCard[],
          note: 'Connect to VibePilot desktop app via Tauri IPC or HTTP to fetch real data',
        };
      }

      case 'get_context_pack': {
        return {
          status: 'ok',
          tool: 'get_context_pack',
          project_id: args.project_id,
          context_pack: null as ContextPack | null,
          note: 'Connect to VibePilot desktop app to generate real context pack',
        };
      }

      case 'record_outcome': {
        return {
          status: 'ok',
          tool: 'record_outcome',
          project_id: args.project_id,
          task_id: args.task_id,
          outcome: args.outcome,
          learning_card_id: null,
          note: 'Connect to VibePilot desktop app to create Learning Card',
        };
      }

      case 'get_project_rules': {
        return {
          status: 'ok',
          tool: 'get_project_rules',
          project_id: args.project_id,
          agents_md: null,
          claude_md: null,
          note: 'Connect to VibePilot desktop app to read AGENTS.md / CLAUDE.md',
        };
      }

      default:
        throw new Error(`Unknown tool: ${name}`);
    }
  }

  async start(): Promise<void> {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    console.error('VibePilot MCP Server started');
  }
}

// Start the server
const server = new VibePilotMCPServer();
server.start().catch(console.error);