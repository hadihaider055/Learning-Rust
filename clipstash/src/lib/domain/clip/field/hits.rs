use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);


impl Hits {
    // This is used to convert the Content(String) to a inner string
    pub into_inner(self) -> String {
        self.0
    }
}
