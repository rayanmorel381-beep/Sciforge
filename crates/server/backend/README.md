# Backend MVP

Ce dossier contient un backend MVP en Rust pour le flux principal :

- creation de compte mobile
- generation d'un code de liaison APK
- reclamation de ce code depuis le site
- creation d'un serveur public ou prive
- consultation du lobby

La persistence peut fonctionner :

- en memoire si aucune URL de stockage n'est fournie
- avec PostgreSQL pour identity, link-service et server-manager
- avec Redis pour les tokens identity

## Services

- api-gateway
- identity
- link-service
- server-manager
- lobby
- realtime-gateway

## Lancement

Dans des terminaux separes :

```bash
cargo run -p identity
cargo run -p link-service
cargo run -p server-manager
cargo run -p lobby
cargo run -p realtime-gateway
cargo run -p api-gateway
```

Le gateway ecoute par defaut sur le port 8080.

## Infrastructure locale

```bash
docker compose -f infra/docker/docker-compose.yml up -d
```
