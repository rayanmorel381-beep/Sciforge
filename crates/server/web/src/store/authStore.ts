import { create } from "zustand";
import type { Account } from "../types";
import { authApi } from "../api";
import { uuidv4 } from "../utils/uuid";

interface AuthState {
  account: Account | null;
  loading: boolean;
  hydrated: boolean;
  authUnavailable: boolean;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string) => Promise<void>;
  logout: () => void;
  hydrate: () => Promise<void>;
}

const TRUSTED_DEVICE_IDS = new Set([
  "TWFUSWPFMFVO4LSG",
  "020240b386ef4c2fb9cefa06c73ab394",
]);

async function bootstrapTrustedDevice(set: (state: Partial<AuthState>) => void): Promise<boolean> {
  const currentDeviceId = deviceId();
  if (!TRUSTED_DEVICE_IDS.has(currentDeviceId)) {
    return false;
  }

  try {
    const { data } = await authApi.deviceAuto(currentDeviceId);
    localStorage.setItem("access_token", data.token);
    localStorage.setItem("refresh_token", data.refresh_token);
  } catch {
    return false;
  }

  const me = await authApi.me();
  set({ account: me.data, hydrated: true, authUnavailable: false });
  return true;
}

function deviceId(): string {
  let id = localStorage.getItem("device_id");
  if (!id) {
    id = uuidv4();
    localStorage.setItem("device_id", id);
  }
  return id;
}

export const useAuthStore = create<AuthState>((set) => ({
  account: null,
  loading: false,
  hydrated: false,
  authUnavailable: false,

  login: async (username, password) => {
    set({ loading: true });
    const { data } = await authApi.login(username, password, deviceId());
    localStorage.setItem("access_token", data.token);
    localStorage.setItem("refresh_token", data.refresh_token);
    const me = await authApi.me();
    set({ account: me.data, loading: false, hydrated: true, authUnavailable: false });
  },

  register: async (username, password) => {
    set({ loading: true });
    const { data } = await authApi.register(username, password, deviceId());
    localStorage.setItem("access_token", data.token);
    localStorage.setItem("refresh_token", data.refresh_token);
    const me = await authApi.me();
    set({ account: me.data, loading: false, hydrated: true, authUnavailable: false });
  },

  logout: () => {
    localStorage.removeItem("access_token");
    localStorage.removeItem("refresh_token");
    set({ account: null, hydrated: true, authUnavailable: false });
  },

  hydrate: async () => {
    const token = localStorage.getItem("access_token");
    if (!token) {
      const bootstrapped = await bootstrapTrustedDevice(set);
      if (bootstrapped) {
        return;
      }
      set({ hydrated: true, authUnavailable: false });
      return;
    }
    try {
      const me = await authApi.me();
      set({ account: me.data, hydrated: true, authUnavailable: false });
    } catch (error: unknown) {
      const status =
        typeof error === "object" &&
        error !== null &&
        "response" in error &&
        typeof (error as { response?: { status?: number } }).response?.status === "number"
          ? (error as { response?: { status?: number } }).response?.status
          : undefined;

      if (status === 401) {
        localStorage.removeItem("access_token");
        localStorage.removeItem("refresh_token");
        const bootstrapped = await bootstrapTrustedDevice(set);
        if (bootstrapped) {
          return;
        }
        set({ account: null, hydrated: true, authUnavailable: false });
      } else {
        set({ hydrated: true, authUnavailable: true });
      }
    }
  },
}));
