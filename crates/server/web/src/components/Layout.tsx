import { Link, NavLink } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { healthApi } from "../api";
import { useAuthStore } from "../store/authStore";

export default function Layout({ children }: { children: React.ReactNode }) {
  const account = useAuthStore((s) => s.account);
  const logout = useAuthStore((s) => s.logout);
  const healthQuery = useQuery({
    queryKey: ["gateway-health", "layout"],
    queryFn: () => healthApi.get().then((r) => r.data),
    refetchInterval: 10000,
    retry: 1,
  });

  const lanStatus =
    healthQuery.isError
      ? "LAN indisponible"
      : healthQuery.data?.gateway.status === "ok"
        ? "LAN connecté"
        : "LAN dégradé";

  return (
    <div className="layout">
      <div className="layout-glow layout-glow-a" />
      <div className="layout-glow layout-glow-b" />
      <div className="layout-shell">
        <nav className="navbar">
          <div className="brand-block">
            <span className="brand-kicker">Control Room</span>
            <Link to="/" className="nav-logo">Sciforge</Link>
          </div>
          <div className="nav-links">
            <NavLink to="/" end className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
              Hub
            </NavLink>
            <NavLink to="/gameplay" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
              Gameplay
            </NavLink>
            <NavLink to="/servers/create" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
              Nouveau serveur
            </NavLink>
            <NavLink to="/link-apk" className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}>
              Liaison APK
            </NavLink>
          </div>
          <div className="nav-meta">
            <span className="nav-status">{lanStatus}</span>
            {account ? (
              <>
                <span className="nav-user">{account.username}</span>
                <button type="button" className="btn-secondary nav-auth-btn" onClick={logout}>
                  Déconnexion
                </button>
              </>
            ) : (
              <>
                <Link to="/login" className="btn-secondary nav-auth-btn">Connexion</Link>
                <Link to="/register" className="btn-primary nav-auth-btn">Inscription</Link>
              </>
            )}
          </div>
        </nav>
        <main className="site-main">{children}</main>
      </div>
    </div>
  );
}
