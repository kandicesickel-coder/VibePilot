// packages/core/src/agent.ts
// Agent Backend adapter interface — the contract all AI backends must implement

export interface AgentCapabilities {
  supportsStreaming: boolean;
  supportsTokenCount: boolean;
  supportsContextCompression: boolean;
  supportsMCP: boolean;
  maxContextTokens: number;
}

export type AgentBackendId = 'claude-code' | 'codex' | 'gemini' | 'ollama';

export interface StartSessionInput {
  projectId: string;
  contextPack: ContextPack;
  workflow: WorkflowStage;
  model: string;
}

export interface AgentTurnInput {
  sessionId: string;
  message: string;
  files?: string[];
}

export type AgentEventType =
  | 'chunk'
  | 'tool_call'
  | 'verification'
  | 'token_update'
  | 'error';

export interface AgentEvent {
  type: AgentEventType;
  payload: unknown;
}

export type WorkflowStage =
  | 'spec'
  | 'plan'
  | 'build'
  | 'test'
  | 'review'
  | 'ship';

export interface ContextPack {
  projectId: string;
  sessionId?: string;
  rulesContent: string;
  learningCards: LearningCard[];
  repoMapSummary: string;
  activeTasks: Task[];
  totalTokensEstimate: number;
  components: ContextPackComponent[];
}

export interface ContextPackComponent {
  name: string;
  tokenEstimate: number;
  contentPreview: string;
}

export interface LearningCard {
  id: string;
  projectId: string;
  cardType: 'failed_attempt' | 'success_path' | 'root_cause' | 'verification_evidence';
  title: string;
  trigger: string;
  body: string;
  tokenCostUsd?: number;
  confirmedAt?: string;
  createdAt: string;
  updatedAt: string;
}

export interface Task {
  id: string;
  projectId: string;
  sessionId?: string;
  title: string;
  description: string;
  status: 'pending' | 'in_progress' | 'completed' | 'blocked';
  acceptanceCriteria: string[];
  verificationMethod: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
  dependencies: string[];
  phase: 'foundation' | 'core' | 'polish';
  createdAt: string;
  updatedAt: string;
  completedAt?: string;
}

export interface TokenUsage {
  id: string;
  sessionId?: string;
  projectId: string;
  model: string;
  inputTokens: number;
  outputTokens: number;
  cachedTokens: number;
  reasoningTokens: number;
  costUsd: number;
  createdAt: string;
}

export interface CostSummary {
  totalInputTokens: number;
  totalOutputTokens: number;
  totalCachedTokens: number;
  totalCostUsd: number;
}

export interface Project {
  id: string;
  name: string;
  path: string;
  repoUrl?: string;
  createdAt: string;
  updatedAt: string;
  lastSessionAt?: string;
  totalTokenCostUsd: number;
}