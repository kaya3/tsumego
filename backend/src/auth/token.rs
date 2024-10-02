use actix_web::HttpMessage;

/// Represents an action to be taken to update the client's cookie, if any.
/// This action should be applied to the response before it is sent.
pub enum AuthTokenAction {
    Issue(String),
    Revoke,
    DoNothing,
}

impl AuthTokenAction {
    pub fn insert_into_request(self, request: &impl HttpMessage) {
        request.extensions_mut()
            .insert(self);
    }
    
    pub fn take_from_request(request: &impl HttpMessage, otherwise: Self) -> Self {
        request.extensions_mut()
            .remove()
            .unwrap_or(otherwise)
    }
}