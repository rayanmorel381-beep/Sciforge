import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router-dom";
import { QRCodeSVG } from "qrcode.react";
import { linkApi } from "../api";
import { useAuthStore } from "../store/authStore";
import { uuidv4 } from "../utils/uuid";

type Phase = "loading" | "pending" | "claimed" | "confirmed" | "expired" | "error";

export default function LinkApkPage() {
  const navigate = useNavigate();
  const account = useAuthStore((s) => s.account);
  const [code, setCode] = useState<string | null>(null);
  const [expiresAt, setExpiresAt] = useState<string | null>(null);
  const [phase, setPhase] = useState<Phase>("loading");
  const sessionId = useRef(uuidv4());
  const pollingRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const issueCode = async () => {
    if (!account?.account_id) {
      setPhase("error");
      return;
    }
    setPhase("loading");
    setCode(null);
    try {
      const { data } = await linkApi.issue(sessionId.current);
      setCode(data.code);
      setExpiresAt(data.expires_at);
      setPhase("pending");
    } catch {
      setPhase("error");
    }
  };

  useEffect(() => {
    if (!account?.account_id) {
      navigate("/login", { replace: true });
      return;
    }
    issueCode();
    return () => {
      if (pollingRef.current) clearInterval(pollingRef.current);
    };
  }, [account?.account_id, navigate]);

  useEffect(() => {
    if (!code || phase !== "pending") return;

    pollingRef.current = setInterval(async () => {
      try {
        const { data } = await linkApi.status(code);

        if (data.status === "claimed") {
          setPhase("claimed");
        }

        if (data.status === "confirmed") {
          setPhase("confirmed");
          if (pollingRef.current) clearInterval(pollingRef.current);
          setTimeout(() => navigate("/"), 1500);
        }

        if (expiresAt && new Date() > new Date(expiresAt)) {
          setPhase("expired");
          if (pollingRef.current) clearInterval(pollingRef.current);
        }
      } catch {
        setPhase("error");
        if (pollingRef.current) clearInterval(pollingRef.current);
      }
    }, 2000);

    return () => {
      if (pollingRef.current) clearInterval(pollingRef.current);
    };
  }, [code, phase, expiresAt, navigate]);

  const qrValue = code
    ? `sciforge://link?code=${code}&account_id=${account?.account_id ?? ""}`
    : "";

  return (
    <div className="page link-page">
      <section className="page-hero page-hero-compact">
        <div className="hero-copy">
          <span className="hero-kicker">Device Pairing</span>
          <h1>Lier l'APK</h1>
          <p className="subtitle">
            Scanne le QR code depuis l'application pour authentifier un appareil sur le portail.
          </p>
        </div>
      </section>

      <div className="qr-container">
        {phase === "loading" && <p>Génération du code…</p>}
        {(phase === "pending" || phase === "claimed") && code && (
          <>
            <QRCodeSVG value={qrValue} size={220} />
            <p className="link-code">{code}</p>
          </>
        )}
        {phase === "claimed" && (
          <div className="status-badge claimed">APK détecté — en attente de confirmation…</div>
        )}
        {phase === "confirmed" && (
          <div className="status-badge confirmed">✓ Lien établi, redirection…</div>
        )}
        {phase === "expired" && (
          <div className="status-badge expired">Code expiré</div>
        )}
        {phase === "error" && (
          <div className="status-badge error">Erreur réseau</div>
        )}
      </div>

      {(phase === "expired" || phase === "error") && (
        <button className="btn-primary" onClick={issueCode}>
          Générer un nouveau code
        </button>
      )}
    </div>
  );
}
