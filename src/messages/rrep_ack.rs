use crate::messages::headers::Type;

#[allow(dead_code)]
pub struct RouteResponseAck {
    ty: Type,
}

impl Default for RouteResponseAck {
    fn default() -> Self {
        Self { ty: Type::Ack }
    }
}
