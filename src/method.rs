#[derive(Debug)]
pub enum Method {
    Join,
    Send,
    PvtMsg,
}

pub fn map_method(method: &str) -> Option<Method> {
    match method {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        "3" => Some(Method::PvtMsg),
        _ => None,
    }
}
