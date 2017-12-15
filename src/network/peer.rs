extern crate nanomsg;
use self::nanomsg::{Socket};
extern crate uuid;
use self::uuid::Uuid;
use std::fmt;


#[derive(Serialize, Deserialize)]
pub struct Peer {
    pub uuid: Uuid,
    pub address: String,

    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    //FIXME: socket not used yet.
    socket: Option<Socket>,
}

impl fmt::Debug for Peer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "peer {{ uuid: {}, address: {} }}", self.uuid, self.address)
    }
}