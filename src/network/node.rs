use std::collections::HashMap;

extern crate uuid;
use self::uuid::Uuid;
use network::peer::{Peer};

use std::fs::{File,OpenOptions};

extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub uuid: Uuid,
    pub peers: HashMap<Uuid, Peer>,
}

impl Default for Node {
    fn default() -> Node {
        Node{
            uuid: Uuid::new_v4(),
            peers: HashMap::new(),
        }
    }
}

impl Node {
    pub fn new() -> Node {
        /* we persist the node uuid and peers in a state file. Load it. */
        File::open("state.json")
            /* Since File::open(...) and serde_json::from_reader(...) return
             * different error types, we convert both of them to a string. */
            .map_err(|err| err.to_string())
            .and_then(|file| -> Result<Node,_> {
                serde_json::from_reader(file)
                .map_err(|err| err.to_string())
            })
            /* If opening the file or deserialization failed, return the
             * default value with a random UUID and no peers. */
            .unwrap_or(Default::default())
            .save()
    }

    fn save(self) -> Node {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("state.json")
            /* Since File::open(...) and serde_json::to_writer(...) return
             * different error types, we convert both of them to a string. */
            .map_err(|err| err.to_string())
            .and_then(|file| {
                serde_json::to_writer(file, &self)
                    .map_err(|err| err.to_string())
            }).and_then(|_|Ok(self)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::panic;
    use std::env;
    extern crate tempdir;

    /* For some of the tests we need to cd to a temporary directory. Use a
     * static mutex to serialize these. */
    lazy_static! { static ref TEST_MUTEX: Mutex<()> = Mutex::new(()); }
    macro_rules! guarded {
        ($body:block) => {
            {
                let guard = TEST_MUTEX.lock().unwrap();
                if let Err(e) = panic::catch_unwind(|| { $body }) {
                    drop(guard);
                    panic::resume_unwind(e);
                }
            }
        }
    }

    #[test]
    fn test_uuid_persistence() {
        /* Test that the UUID is persisted to file */
        guarded! ({
            use self::tempdir::TempDir;
            use std::env;
            extern crate tempdir;
            let dir = TempDir::new("test_persistence")
                .expect("Could not create temporary directory");
            
            let pushd = env::current_dir().unwrap();

            /* make sure we cd back even if we panic, so that we leave a clean
             * slate for other tests. */
            if let Err(e) = panic::catch_unwind(|| {
                env::set_current_dir(dir.path())
                    .expect("could not cd to temporary directory");

                /* Make sure we changed directory */
                assert_ne!(pushd, dir.path());

                let uuid = {
                    let n = super::Node::new();
                    n.uuid
                };

                let uuid_reloaded = {
                    let n = super::Node::new();
                    n.uuid
                };

                assert_eq!(uuid, uuid_reloaded);

                use std::fs;
                fs::remove_file("state.json")
                    .expect("could not delete state file");

                let uuid_renewed = {
                    let n = super::Node::new();
                    n.uuid
                };

                assert_ne!(uuid,uuid_renewed);
            }) {
                env::set_current_dir(pushd)
                    .expect("could not cd back");
                panic::resume_unwind(e);
            }
        })
    }
}