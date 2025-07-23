// #![forbid(missing_docs)]
mod error;
mod index;
mod schema;
mod util;
mod writer;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{schema::*, TantivyError};
use tantivy::{Index, ReloadPolicy};

pub use error::*;
pub use index::RoomIndex;
pub use util::{Event, EventId, MilliSecondsSinceUnixEpoch, OpStamp, UserId};

pub fn search_index(index: &Index, query: &str, top_n: usize) -> Result<Vec<String>, TantivyError> {
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;
    let searcher = reader.searcher();

    let id_field = index.schema().get_field("id").unwrap();
    let message_field = index.schema().get_field("message").unwrap();

    let query_parser = QueryParser::for_index(index, vec![message_field]);
    let query = query_parser.parse_query(query)?;

    let results = searcher.search(&query, &TopDocs::with_limit(top_n))?;
    let mut ret: Vec<String> = Vec::new();

    for (_score, doc_address) in results {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        for f in retrieved_doc.get_all(id_field) {
            ret.push(f.as_str().expect("failed str conv").to_string());
        }
    }

    Ok(ret)
}
