#[derive(Debug, Clone)]
pub struct Session {
    pub user: String,
    pub session_token: String,
    pub expiration: u64, // Session expiration time
}

impl Session {
    pub fn new(user: &str, session_token: &str, expiration: u64) -> Self {
        Session {
            user: user.to_string(),
            session_token: session_token.to_string(),
            expiration,
        }
    }

    // Check if the session is still valid
    pub fn is_session_valid(&self) -> bool {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < self.expiration
    }
}

pub struct MultiSessionManager {
    pub sessions: Vec<Session>,
}

impl MultiSessionManager {
    pub fn new() -> Self {
        MultiSessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    // End a session by token
    pub fn end_session(&mut self, session_token: &str) {
        self.sessions.retain(|s| s.session_token != session_token);
    }

    // End all sessions for a user
    pub fn end_all_sessions(&mut self, user: &str) {
        self.sessions.retain(|s| s.user != user);
    }
}
