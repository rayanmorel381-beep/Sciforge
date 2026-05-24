import { Navigate, useLocation } from "react-router-dom";
import { useAuthStore } from "../store/authStore";

export default function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const location = useLocation();
  const account = useAuthStore((s) => s.account);
  const hydrated = useAuthStore((s) => s.hydrated);
  const authUnavailable = useAuthStore((s) => s.authUnavailable);

  if (!hydrated) {
    return (
      <div className="page">
        <p>Vérification de session…</p>
      </div>
    );
  }

  if (authUnavailable) {
    return (
      <div className="page">
        <p className="error">Service d'authentification indisponible.</p>
      </div>
    );
  }

  if (!account) {
    return <Navigate to="/login" replace state={{ from: location }} />;
  }

  return <>{children}</>;
}
