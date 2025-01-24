pub enum Endpoints {
    CreateAccount,
    CreateInviteCode,
    CreateRecord,
    CreateSession,
    RefreshSession,
    ResolveHandle,
}

impl ToString for Endpoints {
    fn to_string(&self) -> String {
        match self {
            Self::CreateAccount => "com.atproto.server.createAccount".to_string(),
            Self::CreateInviteCode => "com.atproto.server.createInviteCode".to_string(),
            Self::CreateRecord => "com.atproto.repo.createRecord".to_string(),
            Self::CreateSession => "com.atproto.server.createSession".to_string(),
            Self::RefreshSession => "com.atproto.server.refreshSession".to_string(),
            Self::ResolveHandle => "com.atproto.identity.resolveHandle".to_string(),
        }
    }
}
