use actix_web::Scope;

pub trait Endpoint {
    /// Recieves an "/api/v*" scope and returns a scope with the endpoints added
    fn register(&self, scope: Scope) -> Scope;
}
