import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useAuthStore } from "../store/authStore";

export default function RegisterPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const [error, setError] = useState<string | null>(null);
  const register = useAuthStore((s) => s.register);
  const loading = useAuthStore((s) => s.loading);
  const navigate = useNavigate();

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    if (password !== confirm) {
      setError("Les mots de passe ne correspondent pas");
      return;
    }
    try {
      await register(username, password);
      navigate("/", { replace: true });
    } catch {
      setError("Nom d'utilisateur déjà pris ou données invalides");
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-aside">
        <span className="hero-kicker">Onboarding</span>
        <h1>Crée ton accès opérateur pour piloter tes mondes.</h1>
        <p>Le compte te permet de regrouper tes serveurs, tes liaisons et ton historique côté portail.</p>
      </div>
      <div className="auth-card">
        <h1>Créer un compte</h1>
        <form onSubmit={submit}>
          <label>
            Nom d'utilisateur
            <input value={username} onChange={(e) => setUsername(e.target.value)} required autoFocus />
          </label>
          <label>
            Mot de passe
            <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
          </label>
          <label>
            Confirmer le mot de passe
            <input type="password" value={confirm} onChange={(e) => setConfirm(e.target.value)} required />
          </label>
          {error && <p className="error">{error}</p>}
          <button className="btn-primary" type="submit" disabled={loading}>
            {loading ? "Création…" : "S'inscrire"}
          </button>
        </form>
        <p>
          Déjà un compte ? <Link to="/login">Se connecter</Link>
        </p>
      </div>
    </div>
  );
}
