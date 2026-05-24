use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::rooms::runtime::RoomRuntime;

pub struct RoomRegistry {
    rooms: HashMap<Uuid, Arc<RwLock<RoomRuntime>>>,
}

impl RoomRegistry {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn get_or_create_room(&mut self, server_id: Uuid) -> Arc<RwLock<RoomRuntime>> {
        if let Some(room) = self.rooms.get(&server_id) {
            return room.clone();
        }
        let room = Arc::new(RwLock::new(RoomRuntime::new()));
        self.rooms.insert(server_id, room.clone());
        room
    }
}

