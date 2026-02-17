// Custom error types for WebSocket service
// Generated stub for websocket

use use thiserror::Error;
use use std::fmt;

/// WebSocket service error types
pub struct WebSocketError {
    pub ConnectionClosed,
    pub InvalidMessage(String),
    pub SerializationError(String),
    pub RepositoryError(String),
    pub NotFound(String),
    pub AlreadyExists(String),
    pub Internal(String),
}
