use bevy::prelude::*;
use std::collections::VecDeque;

/// Stores recent game messages for display in the HUD
#[derive(Resource)]
pub struct MessageLog {
    messages: VecDeque<String>,
    max_messages: usize,
}

impl Default for MessageLog {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 5,
        }
    }
}

impl MessageLog {
    /// Add a new message to the log
    /// Automatically removes oldest message if at capacity
    pub fn add_message(&mut self, message: impl Into<String>) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(message.into());
    }

    /// Get an iterator over all messages (oldest to newest)
    pub fn get_messages(&self) -> impl Iterator<Item = &String> {
        self.messages.iter()
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get the number of messages currently stored
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if the log is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
