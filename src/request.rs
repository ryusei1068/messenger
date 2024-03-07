use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    method: String,
    from: String,
    to: String,
    text: String,
}

impl Request {
    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    pub fn get_from(&self) -> String {
        self.from.clone()
    }

    pub fn get_to(&self) -> String {
        self.to.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}
