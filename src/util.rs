#[forbid(missing_docs)]
use std::time::Duration;

pub(crate) const TANTIVY_INDEX_MEMORY_BUDGET: usize = 50_000_000;
pub(crate) const MIN_COMMIT_SIZE: usize = 500;
pub(crate) const MAX_COMMIT_TIME: Duration = Duration::from_secs(5);

pub type EventId = String;
pub type UserId = String;
pub type OpStamp = u64;
pub type MilliSecondsSinceUnixEpoch = u64;

pub struct Event {
    id: EventId,
    body: String,
    sender: UserId,
    timestamp: MilliSecondsSinceUnixEpoch,
}

impl Event {
    pub fn new(
        id: EventId,
        body: &str,
        sender: UserId,
        timestamp: MilliSecondsSinceUnixEpoch,
    ) -> Self {
        Self {
            id: id,
            body: body.to_owned(),
            sender: sender,
            timestamp: timestamp,
        }
    }

    pub(crate) fn id(&self) -> &EventId {
        &self.id
    }
    pub(crate) fn body(&self) -> &str {
        &self.body
    }
    pub(crate) fn sender(&self) -> &UserId {
        &self.sender
    }
    pub(crate) fn timestamp(&self) -> MilliSecondsSinceUnixEpoch {
        self.timestamp
    }
}
