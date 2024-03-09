use crate::method::Method;

pub fn map_method(method: &String) -> Option<Method> {
    match method.as_str() {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        "3" => Some(Method::PvtMsg),
        _ => None,
    }
}
