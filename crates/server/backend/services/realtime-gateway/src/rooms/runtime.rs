use std::collections::HashMap;

use chrono::Utc;
use game_protocol::{PlayerPresence, ServerEnvelope, ServerEvent, SessionPhase};
use serde_json::Value;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct RoomRuntime {
    sender: broadcast::Sender<String>,
    players: HashMap<Uuid, PlayerPresence>,
    sequence: u64,
    phase: SessionPhase,
}

impl RoomRuntime {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(512);
        Self {
            sender,
            players: HashMap::new(),
            sequence: 0,
            phase: SessionPhase::Lobby,
        }
    }

    pub fn sender(&self) -> broadcast::Sender<String> {
        self.sender.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    pub fn join(&mut self, server_id: Uuid, player_id: Uuid) -> (String, String, String, Option<String>) {
        let presence = PlayerPresence {
            player_id,
            connected_at: Utc::now(),
        };
        self.players.insert(player_id, presence.clone());

        let welcome = Self::encode(ServerEvent::Welcome { server_id, player: presence.clone() });
        let snapshot = Self::encode(ServerEvent::PresenceSnapshot {
            server_id,
            players: self.players.values().cloned().collect::<Vec<_>>(),
        });
        let joined = Self::encode(ServerEvent::PlayerJoined {
            server_id,
            player: presence,
        });

        let phase_event = self.transition(server_id);
        (welcome, snapshot, joined, phase_event)
    }

    pub fn leave(&mut self, server_id: Uuid, player_id: Uuid) -> Option<(String, Option<String>)> {
        self.players.remove(&player_id)?;
        let left = Self::encode(ServerEvent::PlayerLeft { server_id, player_id });
        let phase_event = self.transition(server_id);
        Some((left, phase_event))
    }

    pub fn action(
        &mut self,
        server_id: Uuid,
        player_id: Uuid,
        kind: String,
        payload: Value,
    ) -> Result<String, String> {
        if self.phase != SessionPhase::Running {
            return Err(Self::encode_error("invalid_phase", "actions are only allowed in running phase"));
        }
        if kind.trim().is_empty() || kind.len() > 48 {
            return Err(Self::encode_error("invalid_action", "kind is invalid"));
        }

        self.sequence = self.sequence.saturating_add(1);
        Ok(Self::encode(ServerEvent::ActionBroadcast {
            server_id,
            player_id,
            kind,
            payload,
            sequence: self.sequence,
        }))
    }

    pub fn heartbeat(&self, server_id: Uuid) -> String {
        Self::encode(ServerEvent::Heartbeat {
            server_id,
            server_time: Utc::now(),
            sequence: self.sequence,
        })
    }

    pub fn pong(&self, server_id: Uuid, client_time: chrono::DateTime<Utc>) -> String {
        Self::encode(ServerEvent::Pong {
            server_id,
            client_time,
            server_time: Utc::now(),
        })
    }

    pub fn encode_error(code: &str, message: &str) -> String {
        Self::encode(ServerEvent::Error {
            code: code.to_string(),
            message: message.to_string(),
        })
    }

    pub fn phase_message(&self, server_id: Uuid) -> String {
        Self::encode(ServerEvent::PhaseChanged {
            server_id,
            phase: self.phase.clone(),
        })
    }

    fn transition(&mut self, server_id: Uuid) -> Option<String> {
        let next = if self.players.len() >= 2 {
            SessionPhase::Running
        } else if self.players.is_empty() {
            SessionPhase::Ended
        } else {
            SessionPhase::Lobby
        };
        if next == self.phase {
            return None;
        }
        self.phase = next.clone();
        Some(Self::encode(ServerEvent::PhaseChanged { server_id, phase: next }))
    }

    fn encode(event: ServerEvent) -> String {
        serde_json::to_string(&ServerEnvelope { event }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use uuid::Uuid;

    use super::RoomRuntime;

    #[test]
    fn running_phase_requires_two_players() {
        let server_id = Uuid::new_v4();
        let mut room = RoomRuntime::new();
        let player_a = Uuid::new_v4();
        let player_b = Uuid::new_v4();

        let (_, _, _, phase_a) = room.join(server_id, player_a);
        assert!(phase_a.is_none());

        let (_, _, _, phase_b) = room.join(server_id, player_b);
        assert!(phase_b.is_some());

        let action = room.action(server_id, player_a, "move".to_string(), json!({"x":1}));
        assert!(action.is_ok());
    }

    #[test]
    fn action_is_rejected_in_lobby() {
        let server_id = Uuid::new_v4();
        let mut room = RoomRuntime::new();
        let player = Uuid::new_v4();
        room.join(server_id, player);

        let action = room.action(server_id, player, "move".to_string(), json!({"x":1}));
        assert!(action.is_err());
    }
}
