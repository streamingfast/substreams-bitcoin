use crate::{pb::btc::v1 as pb};

impl pb::Block {
    pub fn transactions(&self) -> impl Iterator<Item = &pb::Transaction> {
        self.tx.iter()
    }
}
