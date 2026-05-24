export interface AuthResponse {
  account_id: string;
  username: string;
  token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
}

export interface Account {
  account_id: string;
  username: string;
  device_id: string;
}

export type ServerVisibility = "public" | "private" | "invite_only" | "unlisted";
export type ServerState = "online" | "offline" | "maintenance";

export interface GameServer {
  id: string;
  name: string;
  owner_account_id: string;
  link_code: string;
  join_code: string;
  access_code?: string;
  realtime_endpoint: string;
  region: string;
  mode: string;
  visibility: ServerVisibility;
  max_players: number;
  state: ServerState;
  updated_at: string;
  created_at: string;
}

export interface CreateServerPayload {
  owner_account_id: string;
  link_code: string;
  name: string;
  region: string;
  mode: string;
  visibility: ServerVisibility;
  max_players: number;
}

export interface LinkCodeStatus {
  code: string;
  status: "pending" | "claimed" | "confirmed";
  account_id?: string;
  browser_session_id?: string;
  expires_at: string;
}

export interface GameplayDocSummary {
  slug: string;
  title: string;
}

export interface GameplayDoc {
  slug: string;
  title: string;
  content: string;
}

export interface ServiceHealth {
  service: string;
  status: string;
  now: string;
}

export interface UpstreamHealth {
  service: string;
  url: string;
  status: string;
  http_status?: number;
}

export interface GatewayHealth {
  gateway: ServiceHealth;
  upstreams: string[];
  upstream_health: UpstreamHealth[];
}
