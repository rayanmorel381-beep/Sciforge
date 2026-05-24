import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { linkApi, serversApi } from "../api";
import type { ServerVisibility } from "../types";
import { useAuthStore } from "../store/authStore";
import { uuidv4 } from "../utils/uuid";

function asErrorMessage(error: unknown): string {
  if (typeof error === "object" && error !== null) {
    const maybeResponse = error as { response?: { data?: unknown } };
    const payload = maybeResponse.response?.data;
    if (typeof payload === "string" && payload.trim()) return payload;
    if (typeof payload === "object" && payload !== null) {
      const withMessage = payload as { message?: string; error?: string };
      if (typeof withMessage.message === "string" && withMessage.message.trim()) return withMessage.message;
      if (typeof withMessage.error === "string" && withMessage.error.trim()) return withMessage.error;
    }
  }
  if (error instanceof Error && error.message.trim()) return error.message;
  return "Impossible de créer le serveur pour le moment.";
}

export default function CreateServerPage() {
  const navigate = useNavigate();
  const account = useAuthStore((s) => s.account);
  const [name, setName] = useState("");
  const [region, setRegion] = useState("eu-west");
  const [mode, setMode] = useState("sandbox");
  const [visibility, setVisibility] = useState<ServerVisibility>("public");
  const [maxPlayers, setMaxPlayers] = useState(10);

  const mutation = useMutation({
    mutationFn: async () => {
      if (!account?.account_id) throw new Error("Session invalide. Reconnecte-toi.");
      const { data: link } = await linkApi.issue(uuidv4());
      return serversApi.create({
        owner_account_id: account.account_id,
        link_code: link.code,
        name,
        region,
        mode,
        visibility,
        max_players: maxPlayers,
      });
    },
    onSuccess: ({ data }) => navigate(`/servers/${data.id}`),
  });

  const submit = (e: React.FormEvent) => {
    e.preventDefault();
    mutation.mutate();
  };

  return (
    <div className="page">
      <section className="page-hero page-hero-compact">
        <div className="hero-copy">
          <span className="hero-kicker">Provisioning</span>
          <h1>Déploie un monde calibré pour ton groupe.</h1>
          <p>
            Choisis une région, une visibilité et un mode. Le portail génère ensuite l'instance et le canal temps réel associé.
          </p>
        </div>
        <div className="hero-panel form-summary-panel">
          <div className="metric-card">
            <span className="metric-label">Région</span>
            <strong>{region}</strong>
          </div>
          <div className="metric-card">
            <span className="metric-label">Mode</span>
            <strong>{mode}</strong>
          </div>
          <div className="metric-card metric-card-accent">
            <span className="metric-label">Capacité</span>
            <strong>{maxPlayers} joueurs</strong>
          </div>
        </div>
      </section>

      <div className="form-layout">
        <form onSubmit={submit} className="form-card form-card-large">
          <label>
            Nom du serveur
            <input value={name} onChange={(e) => setName(e.target.value)} required autoFocus placeholder="Ex: Atlas Prime" />
          </label>
          <div className="form-grid">
            <label>
              Région
              <select value={region} onChange={(e) => setRegion(e.target.value)}>
                <option value="eu-west">EU West</option>
                <option value="eu-central">EU Central</option>
                <option value="us-east">US East</option>
                <option value="us-west">US West</option>
                <option value="asia-south">Asia South</option>
              </select>
            </label>
            <label>
              Mode de jeu
              <select value={mode} onChange={(e) => setMode(e.target.value)}>
                <option value="sandbox">Sandbox</option>
                <option value="survival">Survival</option>
                <option value="arena">Arena</option>
                <option value="co-op">Co-op</option>
              </select>
            </label>
            <label>
              Visibilité
              <select value={visibility} onChange={(e) => setVisibility(e.target.value as ServerVisibility)}>
                <option value="public">Public</option>
                <option value="private">Privé</option>
                <option value="invite_only">Sur invitation</option>
                <option value="unlisted">Non listé</option>
              </select>
            </label>
            <label>
              Joueurs max
              <input
                type="number"
                min={2}
                max={100}
                value={maxPlayers}
                onChange={(e) => setMaxPlayers(Number(e.target.value))}
              />
            </label>
          </div>
          {mutation.isError && <p className="error">{asErrorMessage(mutation.error)}</p>}
          <button className="btn-primary" type="submit" disabled={mutation.isPending || !account}>
            {mutation.isPending ? "Création…" : "Créer le serveur"}
          </button>
        </form>

        <aside className="info-panel">
          <h2>Résumé opérationnel</h2>
          <div className="info-list">
            <div className="info-item">
              <span>Visibilité</span>
              <strong>{visibility}</strong>
            </div>
            <div className="info-item">
              <span>Mode</span>
              <strong>{mode}</strong>
            </div>
            <div className="info-item">
              <span>Session temps réel</span>
              <strong>Provisionnée à la création</strong>
            </div>
            <div className="info-item">
              <span>Compte opérateur</span>
              <strong>{account?.username ?? "Non connecté"}</strong>
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
}
