import { useQuery } from "@tanstack/react-query";
import { Link, useParams } from "react-router-dom";
import { serversApi } from "../api";
import { useAuthStore } from "../store/authStore";

export default function ServerDetailPage() {
  const { id } = useParams<{ id: string }>();
  const account = useAuthStore((s) => s.account);

  const { data: server, isLoading, isError } = useQuery({
    queryKey: ["server", id],
    queryFn: () => serversApi.get(id!).then((r) => r.data),
    enabled: !!id,
    refetchInterval: 5000,
  });

  if (isLoading) return <div className="page"><p>Chargement…</p></div>;
  if (isError || !server) return <div className="page"><p>Serveur introuvable</p></div>;

  const isOwner = account?.account_id === server.owner_account_id;

  return (
    <div className="page">
      <section className="page-hero page-hero-compact">
        <div className="hero-copy">
          <span className="hero-kicker">Server Profile</span>
          <h1>{server.name}</h1>
          <p>Vue synthétique de l'instance, de son état réseau et de ses accès temps réel.</p>
        </div>
        <div className="hero-panel">
          <div className="metric-card">
            <span className="metric-label">Visibilité</span>
            <strong>{server.visibility}</strong>
          </div>
          <div className="metric-card">
            <span className="metric-label">Etat</span>
            <strong>{server.state}</strong>
          </div>
          <div className="metric-card metric-card-accent">
            <span className="metric-label">Capacité</span>
            <strong>{server.max_players} joueurs</strong>
          </div>
        </div>
      </section>

      <div className="detail-card detail-card-grid">
        <div className="detail-item"><span>Etat</span><strong>{server.state}</strong></div>
        <div className="detail-item"><span>Région</span><strong>{server.region}</strong></div>
        <div className="detail-item"><span>Mode</span><strong>{server.mode}</strong></div>
        <div className="detail-item"><span>Join code</span><code>{server.join_code}</code></div>
        <div className="detail-item detail-item-wide"><span>Realtime</span><code>{server.realtime_endpoint}</code></div>
        <div className="detail-item"><span>Créé le</span><strong>{new Date(server.created_at).toLocaleString("fr-FR")}</strong></div>
        {server.access_code && (
          <div className="detail-item"><span>Code d'accès</span><code>{server.access_code}</code></div>
        )}
      </div>

      <div className="detail-actions">
        {isOwner && (
          <Link to={`/servers/${id}/link`} className="btn-primary">
            Lier un appareil APK
          </Link>
        )}
        <Link to="/" className="btn-secondary">Retour au lobby</Link>
      </div>
    </div>
  );
}
