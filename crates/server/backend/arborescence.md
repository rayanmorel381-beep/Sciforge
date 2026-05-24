# Arborescence cible de la plateforme multijoueur

Ce document propose une arborescence complete pour une plateforme de jeu mobile multijoueur internationale en Rust.

Contraintes prises en compte :

- client mobile uniquement, avec Android et iOS
- plusieurs types de gameplay et de scenarios
- plusieurs serveurs de jeu
- un site web qui sert uniquement a lier une session APK et a creer/configurer un serveur
- serveurs publics et prives
- jeu gratuit, sans abonnement
- site web distinct des serveurs de jeu

## Vue d'ensemble exhaustive

```text
server/
├── arborescence.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .env.example
├── .gitignore
├── README.md
├── docs/
│   ├── architecture/
│   │   ├── overview.md
│   │   ├── control-plane.md
│   │   ├── game-plane.md
│   │   ├── regions.md
│   │   ├── networking.md
│   │   ├── scaling.md
│   │   └── security.md
│   ├── gameplay/
│   │   ├── modes.md
│   │   ├── scenarios.md
│   │   ├── persistence.md
│   │   └── authority-model.md
│   ├── api/
│   │   ├── public-http.md
│   │   ├── internal-http.md
│   │   ├── realtime.md
│   │   └── error-codes.md
│   └── ops/
│       ├── deployment.md
│       ├── incident-runbook.md
│       ├── observability.md
│       └── backup-restore.md
├── apps/
│   ├── web-control/
│   │   ├── public/
│   │   │   ├── favicon.ico
│   │   │   ├── manifest.webmanifest
│   │   │   └── robots.txt
│   │   ├── src/
│   │   │   ├── app/
│   │   │   │   ├── router.tsx
│   │   │   │   ├── providers.tsx
│   │   │   │   ├── store.ts
│   │   │   │   └── bootstrap.tsx
│   │   │   ├── pages/
│   │   │   │   ├── home.tsx
│   │   │   │   ├── login.tsx
│   │   │   │   ├── link.tsx
│   │   │   │   ├── create-server.tsx
│   │   │   │   ├── server-details.tsx
│   │   │   │   ├── my-servers.tsx
│   │   │   │   ├── invites.tsx
│   │   │   │   ├── terms.tsx
│   │   │   │   └── not-found.tsx
│   │   │   ├── features/
│   │   │   │   ├── auth/
│   │   │   │   │   ├── api.ts
│   │   │   │   │   ├── model.ts
│   │   │   │   │   ├── schema.ts
│   │   │   │   │   ├── LoginForm.tsx
│   │   │   │   │   └── SessionGuard.tsx
│   │   │   │   ├── link-code/
│   │   │   │   │   ├── api.ts
│   │   │   │   │   ├── model.ts
│   │   │   │   │   ├── LinkCodeForm.tsx
│   │   │   │   │   ├── LinkCodeStatus.tsx
│   │   │   │   │   └── LinkCodeTimer.tsx
│   │   │   │   ├── server-create/
│   │   │   │   │   ├── api.ts
│   │   │   │   │   ├── model.ts
│   │   │   │   │   ├── schema.ts
│   │   │   │   │   ├── ServerCreateForm.tsx
│   │   │   │   │   ├── RegionSelector.tsx
│   │   │   │   │   ├── ModeSelector.tsx
│   │   │   │   │   └── VisibilitySelector.tsx
│   │   │   │   ├── server-settings/
│   │   │   │   │   ├── api.ts
│   │   │   │   │   ├── model.ts
│   │   │   │   │   ├── ServerSettingsForm.tsx
│   │   │   │   │   ├── CapacityInput.tsx
│   │   │   │   │   └── RulesEditor.tsx
│   │   │   │   ├── server-status/
│   │   │   │   │   ├── api.ts
│   │   │   │   │   ├── model.ts
│   │   │   │   │   ├── ServerStatusCard.tsx
│   │   │   │   │   ├── RuntimeHealthBadge.tsx
│   │   │   │   │   └── PlayerCount.tsx
│   │   │   │   └── invites/
│   │   │   │       ├── api.ts
│   │   │   │       ├── model.ts
│   │   │   │       ├── InviteList.tsx
│   │   │   │       ├── InviteCreateForm.tsx
│   │   │   │       └── InviteCodeCard.tsx
│   │   │   ├── components/
│   │   │   │   ├── layout/
│   │   │   │   │   ├── Header.tsx
│   │   │   │   │   ├── Footer.tsx
│   │   │   │   │   ├── Sidebar.tsx
│   │   │   │   │   └── PageShell.tsx
│   │   │   │   ├── feedback/
│   │   │   │   │   ├── ErrorBanner.tsx
│   │   │   │   │   ├── EmptyState.tsx
│   │   │   │   │   ├── LoadingBlock.tsx
│   │   │   │   │   └── ToastViewport.tsx
│   │   │   │   └── forms/
│   │   │   │       ├── TextField.tsx
│   │   │   │       ├── SelectField.tsx
│   │   │   │       ├── ToggleField.tsx
│   │   │   │       └── SubmitButton.tsx
│   │   │   ├── api/
│   │   │   │   ├── client.ts
│   │   │   │   ├── endpoints.ts
│   │   │   │   ├── errors.ts
│   │   │   │   └── auth.ts
│   │   │   ├── lib/
│   │   │   │   ├── dates.ts
│   │   │   │   ├── formatters.ts
│   │   │   │   ├── validators.ts
│   │   │   │   └── query.ts
│   │   │   ├── styles/
│   │   │   │   ├── tokens.css
│   │   │   │   ├── reset.css
│   │   │   │   ├── app.css
│   │   │   │   └── utilities.css
│   │   │   ├── env.d.ts
│   │   │   └── main.ts
│   │   ├── tests/
│   │   │   ├── e2e/
│   │   │   │   ├── auth.spec.ts
│   │   │   │   ├── link-code.spec.ts
│   │   │   │   └── server-create.spec.ts
│   │   │   └── unit/
│   │   │       ├── validators.test.ts
│   │   │       └── formatters.test.ts
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   └── README.md
│   └── admin-console/
│       ├── src/
│       │   ├── app/
│       │   │   ├── router.tsx
│       │   │   └── bootstrap.tsx
│       │   ├── pages/
│       │   │   ├── dashboard.tsx
│       │   │   ├── realms.tsx
│       │   │   ├── users.tsx
│       │   │   ├── moderation.tsx
│       │   │   └── incidents.tsx
│       │   ├── features/
│       │   │   ├── realms/
│       │   │   │   ├── api.ts
│       │   │   │   ├── RealmList.tsx
│       │   │   │   └── RealmActions.tsx
│       │   │   ├── moderation/
│       │   │   │   ├── api.ts
│       │   │   │   ├── ReportQueue.tsx
│       │   │   │   └── SanctionForm.tsx
│       │   │   └── incidents/
│       │   │       ├── api.ts
│       │   │       ├── IncidentList.tsx
│       │   │       └── IncidentDetails.tsx
│       │   ├── api/
│       │   │   ├── client.ts
│       │   │   └── endpoints.ts
│       │   ├── styles/
│       │   │   ├── tokens.css
│       │   │   └── app.css
│       │   └── main.ts
│       ├── tests/
│       │   └── e2e/
│       │       ├── realms.spec.ts
│       │       └── moderation.spec.ts
│       ├── package.json
│       ├── tsconfig.json
│       └── README.md
├── services/
│   ├── api-gateway/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── state.rs
│   │   │   ├── bootstrap.rs
│   │   │   ├── clients/
│   │   │   │   ├── identity.rs
│   │   │   │   ├── link_service.rs
│   │   │   │   ├── server_manager.rs
│   │   │   │   ├── lobby.rs
│   │   │   │   └── telemetry.rs
│   │   │   ├── routes/
│   │   │   │   ├── auth.rs
│   │   │   │   ├── link.rs
│   │   │   │   ├── servers.rs
│   │   │   │   ├── server_settings.rs
│   │   │   │   ├── invites.rs
│   │   │   │   ├── lobby.rs
│   │   │   │   ├── profile.rs
│   │   │   │   └── health.rs
│   │   │   ├── middleware/
│   │   │   │   ├── auth.rs
│   │   │   │   ├── tracing.rs
│   │   │   │   ├── rate_limit.rs
│   │   │   │   ├── cors.rs
│   │   │   │   └── request_id.rs
│   │   │   └── extractors/
│   │   │       ├── session.rs
│   │   │       └── locale.rs
│   │   ├── tests/
│   │   │   ├── auth_routes.rs
│   │   │   ├── link_routes.rs
│   │   │   ├── server_routes.rs
│   │   │   └── health_routes.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── identity/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── api/
│   │   │   │   ├── login.rs
│   │   │   │   ├── logout.rs
│   │   │   │   ├── refresh.rs
│   │   │   │   ├── register.rs
│   │   │   │   └── profile.rs
│   │   │   ├── account/
│   │   │   │   ├── service.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── password.rs
│   │   │   ├── device/
│   │   │   │   ├── service.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── attestation.rs
│   │   │   ├── session/
│   │   │   │   ├── service.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── cleanup.rs
│   │   │   ├── token/
│   │   │   │   ├── issuer.rs
│   │   │   │   ├── verifier.rs
│   │   │   │   └── claims.rs
│   │   │   └── storage/
│   │   │       ├── postgres.rs
│   │   │       ├── redis.rs
│   │   │       └── migrations.rs
│   │   ├── migrations/
│   │   │   ├── 0001_accounts.sql
│   │   │   ├── 0002_devices.sql
│   │   │   └── 0003_sessions.sql
│   │   ├── tests/
│   │   │   ├── login_flow.rs
│   │   │   ├── refresh_flow.rs
│   │   │   └── device_binding.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── link-service/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── api/
│   │   │   │   ├── create_code.rs
│   │   │   │   ├── verify_code.rs
│   │   │   │   ├── claim_code.rs
│   │   │   │   └── link_status.rs
│   │   │   ├── code/
│   │   │   │   ├── generator.rs
│   │   │   │   ├── verifier.rs
│   │   │   │   ├── expiry.rs
│   │   │   │   ├── alphabet.rs
│   │   │   │   └── rate_limit.rs
│   │   │   ├── binding/
│   │   │   │   ├── apk_session.rs
│   │   │   │   ├── browser_session.rs
│   │   │   │   ├── binding_state.rs
│   │   │   │   └── finalizer.rs
│   │   │   ├── storage/
│   │   │   │   ├── postgres.rs
│   │   │   │   ├── redis.rs
│   │   │   │   └── memory.rs
│   │   │   └── workers/
│   │   │       ├── expiry_cleanup.rs
│   │   │       └── replay_guard.rs
│   │   ├── tests/
│   │   │   ├── code_generation.rs
│   │   │   ├── code_claim.rs
│   │   │   └── expiry_cleanup.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── server-manager/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── api/
│   │   │   │   ├── create_server.rs
│   │   │   │   ├── update_server.rs
│   │   │   │   ├── stop_server.rs
│   │   │   │   ├── delete_server.rs
│   │   │   │   └── list_templates.rs
│   │   │   ├── create/
│   │   │   │   ├── validator.rs
│   │   │   │   ├── planner.rs
│   │   │   │   └── service.rs
│   │   │   ├── allocate/
│   │   │   │   ├── region_picker.rs
│   │   │   │   ├── cluster_picker.rs
│   │   │   │   └── capacity_guard.rs
│   │   │   ├── lifecycle/
│   │   │   │   ├── starter.rs
│   │   │   │   ├── stopper.rs
│   │   │   │   ├── archiver.rs
│   │   │   │   └── reaper.rs
│   │   │   ├── templates/
│   │   │   │   ├── loader.rs
│   │   │   │   ├── registry.rs
│   │   │   │   └── defaults.rs
│   │   │   ├── quotas/
│   │   │   │   ├── account.rs
│   │   │   │   ├── region.rs
│   │   │   │   └── runtime.rs
│   │   │   ├── visibility/
│   │   │   │   ├── policy.rs
│   │   │   │   └── invites.rs
│   │   │   └── storage/
│   │   │       ├── postgres.rs
│   │   │       ├── redis.rs
│   │   │       └── event_log.rs
│   │   ├── tests/
│   │   │   ├── create_server.rs
│   │   │   ├── quotas.rs
│   │   │   └── lifecycle.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── lobby/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── api/
│   │   │   │   ├── list_public.rs
│   │   │   │   ├── list_owned.rs
│   │   │   │   ├── get_details.rs
│   │   │   │   └── get_invites.rs
│   │   │   ├── browser/
│   │   │   │   ├── service.rs
│   │   │   │   ├── search.rs
│   │   │   │   └── sort.rs
│   │   │   ├── filters/
│   │   │   │   ├── region.rs
│   │   │   │   ├── visibility.rs
│   │   │   │   ├── mode.rs
│   │   │   │   └── capacity.rs
│   │   │   ├── presence/
│   │   │   │   ├── updater.rs
│   │   │   │   └── aggregator.rs
│   │   │   └── status/
│   │   │       ├── mapper.rs
│   │   │       └── health.rs
│   │   ├── tests/
│   │   │   ├── filters.rs
│   │   │   └── browser.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── matchmaking/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── api/
│   │   │   │   ├── enqueue.rs
│   │   │   │   ├── cancel.rs
│   │   │   │   └── status.rs
│   │   │   ├── queues/
│   │   │   │   ├── queue.rs
│   │   │   │   ├── ticket.rs
│   │   │   │   └── cleanup.rs
│   │   │   ├── regions/
│   │   │   │   ├── latency.rs
│   │   │   │   ├── policy.rs
│   │   │   │   └── fallback.rs
│   │   │   ├── rules/
│   │   │   │   ├── mode.rs
│   │   │   │   ├── skill.rs
│   │   │   │   └── party.rs
│   │   │   └── placement/
│   │   │       ├── planner.rs
│   │   │       ├── allocator.rs
│   │   │       └── notifier.rs
│   │   ├── tests/
│   │   │   ├── queue.rs
│   │   │   ├── region_policy.rs
│   │   │   └── placement.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── session-orchestrator/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── api/
│   │   │   │   ├── assign.rs
│   │   │   │   ├── reconnect.rs
│   │   │   │   └── terminate.rs
│   │   │   ├── assignment/
│   │   │   │   ├── planner.rs
│   │   │   │   ├── reservation.rs
│   │   │   │   └── commit.rs
│   │   │   ├── transitions/
│   │   │   │   ├── pending.rs
│   │   │   │   ├── starting.rs
│   │   │   │   ├── online.rs
│   │   │   │   └── draining.rs
│   │   │   ├── start/
│   │   │   │   ├── runtime_boot.rs
│   │   │   │   └── health_wait.rs
│   │   │   ├── stop/
│   │   │   │   ├── graceful.rs
│   │   │   │   └── forced.rs
│   │   │   └── recovery/
│   │   │       ├── reconnect.rs
│   │   │       ├── resync.rs
│   │   │       └── failover.rs
│   │   ├── tests/
│   │   │   ├── assignment.rs
│   │   │   └── recovery.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── realtime-gateway/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── connection/
│   │   │   │   ├── accept.rs
│   │   │   │   ├── session.rs
│   │   │   │   ├── heartbeat.rs
│   │   │   │   └── close.rs
│   │   │   ├── handshake/
│   │   │   │   ├── auth.rs
│   │   │   │   ├── protocol.rs
│   │   │   │   └── version.rs
│   │   │   ├── session_map/
│   │   │   │   ├── registry.rs
│   │   │   │   ├── shard.rs
│   │   │   │   └── lookup.rs
│   │   │   ├── relay/
│   │   │   │   ├── upstream.rs
│   │   │   │   ├── downstream.rs
│   │   │   │   └── fanout.rs
│   │   │   ├── compression/
│   │   │   │   ├── encoder.rs
│   │   │   │   └── decoder.rs
│   │   │   └── metrics/
│   │   │       ├── counters.rs
│   │   │       ├── histograms.rs
│   │   │       └── labels.rs
│   │   ├── tests/
│   │   │   ├── handshake.rs
│   │   │   ├── relay.rs
│   │   │   └── heartbeat.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── runtimes/
│   │   ├── world-runtime/
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── bootstrap.rs
│   │   │   │   ├── simulation/
│   │   │   │   │   ├── tick_loop.rs
│   │   │   │   │   ├── scheduler.rs
│   │   │   │   │   └── command_queue.rs
│   │   │   │   ├── world_state/
│   │   │   │   │   ├── loader.rs
│   │   │   │   │   ├── store.rs
│   │   │   │   │   └── mutator.rs
│   │   │   │   ├── replication/
│   │   │   │   │   ├── snapshot_builder.rs
│   │   │   │   │   ├── delta_builder.rs
│   │   │   │   │   └── priority.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── movement.rs
│   │   │   │   │   ├── interaction.rs
│   │   │   │   │   ├── crafting.rs
│   │   │   │   │   └── moderation.rs
│   │   │   │   └── snapshots/
│   │   │   │       ├── persist.rs
│   │   │   │       ├── restore.rs
│   │   │   │       └── prune.rs
│   │   │   ├── tests/
│   │   │   │   ├── tick_loop.rs
│   │   │   │   ├── replication.rs
│   │   │   │   └── commands.rs
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   ├── coop-runtime/
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── objectives.rs
│   │   │   │   ├── enemy_waves.rs
│   │   │   │   ├── rewards.rs
│   │   │   │   └── session.rs
│   │   │   ├── tests/
│   │   │   │   ├── objectives.rs
│   │   │   │   └── rewards.rs
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   ├── pvp-runtime/
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── ranking.rs
│   │   │   │   ├── teams.rs
│   │   │   │   ├── scoring.rs
│   │   │   │   └── anti_cheat.rs
│   │   │   ├── tests/
│   │   │   │   ├── scoring.rs
│   │   │   │   └── anti_cheat.rs
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   ├── sandbox-runtime/
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── ownership.rs
│   │   │   │   ├── building.rs
│   │   │   │   ├── permissions.rs
│   │   │   │   └── cleanup.rs
│   │   │   ├── tests/
│   │   │   │   ├── ownership.rs
│   │   │   │   └── permissions.rs
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   └── event-runtime/
│   │       ├── src/
│   │       │   ├── main.rs
│   │       │   ├── config.rs
│   │       │   ├── schedule.rs
│   │       │   ├── rules.rs
│   │       │   └── rewards.rs
│   │       ├── tests/
│   │       │   ├── schedule.rs
│   │       │   └── rewards.rs
│   │       ├── Cargo.toml
│   │       └── README.md
│   ├── social/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── api/
│   │   │   │   ├── friends.rs
│   │   │   │   ├── blocks.rs
│   │   │   │   ├── presence.rs
│   │   │   │   └── invites.rs
│   │   │   ├── friends/
│   │   │   │   ├── service.rs
│   │   │   │   └── repository.rs
│   │   │   ├── blocks/
│   │   │   │   ├── service.rs
│   │   │   │   └── repository.rs
│   │   │   ├── presence/
│   │   │   │   ├── service.rs
│   │   │   │   └── cache.rs
│   │   │   └── invites/
│   │   │       ├── service.rs
│   │   │       └── repository.rs
│   │   ├── tests/
│   │   │   ├── friends.rs
│   │   │   └── blocks.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── chat/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── api/
│   │   │   │   ├── send.rs
│   │   │   │   ├── history.rs
│   │   │   │   └── presence.rs
│   │   │   ├── channels/
│   │   │   │   ├── public.rs
│   │   │   │   ├── private.rs
│   │   │   │   ├── system.rs
│   │   │   │   └── runtime.rs
│   │   │   ├── delivery/
│   │   │   │   ├── enqueue.rs
│   │   │   │   ├── dispatch.rs
│   │   │   │   └── retention.rs
│   │   │   ├── filters/
│   │   │   │   ├── profanity.rs
│   │   │   │   ├── spam.rs
│   │   │   │   └── flood.rs
│   │   │   └── moderation_hooks/
│   │   │       ├── report.rs
│   │   │       └── sanction.rs
│   │   ├── tests/
│   │   │   ├── profanity.rs
│   │   │   └── retention.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── persistence/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs
│   │   │   ├── api/
│   │   │   │   ├── player.rs
│   │   │   │   ├── realm.rs
│   │   │   │   ├── session.rs
│   │   │   │   └── world.rs
│   │   │   ├── player/
│   │   │   │   ├── repository.rs
│   │   │   │   └── mapper.rs
│   │   │   ├── realm/
│   │   │   │   ├── repository.rs
│   │   │   │   └── mapper.rs
│   │   │   ├── session/
│   │   │   │   ├── repository.rs
│   │   │   │   └── mapper.rs
│   │   │   ├── world/
│   │   │   │   ├── repository.rs
│   │   │   │   ├── mapper.rs
│   │   │   │   └── snapshot.rs
│   │   │   └── storage/
│   │   │       ├── postgres.rs
│   │   │       ├── object_store.rs
│   │   │       └── transaction.rs
│   │   ├── migrations/
│   │   │   ├── 0001_players.sql
│   │   │   ├── 0002_realms.sql
│   │   │   ├── 0003_sessions.sql
│   │   │   └── 0004_world_snapshots.sql
│   │   ├── tests/
│   │   │   ├── player_repo.rs
│   │   │   └── world_snapshot.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── moderation/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── api/
│   │   │   │   ├── reports.rs
│   │   │   │   ├── sanctions.rs
│   │   │   │   └── appeals.rs
│   │   │   ├── reports/
│   │   │   │   ├── service.rs
│   │   │   │   └── repository.rs
│   │   │   ├── sanctions/
│   │   │   │   ├── service.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── policy.rs
│   │   │   ├── automation/
│   │   │   │   ├── spam_rules.rs
│   │   │   │   ├── abuse_rules.rs
│   │   │   │   └── escalation.rs
│   │   │   └── appeals/
│   │   │       ├── service.rs
│   │   │       └── repository.rs
│   │   ├── tests/
│   │   │   ├── sanctions.rs
│   │   │   └── automation.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── telemetry/
│       ├── src/
│       │   ├── main.rs
│       │   ├── config.rs
│       │   ├── api/
│       │   │   ├── metrics.rs
│       │   │   ├── traces.rs
│       │   │   └── health.rs
│       │   ├── logs/
│       │   │   ├── sink.rs
│       │   │   └── redact.rs
│       │   ├── metrics/
│       │   │   ├── exporter.rs
│       │   │   ├── registry.rs
│       │   │   └── aggregation.rs
│       │   ├── traces/
│       │   │   ├── exporter.rs
│       │   │   ├── sampler.rs
│       │   │   └── correlation.rs
│       │   └── alerts/
│       │       ├── rules.rs
│       │       ├── notifier.rs
│       │       └── dedupe.rs
│       ├── tests/
│       │   ├── metrics.rs
│       │   └── traces.rs
│       ├── Cargo.toml
│       └── README.md
├── crates/
│   ├── core-domain/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── entity_id.rs
│   │   │   ├── account_id.rs
│   │   │   ├── realm_id.rs
│   │   │   ├── session_id.rs
│   │   │   ├── ids.rs
│   │   │   ├── errors.rs
│   │   │   ├── result.rs
│   │   │   ├── time.rs
│   │   │   └── region.rs
│   │   ├── tests/
│   │   │   ├── ids.rs
│   │   │   └── region.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── auth-model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── account.rs
│   │   │   ├── device.rs
│   │   │   ├── session.rs
│   │   │   ├── claims.rs
│   │   │   ├── refresh_token.rs
│   │   │   └── password_hash.rs
│   │   ├── tests/
│   │   │   ├── claims.rs
│   │   │   └── session.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── link-protocol/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── request.rs
│   │   │   ├── response.rs
│   │   │   ├── code.rs
│   │   │   ├── binding.rs
│   │   │   ├── status.rs
│   │   │   └── error_code.rs
│   │   ├── tests/
│   │   │   ├── request.rs
│   │   │   └── code.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── server-control/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── realm.rs
│   │   │   ├── template.rs
│   │   │   ├── visibility.rs
│   │   │   ├── limits.rs
│   │   │   ├── ownership.rs
│   │   │   ├── invite.rs
│   │   │   ├── state.rs
│   │   │   └── lifecycle.rs
│   │   ├── tests/
│   │   │   ├── limits.rs
│   │   │   └── visibility.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── game-protocol/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client_message.rs
│   │   │   ├── server_message.rs
│   │   │   ├── snapshot.rs
│   │   │   ├── event.rs
│   │   │   ├── delta.rs
│   │   │   ├── command.rs
│   │   │   ├── ack.rs
│   │   │   └── version.rs
│   │   ├── tests/
│   │   │   ├── encoding.rs
│   │   │   └── version.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── net-code/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── reliability.rs
│   │   │   ├── sequencing.rs
│   │   │   ├── ack.rs
│   │   │   ├── transport.rs
│   │   │   ├── packet.rs
│   │   │   ├── mtu.rs
│   │   │   └── congestion.rs
│   │   ├── tests/
│   │   │   ├── packet.rs
│   │   │   └── sequencing.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── world-model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── map.rs
│   │   │   ├── entity.rs
│   │   │   ├── biome.rs
│   │   │   ├── resources.rs
│   │   │   ├── rules.rs
│   │   │   ├── weather.rs
│   │   │   ├── spawn.rs
│   │   │   └── ownership.rs
│   │   ├── tests/
│   │   │   ├── map.rs
│   │   │   └── spawn.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── scenario-model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── scenario.rs
│   │   │   ├── objective.rs
│   │   │   ├── victory.rs
│   │   │   ├── rotation.rs
│   │   │   ├── modifiers.rs
│   │   │   ├── matchmaking.rs
│   │   │   └── rewards.rs
│   │   ├── tests/
│   │   │   ├── objective.rs
│   │   │   └── rewards.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── simulation/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tick.rs
│   │   │   ├── command.rs
│   │   │   ├── resolver.rs
│   │   │   ├── replication.rs
│   │   │   ├── validation.rs
│   │   │   ├── scheduler.rs
│   │   │   ├── interest.rs
│   │   │   └── rollback.rs
│   │   ├── tests/
│   │   │   ├── tick.rs
│   │   │   ├── resolver.rs
│   │   │   └── rollback.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── persistence-model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── player.rs
│   │   │   ├── realm.rs
│   │   │   ├── session.rs
│   │   │   ├── world.rs
│   │   │   ├── invite.rs
│   │   │   └── audit.rs
│   │   ├── tests/
│   │   │   ├── realm.rs
│   │   │   └── audit.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── config-model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── regions.rs
│   │   │   ├── features.rs
│   │   │   ├── networking.rs
│   │   │   ├── quotas.rs
│   │   │   ├── runtime.rs
│   │   │   └── observability.rs
│   │   ├── tests/
│   │   │   ├── quotas.rs
│   │   │   └── networking.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── i18n/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── locale.rs
│   │   │   ├── catalog.rs
│   │   │   ├── formatter.rs
│   │   │   ├── plural.rs
│   │   │   └── negotiation.rs
│   │   ├── locales/
│   │   │   ├── en/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── fr/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── de/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── es/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── pt-BR/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── ja/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   ├── ko/
│   │   │   │   ├── common.ftl
│   │   │   │   ├── auth.ftl
│   │   │   │   ├── lobby.ftl
│   │   │   │   └── servers.ftl
│   │   │   └── zh-CN/
│   │   │       ├── common.ftl
│   │   │       ├── auth.ftl
│   │   │       ├── lobby.ftl
│   │   │       └── servers.ftl
│   │   ├── tests/
│   │   │   ├── locale.rs
│   │   │   └── negotiation.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── utils/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── hashing.rs
│       │   ├── serialization.rs
│       │   ├── random.rs
│       │   ├── validation.rs
│       │   ├── pagination.rs
│       │   ├── env.rs
│       │   └── tracing.rs
│       ├── tests/
│       │   ├── hashing.rs
│       │   └── pagination.rs
│       ├── Cargo.toml
│       └── README.md
├── config/
│   ├── dev/
│   │   ├── gateway.toml
│   │   ├── identity.toml
│   │   ├── link-service.toml
│   │   ├── server-manager.toml
│   │   ├── lobby.toml
│   │   ├── matchmaking.toml
│   │   ├── realtime-gateway.toml
│   │   ├── world-runtime.toml
│   │   └── runtimes.toml
│   ├── staging/
│   │   ├── gateway.toml
│   │   ├── identity.toml
│   │   ├── link-service.toml
│   │   ├── server-manager.toml
│   │   ├── matchmaking.toml
│   │   └── runtimes.toml
│   ├── prod/
│   │   ├── gateway.toml
│   │   ├── identity.toml
│   │   ├── link-service.toml
│   │   ├── server-manager.toml
│   │   ├── matchmaking.toml
│   │   ├── telemetry.toml
│   │   └── runtimes.toml
│   ├── regions/
│   │   ├── eu-west.toml
│   │   ├── us-east.toml
│   │   ├── ap-southeast.toml
│   │   └── sa-east.toml
│   ├── gameplay/
│   │   ├── coop.toml
│   │   ├── pvp.toml
│   │   ├── sandbox.toml
│   │   ├── event.toml
│   │   ├── rotation.toml
│   │   └── progression.toml
│   └── quotas/
│       ├── free-tier.toml
│       ├── public-servers.toml
│       ├── private-servers.toml
│       ├── link-codes.toml
│       └── region-capacity.toml
├── data/
│   ├── gameplay/
│   │   ├── modes/
│   │   │   ├── coop.json
│   │   │   ├── pvp.json
│   │   │   ├── sandbox.json
│   │   │   └── event.json
│   │   ├── rotations/
│   │   │   ├── weekly.json
│   │   │   └── seasonal.json
│   │   ├── match-rules/
│   │   │   ├── public.json
│   │   │   ├── private.json
│   │   │   └── ranked.json
│   │   └── balancing/
│   │       ├── economy.json
│   │       ├── combat.json
│   │       └── crafting.json
│   ├── scenarios/
│   │   ├── coop/
│   │   │   ├── rescue_mission.json
│   │   │   ├── horde_survival.json
│   │   │   └── convoy_defense.json
│   │   ├── pvp/
│   │   │   ├── team_elimination.json
│   │   │   ├── territory_control.json
│   │   │   └── capture_core.json
│   │   ├── sandbox/
│   │   │   ├── default_build.json
│   │   │   ├── survival_build.json
│   │   │   └── creative_build.json
│   │   └── events/
│   │       ├── meteor_storm.json
│   │       ├── dark_week.json
│   │       └── world_boss.json
│   ├── worldgen/
│   │   ├── seeds/
│   │   │   ├── public-worlds.json
│   │   │   └── events.json
│   │   ├── maps/
│   │   │   ├── archipelago.json
│   │   │   ├── crater_fields.json
│   │   │   └── frozen_valley.json
│   │   └── biome-sets/
│   │       ├── temperate.json
│   │       ├── arid.json
│   │       └── alien.json
│   └── localization/
│       ├── en/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── fr/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── de/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── es/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── pt-BR/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── ja/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       ├── ko/
│       │   ├── system.json
│       │   ├── errors.json
│       │   └── gameplay.json
│       └── zh-CN/
│           ├── system.json
│           ├── errors.json
│           └── gameplay.json
├── infra/
│   ├── docker/
│   │   ├── api-gateway.Dockerfile
│   │   ├── link-service.Dockerfile
│   │   ├── server-manager.Dockerfile
│   │   ├── realtime-gateway.Dockerfile
│   │   ├── world-runtime.Dockerfile
│   │   ├── coop-runtime.Dockerfile
│   │   ├── pvp-runtime.Dockerfile
│   │   ├── sandbox-runtime.Dockerfile
│   │   └── event-runtime.Dockerfile
│   ├── kubernetes/
│   │   ├── base/
│   │   │   ├── namespace.yaml
│   │   │   ├── api-gateway.yaml
│   │   │   ├── link-service.yaml
│   │   │   ├── server-manager.yaml
│   │   │   ├── realtime-gateway.yaml
│   │   │   ├── persistence.yaml
│   │   │   ├── telemetry.yaml
│   │   │   └── ingress.yaml
│   │   ├── regions/
│   │   │   ├── eu-west/
│   │   │   │   ├── kustomization.yaml
│   │   │   │   ├── world-runtime.yaml
│   │   │   │   ├── coop-runtime.yaml
│   │   │   │   └── pvp-runtime.yaml
│   │   │   ├── us-east/
│   │   │   │   ├── kustomization.yaml
│   │   │   │   ├── world-runtime.yaml
│   │   │   │   ├── coop-runtime.yaml
│   │   │   │   └── pvp-runtime.yaml
│   │   │   ├── ap-southeast/
│   │   │   │   ├── kustomization.yaml
│   │   │   │   ├── world-runtime.yaml
│   │   │   │   └── event-runtime.yaml
│   │   │   └── sa-east/
│   │   │       ├── kustomization.yaml
│   │   │       ├── world-runtime.yaml
│   │   │       └── sandbox-runtime.yaml
│   │   └── addons/
│   │       ├── redis.yaml
│   │       ├── postgres.yaml
│   │       ├── loki.yaml
│   │       └── prometheus.yaml
│   ├── terraform/
│   │   ├── modules/
│   │   │   ├── network/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── outputs.tf
│   │   │   ├── kubernetes/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── outputs.tf
│   │   │   ├── database/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── outputs.tf
│   │   │   └── object-storage/
│   │   │       ├── main.tf
│   │   │       ├── variables.tf
│   │   │       └── outputs.tf
│   │   ├── environments/
│   │   │   ├── dev/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── backend.tf
│   │   │   ├── staging/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── backend.tf
│   │   │   └── prod/
│   │   │       ├── main.tf
│   │   │       ├── variables.tf
│   │   │       └── backend.tf
│   │   └── regions/
│   │       ├── eu-west.tfvars
│   │       ├── us-east.tfvars
│   │       ├── ap-southeast.tfvars
│   │       └── sa-east.tfvars
│   └── scripts/
│       ├── bootstrap.sh
│       ├── migrate.sh
│       ├── deploy.sh
│       ├── rollback.sh
│       ├── seed_data.sh
│       └── port_forward.sh
├── tests/
│   ├── integration/
│   │   ├── apk_link_flow.rs
│   │   ├── server_create_flow.rs
│   │   ├── invite_flow.rs
│   │   └── reconnect_flow.rs
│   ├── load/
│   │   ├── lobby_browse.js
│   │   ├── server_create.js
│   │   └── realtime_connect.js
│   ├── soak/
│   │   ├── long_world_runtime.js
│   │   └── long_realtime_gateway.js
│   ├── replay/
│   │   ├── movement_trace.json
│   │   ├── server_create_trace.json
│   │   └── desync_trace.json
│   ├── protocol/
│   │   ├── handshake_compat.rs
│   │   ├── snapshot_decode.rs
│   │   └── delta_encode.rs
│   └── region-failover/
│       ├── primary_loss.rs
│       └── reconnect_after_move.rs
└── .github/
    └── workflows/
        ├── ci.yml
        ├── tests.yml
        ├── release.yml
    └── deploy.yml
```

## Lecture par couches

### 1. apps/

Contient les applications web de pilotage.

- web-control : site public utilise par le joueur pour lier une session APK avec un code temporaire puis creer un serveur public ou prive.
- admin-console : interface interne d'administration, reservee a la moderation et a l'exploitation.

Le site web ne fait pas tourner le jeu. Il appelle uniquement les API du control plane.

### 2. services/

Contient tous les executables backend.

#### api-gateway/

Point d'entree HTTP principal.

Responsabilites :

- authentification des appels web et mobile
- rate limiting
- exposition des routes publiques
- propagation du tracing et des identifiants de requete

#### identity/

Gestion des comptes, appareils et sessions.

Responsabilites :

- compte joueur
- liaison appareil -> session
- jetons d'authentification
- revocation de session

#### link-service/

Service central pour relier l'APK et le site.

Responsabilites :

- generer un code court temporaire cote APK
- verifier ce code depuis le site
- lier une session mobile a une session navigateur
- invalider un code deja utilise ou expire

#### server-manager/

Service de creation et de gestion des serveurs visibles par les joueurs.

Responsabilites :

- creer un serveur public ou prive
- appliquer des templates de modes de jeu
- choisir une region et une capacite
- imposer les quotas gratuits
- demarrer, stopper, archiver, supprimer

#### lobby/

Catalogue des serveurs et de leur etat.

Responsabilites :

- liste des serveurs publics
- affichage des serveurs lies au joueur
- filtres par region, mode, disponibilite
- statut en ligne, hors ligne, demarrage, plein

#### matchmaking/

Utilise pour les modes automatiques et l'affectation intelligente.

Responsabilites :

- mise en file
- choix de region
- regles de placement
- distribution vers un runtime libre

#### session-orchestrator/

Supervise les transitions de session.

Responsabilites :

- affectation joueur -> runtime
- changement d'etat de session
- reprise apres incident
- fermeture propre de partie

#### realtime-gateway/

Porte d'entree temps reel entre mobile et runtime de jeu.

Responsabilites :

- handshake de connexion
- routage vers le bon runtime
- compression ou encapsulation des messages
- metriques reseau

#### runtimes/

Contient les vrais serveurs de jeu.

- world-runtime : monde persistant ou semi-persistant
- coop-runtime : parties cooperatives
- pvp-runtime : parties competitives
- sandbox-runtime : mondes libres et serveurs communautaires limites
- event-runtime : modes temporaires ou evenementiels

Chaque runtime doit rester autoritaire : le client envoie des intentions, le runtime valide et decide.

#### social/

Amis, invitations, presence.

#### chat/

Messagerie hors gameplay critique.

#### persistence/

Sauvegarde des donnees durables.

Responsabilites :

- profils joueurs
- metadonnees de serveurs
- sessions terminees
- mondes persistants

#### moderation/

Gestion des abus et sanctions.

#### telemetry/

Observabilite technique.

Responsabilites :

- logs
- metriques
- traces distribuees
- alertes

### 3. crates/

Contient le code Rust partage entre services.

#### core-domain/

Types de base communs : identifiants, erreurs, temps, region.

#### auth-model/

Modeles de compte, appareil, session et claims.

#### link-protocol/

Contrats stricts pour la liaison APK <-> site.

#### server-control/

Definitions metier d'un serveur : proprietaire, visibilite, limites, template.

#### game-protocol/

Messages echanges entre client mobile et runtime.

#### net-code/

Utilitaires reseau bas niveau pour sequence, ack, fiabilite et format de paquets.

#### world-model/

Modeles du monde et des regles globales.

#### scenario-model/

Modeles de scenarios et variantes de gameplay.

#### simulation/

Coeur de resolution serveur autoritaire.

#### persistence-model/

Schemas applicatifs des objets sauvegardes.

#### config-model/

Schemas de configuration des regions, quotas, flags et options reseau.

#### i18n/

Support multi-langue et formatage.

#### utils/

Fonctions techniques partagees, sans logique produit.

### 4. config/

Configuration versionnee par environnement et par region.

Points importants :

- separer dev, staging et prod
- separer les quotas gratuits des serveurs publics et prives
- configurer les regions de facon explicite
- versionner les modes de jeu separement

### 5. data/

Donnees de reference du jeu.

- gameplay : regles, rotations, equilibrage
- scenarios : definitions de contenu par type de runtime
- worldgen : graines, cartes, ensembles de biomes
- localization : textes metier et contenus localises

### 6. infra/

Definition du deploiement.

- docker : images par service
- kubernetes : orchestration multi-instance et multi-region
- terraform : infrastructure cloud
- scripts : automatisation d'operations courantes

### 7. tests/

Tests techniques et fonctionnels a l'echelle plateforme.

- integration : flux inter-services
- load : charge
- soak : endurance
- replay : reproduction de session
- protocol : compatibilite client/serveur
- region-failover : bascule multi-region

## Flux principal APK -> site -> serveur

```text
APK mobile
  -> demande un code temporaire au link-service
  -> affiche ce code a l'utilisateur

Utilisateur
  -> ouvre le site web-control
  -> saisit le code

web-control
  -> appelle api-gateway
  -> api-gateway transmet au link-service
  -> link-service valide la session APK

Utilisateur
  -> choisit region, mode, public/prive, nom, limite de joueurs

web-control
  -> appelle server-manager
  -> server-manager cree la definition du serveur
  -> session-orchestrator choisit un runtime cible
  -> runtime demarre ou rejoint une capacite existante

APK mobile
  -> recupere les informations du serveur lie
  -> se connecte au realtime-gateway
  -> rejoint le runtime assigne
```

## Regles produit a integrer des le debut

Comme le jeu est gratuit, la structure doit deja prevoir des limites cote backend.

### Limites recommandees

- nombre maximum de serveurs prives par compte
- temps d'expiration d'un serveur prive inactif
- nombre maximum de joueurs par mode
- limitation par region si la capacite est saturee
- reutilisation de templates plutot que serveurs totalement arbitraires

### Etats de serveur recommandes

- pending
- linking
- provisioning
- starting
- online
- full
- draining
- stopped
- archived
- failed

### Types de visibilite recommandes

- public
- private
- invite-only
- unlisted

## Ordre de construction recommande

Si tu construis ce backend progressivement, l'ordre le plus sain est :

1. core-domain, auth-model, link-protocol, server-control, config-model
2. identity, link-service, api-gateway
3. server-manager, lobby
4. realtime-gateway, simulation, game-protocol
5. world-runtime
6. coop-runtime, pvp-runtime, sandbox-runtime
7. persistence, telemetry, moderation
8. matchmaking, session-orchestrator, social, chat
9. admin-console et industrialisation multi-region

## Version MVP minimale

Si toute cette structure est trop large pour un premier jalon, voici le noyau minimal :

```text
server/
├── apps/
│   └── web-control/
├── services/
│   ├── api-gateway/
│   ├── identity/
│   ├── link-service/
│   ├── server-manager/
│   ├── lobby/
│   ├── realtime-gateway/
│   └── runtimes/
│       └── world-runtime/
├── crates/
│   ├── core-domain/
│   ├── auth-model/
│   ├── link-protocol/
│   ├── server-control/
│   ├── game-protocol/
│   ├── simulation/
│   └── config-model/
├── config/
├── infra/
└── tests/
```

Ce MVP suffit pour :

- lier une APK a un site web
- creer un serveur public ou prive
- demarrer un premier runtime de jeu
- connecter le mobile au runtime cree
- preparer ensuite l'ajout de plusieurs gameplays
