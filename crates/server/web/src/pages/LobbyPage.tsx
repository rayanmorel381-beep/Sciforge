import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router-dom";
import { healthApi, lobbyApi } from "../api";
import { useAuthStore } from "../store/authStore";
import type { GameServer } from "../types";

function ServerCard({ server }: { server: GameServer }) {
  return (
    <div className="server-card">
      <div className="server-card-header">
        <span className="server-name">{server.name}</span>
        <span className={`badge ${server.visibility}`}>{server.visibility}</span>
      </div>
      <div className="server-card-row">
        <span className="server-card-label">Zone</span>
        <span>{server.region}</span>
      </div>
      <div className="server-card-row">
        <span className="server-card-label">Mode</span>
        <span>{server.mode}</span>
      </div>
      <div className="server-card-row">
        <span className="server-card-label">Capacité</span>
        <span>{server.max_players} joueurs</span>
      </div>
      <div className="server-card-footer">
        <span className="server-state">{server.state}</span>
        <Link to={`/servers/${server.id}`} className="btn-secondary">
          Ouvrir
        </Link>
      </div>
    </div>
  );
}

export default function LobbyPage() {
  const account = useAuthStore((s) => s.account);

  const publicQuery = useQuery({
    queryKey: ["lobby", "public"],
    queryFn: () => lobbyApi.public().then((r) => r.data),
    refetchInterval: 10000,
  });

  const ownedQuery = useQuery({
    queryKey: ["lobby", "owned", account?.account_id],
    queryFn: () => lobbyApi.owned(account!.account_id).then((r) => r.data),
    enabled: !!account,
    refetchInterval: 10000,
  });

  const healthQuery = useQuery({
    queryKey: ["gateway-health", "lobby"],
    queryFn: () => healthApi.get().then((r) => r.data),
    refetchInterval: 10000,
    retry: 1,
  });

  const networkStatus =
    healthQuery.isError
      ? "Indisponible"
      : healthQuery.data?.gateway.status === "ok"
        ? "Connecté"
        : "Dégradé";

  const healthyUpstreams = healthQuery.data?.upstream_health.filter((u) => u.status === "ok").length ?? 0;
  const totalUpstreams = healthQuery.data?.upstream_health.length ?? 0;

  return (
    <div className="page">
      <section className="page-hero">
        <div className="hero-copy">
          <span className="hero-kicker">Command Center</span>
          <h1>Orchestre tes mondes privés et publics sans quitter le LAN.</h1>
          <p>
            Le hub regroupe les serveurs disponibles, les mondes que tu pilotes déjà et le point de liaison vers l'application APK.
          </p>
          <div className="hero-actions">
            <Link to="/servers/create" className="btn-primary">Créer un serveur</Link>
            <Link to="/link-apk" className="btn-secondary">Lier un appareil</Link>
          </div>
        </div>
        <div className="hero-panel">
          <div className="metric-card">
            <span className="metric-label">Serveurs publics</span>
            <strong>{publicQuery.data?.length ?? 0}</strong>
          </div>
          <div className="metric-card">
            <span className="metric-label">Mes instances</span>
            <strong>{ownedQuery.data?.length ?? 0}</strong>
          </div>
          <div className="metric-card metric-card-accent">
            <span className="metric-label">Statut réseau</span>
            <strong>{networkStatus}</strong>
            <small>{healthyUpstreams}/{totalUpstreams} services opérationnels</small>
          </div>
        </div>
      </section>

      {account && (
        <section className="content-section">
          <div className="section-heading">
            <h2>Mes serveurs</h2>
            <p>Les mondes sous ton contrôle, avec accès direct aux détails et à la liaison appareil.</p>
          </div>
          {ownedQuery.isLoading && <p>Chargement…</p>}
          <div className="server-grid">
            {ownedQuery.data?.map((s) => <ServerCard key={s.id} server={s} />)}
            {ownedQuery.data?.length === 0 && <p className="empty">Aucun serveur créé</p>}
          </div>
        </section>
      )}

      <section className="content-section">
        <div className="section-heading">
          <h2>Serveurs publics</h2>
          <p>Panorama des instances ouvertes sur le réseau local.</p>
        </div>
        {publicQuery.isLoading && <p>Chargement…</p>}
        <div className="server-grid">
          {publicQuery.data?.map((s) => <ServerCard key={s.id} server={s} />)}
          {publicQuery.data?.length === 0 && <p className="empty">Aucun serveur public disponible</p>}
        </div>
      </section>
    </div>
  );
}
