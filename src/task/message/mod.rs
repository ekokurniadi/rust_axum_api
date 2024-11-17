use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<T> {
    pub action: String,
    pub push_type: String,
    pub message_time: String,
    pub message_uid: String,
    pub payload: T,
}

impl<T> Message<T> where T: Serialize {}
