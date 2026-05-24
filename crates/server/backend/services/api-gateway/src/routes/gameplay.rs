use std::{fs, path::{Path, PathBuf}};

use axum::{extract::{Path as AxumPath, State}, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::app_state::AppState;

#[derive(Serialize)]
pub struct GameplayDocSummary {
    pub slug: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct GameplayDoc {
    pub slug: String,
    pub title: String,
    pub content: String,
}

fn gameplay_root() -> PathBuf {
    if let Ok(dir) = std::env::var("GAMEPLAY_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() {
            return path;
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join("../../gameplay");
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("gameplay")
}

fn slug_title_from_file(path: &Path, slug: &str) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    if let Some(line) = content.lines().find(|line| line.trim_start().starts_with('#')) {
        return line.trim_start_matches('#').trim().to_string();
    }
    slug.replace('-', " ")
}

pub async fn list_docs(State(_state): State<AppState>) -> impl IntoResponse {
    let root = gameplay_root();
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return (StatusCode::NOT_FOUND, "gameplay directory unavailable").into_response(),
    };

    let mut docs: Vec<GameplayDocSummary> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let file_type = entry.file_type().ok()?;
            if !file_type.is_dir() {
                return None;
            }
            let slug = entry.file_name().to_string_lossy().to_string();
            let readme = entry.path().join("README.md");
            if !readme.exists() {
                return None;
            }
            let title = slug_title_from_file(&readme, &slug);
            Some(GameplayDocSummary { slug, title })
        })
        .collect();

    docs.sort_by(|a, b| a.slug.cmp(&b.slug));
    Json(docs).into_response()
}

pub async fn get_doc(State(_state): State<AppState>, AxumPath(slug): AxumPath<String>) -> impl IntoResponse {
    if slug.is_empty() || slug.contains('/') || slug.contains("..") {
        return (StatusCode::BAD_REQUEST, "invalid slug").into_response();
    }

    let path = gameplay_root().join(&slug).join("README.md");
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return (StatusCode::NOT_FOUND, "gameplay doc not found").into_response(),
    };

    let title = content
        .lines()
        .find(|line| line.trim_start().starts_with('#'))
        .map(|line| line.trim_start_matches('#').trim().to_string())
        .unwrap_or_else(|| slug.replace('-', " "));

    Json(GameplayDoc {
        slug,
        title,
        content,
    })
    .into_response()
}
