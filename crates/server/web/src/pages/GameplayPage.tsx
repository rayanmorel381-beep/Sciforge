import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { gameplayApi } from "../api";

function renderMarkdown(text: string): string {
  return text
    .replace(/^###\s+(.*)$/gm, "<h3>$1</h3>")
    .replace(/^##\s+(.*)$/gm, "<h2>$1</h2>")
    .replace(/^#\s+(.*)$/gm, "<h1>$1</h1>")
    .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
    .replace(/^\-\s+(.*)$/gm, "<li>$1</li>")
    .replace(/(<li>.*<\/li>\n?)+/g, (list) => `<ul>${list}</ul>`)
    .replace(/\n\n+/g, "</p><p>")
    .replace(/^(?!<h\d>|<ul>|<li>|<\/ul>)(.+)$/gm, "<p>$1</p>")
    .replace(/<p><\/p>/g, "");
}

export default function GameplayPage() {
  const [selectedSlug, setSelectedSlug] = useState<string | null>(null);

  const listQuery = useQuery({
    queryKey: ["gameplay", "list"],
    queryFn: () => gameplayApi.list().then((response) => response.data),
  });

  const effectiveSlug = selectedSlug ?? listQuery.data?.[0]?.slug ?? null;

  const docQuery = useQuery({
    queryKey: ["gameplay", "doc", effectiveSlug],
    queryFn: () => gameplayApi.get(effectiveSlug!).then((response) => response.data),
    enabled: !!effectiveSlug,
  });

  const renderedDoc = useMemo(() => {
    if (!docQuery.data?.content) return "";
    return renderMarkdown(docQuery.data.content);
  }, [docQuery.data?.content]);

  return (
    <div className="page">
      <section className="page-hero page-hero-compact">
        <div className="hero-copy">
          <span className="hero-kicker">Design Atlas</span>
          <h1>Bibliothèque des concepts gameplay.</h1>
          <p>Contenu chargé depuis les fichiers README réels du dossier gameplay.</p>
        </div>
        <div className="hero-panel">
          <div className="metric-card metric-card-accent">
            <span className="metric-label">Concepts</span>
            <strong>{listQuery.data?.length ?? 0}</strong>
          </div>
        </div>
      </section>

      {listQuery.isLoading && <p>Chargement des concepts…</p>}
      {listQuery.isError && <p className="error">Impossible de charger la liste gameplay.</p>}

      <div className="gameplay-layout">
        <div className="server-grid gameplay-grid">
          {listQuery.data?.map((doc) => (
            <button
              key={doc.slug}
              type="button"
              className={`server-card gameplay-card gameplay-card-button${effectiveSlug === doc.slug ? " active" : ""}`}
              onClick={() => setSelectedSlug(doc.slug)}
            >
            <div className="server-card-header">
                <h3>{doc.title}</h3>
                <span className="badge">{doc.slug}</span>
            </div>
              <p className="server-card-info">Ouvrir le document détaillé</p>
            </button>
          ))}
        </div>

        <article className="detail-card gameplay-doc">
          {docQuery.isLoading && <p>Chargement du document…</p>}
          {docQuery.isError && <p className="error">Impossible de charger ce document.</p>}
          {docQuery.data && (
            <div
              className="gameplay-markdown"
              dangerouslySetInnerHTML={{ __html: renderedDoc }}
            />
          )}
        </article>
      </div>
    </div>
  );
}
