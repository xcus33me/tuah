use std::{collections::HashMap, io::Read, sync::{Arc, Mutex}};

use futures::channel::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Hub {
    clients: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl Hub {
    pub fn new() -> Arc<Self> {
        Arc::new(Hub {
            clients: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn add_client(self: Arc<Self>, client_id: String, sender: UnboundedSender<Message>) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(client_id, sender);
    }

    pub fn remove_client(self: Arc<Self>, client_id: &str) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(client_id);
    }

    pub fn broadcast(self: Arc<Self>, sender_id: &str, message: Message) {
        let clients = self.clients.lock().unwrap();
        for (client_id, sender) in clients.iter() {
            if client_id != sender_id {
                let _ = sender.unbounded_send(message.clone());
            }
        }
    }
}