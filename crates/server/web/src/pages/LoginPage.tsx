import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useAuthStore } from "../store/authStore";

export default function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const login = useAuthStore((s) => s.login);
  const loading = useAuthStore((s) => s.loading);
  const navigate = useNavigate();

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    try {
      await login(username, password);
      navigate("/", { replace: true });
    } catch {
      setError("Identifiants incorrects");
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-aside">
        <span className="hero-kicker">Portal Access</span>
        <h1>Entre dans le hub de supervision Sciforge.</h1>
        <p>Gère les serveurs, surveille les liaisons APK et prépare tes scénarios depuis une seule console.</p>
      </div>
      <div className="auth-card">
        <h1>Connexion</h1>
        <form onSubmit={submit}>
          <label>
            Nom d'utilisateur
            <input value={username} onChange={(e) => setUsername(e.target.value)} required autoFocus />
          </label>
          <label>
            Mot de passe
            <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
          </label>
          {error && <p className="error">{error}</p>}
          <button className="btn-primary" type="submit" disabled={loading}>
            {loading ? "Connexion…" : "Se connecter"}
          </button>
          <p>
            Pas encore de compte ? <Link to="/register">S'inscrire</Link>
          </p>
        </form>
      </div>
    </div>
  );
}
