use crate::utilities::types::IdType;

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    id: String,
    user_id: IdType,
    profile_id: IdType,
    time_until_expiration: Option<usize>,
}

impl Session {
    pub fn new(id: String, user_id: IdType, profile_id: IdType, time_until_expiration: Option<usize>) -> Self {
        Self {
            id,
            user_id,
            profile_id,
            time_until_expiration,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_user_id(&self) -> IdType {
        self.user_id
    }

    pub fn get_profile_id(&self) -> IdType {
        self.profile_id
    }

    //TODO config session expiration time
    pub fn get_time_until_expiration(&self) -> Option<usize> {
        self.time_until_expiration
    }
}

#[derive(Clone, Debug)]
pub struct SessionOption {
    session_opt: Option<SessionFromStore>,
}

impl SessionOption {
    pub fn new(session: Option<SessionFromStore>) -> Self {
        Self {
            session_opt: session,
        }
    }

    pub fn get_session_opt(&self) -> &Option<SessionFromStore> {
        &self.session_opt
    }
}

#[derive(Clone, Debug)]
pub struct SessionFromStore {
    profile_id: IdType,
}

impl SessionFromStore {
    pub fn get_profile_id(&self) -> IdType {
        self.profile_id
    }
}

impl From<Session> for SessionFromStore {
    fn from(value: Session) -> Self {
        Self {
            profile_id: value.profile_id,
        }
    }
}