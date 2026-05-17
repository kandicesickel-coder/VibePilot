// packages/runner-protocol/src/index.ts
// Desktop ↔ Mobile communication protocol
// Mobile connects to desktop daemon via WebSocket for remote control

export type RunnerTransport = 'websocket' | 'grpc' | 'ssh-tunnel';

export interface RunnerConnection {
  transport: RunnerTransport;
  host: string;        // Desktop machine hostname/IP
  port: number;
  tls: boolean;        // Use TLS (wss:// or grpcs://)
}

export interface RunnerCommand {
  id: string;
  type: 'start_session' | 'send_turn' | 'cancel_session' | 'get_status';
  payload: unknown;
}

export interface RunnerEvent {
  id: string;
  type: 'chunk' | 'tool_call' | 'verification' | 'token_update' | 'error';
  payload: unknown;
}

export interface RunnerStatus {
  connected: boolean;
  desktop_version: string;
  active_session_id?: string;
  last_ping_ms: number;
}