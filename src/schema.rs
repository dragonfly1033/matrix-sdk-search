use tantivy::{
    doc,
    schema::{Field, Schema, INDEXED, STORED, STRING, TEXT},
    DateOptions, DateTime, DateTimePrecision, TantivyDocument,
};

use crate::{
    error::IndexSchemaError,
    util::{EventId, MilliSecondsSinceUnixEpoch, UserId},
};

#[derive(Debug, Clone)]
pub(crate) struct RoomMessageSchema {
    inner: Schema,
    event_id_field: Field,
    body_field: Field,
    date_field: Field,
    sender_field: Field,
    default_search_fields: Vec<Field>,
}

impl RoomMessageSchema {
    pub(crate) fn new() -> Self {
        let mut schema = Schema::builder();
        let event_id_field = schema.add_text_field("event_id", STORED | STRING);
        let body_field = schema.add_text_field("body", TEXT);

        let date_options = DateOptions::from(INDEXED)
            .set_fast()
            .set_precision(DateTimePrecision::Seconds);

        let date_field = schema.add_date_field("date", date_options);
        let sender_field = schema.add_text_field("sender", TEXT);

        let default_search_fields = vec![body_field];

        let schema = schema.build();

        Self {
            inner: schema,
            event_id_field,
            body_field,
            date_field,
            sender_field,
            default_search_fields,
        }
    }

    pub(crate) fn default_search_fields(&self) -> Vec<Field> {
        self.default_search_fields.clone()
    }

    pub(crate) fn primary_key(&self) -> Field {
        self.event_id_field
    }

    pub(crate) fn as_tantivy_schema(&self) -> Schema {
        return self.inner.clone();
    }

    pub(crate) fn make_doc(
        &self,
        event_id: &EventId,
        body: &str,
        timestamp: MilliSecondsSinceUnixEpoch,
        sender: &UserId,
    ) -> TantivyDocument {
        doc!(
            self.event_id_field => event_id.to_string(),
            self.body_field => body,
            self.date_field => DateTime::from_timestamp_millis(timestamp.try_into().unwrap_or(0)),
            self.sender_field => sender.to_string(),
        )
    }
}

impl TryFrom<Schema> for RoomMessageSchema {
    type Error = IndexSchemaError;

    fn try_from(schema: Schema) -> Result<RoomMessageSchema, Self::Error> {
        let event_id_field = schema.get_field("event_id")?;
        let body_field = schema.get_field("body")?;
        let date_field = schema.get_field("date")?;
        let sender_field = schema.get_field("sender")?;

        let default_search_fields = vec![body_field];

        Ok(Self {
            inner: schema,
            event_id_field,
            body_field,
            date_field,
            sender_field,
            default_search_fields,
        })
    }
}
