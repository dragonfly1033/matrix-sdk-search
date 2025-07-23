// #![forbid(missing_docs)]
use crate::schema::RoomMessageSchema;
use crate::writer::SearchIndexWriter;
use crate::{util::*, IndexError};
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::OwnedValue;
use tantivy::{Index, IndexReader, TantivyDocument};

pub struct RoomIndex {
    schema: RoomMessageSchema,
    writer: SearchIndexWriter,
    reader: IndexReader,
    query_parser: QueryParser,
}

impl RoomIndex {
    pub fn intoo(&self) -> Index {
        self.reader.searcher().index().to_owned()
    }
}

impl RoomIndex {
    fn new_with(index: Index, schema: RoomMessageSchema) -> Result<RoomIndex, IndexError> {
        let writer = index.writer(TANTIVY_INDEX_MEMORY_BUDGET)?;
        let reader = index.reader_builder().try_into()?;

        let query_parser = QueryParser::for_index(&index, schema.default_search_fields());
        Ok(Self {
            schema,
            writer: writer.into(),
            reader,
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
        let opstamp = self.writer.force_commit()?;
        self.reader.reload()?;

        Ok(opstamp)
    }

    // TODO: probably make a query builder
    pub fn search(&self, query: &str, number_of_results: usize) -> Result<Vec<String>, IndexError> {
        let query = self.query_parser.parse_query(query)?;
        let searcher = self.reader.searcher();

        let results = searcher.search(&query, &TopDocs::with_limit(number_of_results))?;
        let mut ret: Vec<String> = Vec::new();
        let pk = self.schema.primary_key();

        for (_score, doc_address) in results {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            for f in retrieved_doc.get_all(pk) {
                // TODO: what does this really do
                match f {
                    OwnedValue::Str(s) => ret.push(s.to_string()),
                    _ => println!("how"),
                };
            }
        }

        Ok(ret)
    }
}
