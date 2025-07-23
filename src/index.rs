use crate::schema::RoomMessageSchema;
use crate::writer::SearchIndexWriter;
use crate::{util::*, IndexError};
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::{Index, Searcher, TantivyDocument};

pub struct RoomIndex {
    schema: RoomMessageSchema,
    writer: SearchIndexWriter,
    searcher: Searcher,
    query_parser: QueryParser,
}

impl RoomIndex {
    fn new_with(index: Index, schema: RoomMessageSchema) -> Result<RoomIndex, IndexError> {
        let writer = index.writer(TANTIVY_INDEX_MEMORY_BUDGET)?;
        let reader = index.reader_builder().try_into()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, schema.default_search_fields());
        Ok(Self {
            schema,
            writer: writer.into(),
            searcher,
            query_parser,
        })
    }

    pub fn new(path: &Path) -> Result<RoomIndex, IndexError> {
        let schema = RoomMessageSchema::new();
        let index = Index::create_in_dir(path, schema.as_tantivy_schema())?;
        RoomIndex::new_with(index, schema)
    }

    pub fn new_in_ram() -> Result<RoomIndex, IndexError> {
        let schema = RoomMessageSchema::new();
        let index = Index::create_in_ram(schema.as_tantivy_schema());
        RoomIndex::new_with(index, schema)
    }

    pub fn open_or_create(path: &Path) -> Result<RoomIndex, IndexError> {
        let schema = RoomMessageSchema::new();
        let mmap_dir = MmapDirectory::open(path)?;
        let index = Index::open_or_create(mmap_dir, schema.as_tantivy_schema())?;
        RoomIndex::new_with(index, schema)
    }

    pub fn open(path: &Path) -> Result<RoomIndex, IndexError> {
        let index_path = MmapDirectory::open(path)?;
        let index = Index::open(index_path)?;
        let schema: RoomMessageSchema = index.schema().try_into()?;
        RoomIndex::new_with(index, schema)
    }

    pub fn add_event(&mut self, event: Event) -> Result<OpStamp, IndexError> {
        let doc = self
            .schema
            .make_doc(event.id(), event.body(), event.timestamp(), event.sender());
        self.writer.add_document(doc)?;
        let last_commit_opstamp = self.writer.commit()?;

        Ok(last_commit_opstamp)
    }

    pub fn force_commit(&mut self) -> Result<OpStamp, IndexError> {
        self.writer.force_commit()
    }

    // TODO: probably make a query builder
    pub fn search(&self, query: &str, number_of_results: usize) -> Result<Vec<String>, IndexError> {
        let query = self.query_parser.parse_query(query)?;

        let results = self
            .searcher
            .search(&query, &TopDocs::with_limit(number_of_results))?;
        let mut ret: Vec<String> = Vec::new();
        let pk = self.schema.primary_key();

        for (_score, doc_address) in results {
            let retrieved_doc: TantivyDocument = self.searcher.doc(doc_address)?;
            for f in retrieved_doc.get_all(pk) { // TODO: what does this really do
                ret.push(format!("{f:?}"));
            }
        }

        Ok(ret)
    }
}
