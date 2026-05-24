import api from "./client";
import type {
  AuthResponse,
  Account,
  GameServer,
  CreateServerPayload,
  LinkCodeStatus,
  GameplayDoc,
  GameplayDocSummary,
  GatewayHealth,
} from "../types";

export const authApi = {
  register: (username: string, password: string, device_id: string) =>
    api.post<AuthResponse>("/auth/register", { username, password, device_id }),
  login: (username: string, password: string, device_id: string) =>
    api.post<AuthResponse>("/auth/login", { username, password, device_id }),
  deviceAuto: (device_id: string) =>
    api.post<AuthResponse>("/auth/device-auto", { device_id }),
  me: () => api.get<Account>("/auth/me"),
};

export const serversApi = {
  list: (account_id?: string, visibility?: string) =>
    api.get<GameServer[]>("/servers", { params: { owner_account_id: account_id, visibility } }),
  create: (payload: CreateServerPayload) => api.post<GameServer>("/servers", payload),
  get: (id: string) => api.get<GameServer>(`/servers/${id}`),
  update: (id: string, payload: Partial<CreateServerPayload>) =>
    api.patch<GameServer>(`/servers/${id}`, payload),
};

export const lobbyApi = {
  public: () => api.get<GameServer[]>("/lobby/public"),
  owned: (account_id: string) => api.get<GameServer[]>(`/lobby/owned/${account_id}`),
};

export const linkApi = {
  issue: (browser_session_id: string) =>
    api.post<{ code: string; expires_at: string }>("/link/codes", {
      apk_session_id: browser_session_id,
    }),
  status: (code: string) => api.get<LinkCodeStatus>(`/link/codes/${code}`),
};

export const gameplayApi = {
  list: () => api.get<GameplayDocSummary[]>("/gameplay"),
  get: (slug: string) => api.get<GameplayDoc>(`/gameplay/${slug}`),
};

export const healthApi = {
  get: () => api.get<GatewayHealth>("/health"),
};
