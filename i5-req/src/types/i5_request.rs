use base64::{Engine, engine::general_purpose};
use serde::Serialize;

use crate::types::i5_error::I5RequestError;

/// Represents a single field within an Interface5 document.
///
/// A field can be used either as a header field (without item number) or as an item field (with item number).
///
/// # Serialization
///
/// The struct will serialize to JSON with the following keys:
/// - `"Name"`
/// - `"Value"`
/// - `"ItemNo"`
#[derive(Serialize, Debug)]
pub struct Field {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "ItemNo")]
    item_number: i32,
}

impl Field {
    /// Creates a new [`Field`].
    ///
    /// # Arguments
    /// - `name`: Field name.
    /// - `value`: Field value.
    /// - `item_number`: The associated item number (for header fields typically `0`).
    fn new(name: impl Into<String>, value: impl Into<String>, item_number: i32) -> Field {
        Field {
            name: name.into(),
            value: value.into(),
            item_number,
        }
    }
}

/// Represents a file attachment in an Interface5 document.
///
/// The file content is stored as a base64-encoded string.
#[derive(Serialize, Debug)]
pub struct File {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Key")]
    key: Option<String>,
    #[serde(rename = "Data")]
    data_base64: String,
}

/// Creates a new [`File`] from a base64-encoded data string.
///
/// # Arguments
/// - `name`: File name.
/// - `data`: Base64-encoded file data.
impl File {
    /// Create a new File Object. Data should be a base64 string!
    fn new(name: impl Into<String>, data: String) -> File {
        File {
            name: name.into(),
            key: None,
            data_base64: data,
        }
    }
}

/// Represents a single document in an Interface5 request.
///
/// Each document can contain multiple fields and files.
#[derive(Serialize, Debug)]
pub struct Document {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Fields")]
    fields: Vec<Field>,
    #[serde(rename = "Files")]
    files: Vec<File>,
}

impl Document {
    /// Creates a new empty [`Document`].
    ///
    /// # Arguments
    /// - `document_name`: Name of the document.
    fn new(document_name: impl Into<String>) -> Document {
        Document {
            name: document_name.into(),
            fields: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Adds a header field (item number `0`).
    pub fn add_header_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.fields.push(Field::new(name, value, 0));
        self
    }

    /// Adds an item field with a specific item number.
    pub fn add_item_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        item_number: i32,
    ) -> &mut Self {
        self.fields.push(Field::new(name, value, item_number));
        self
    }

    /// Adds a file from a base64-encoded string.
    pub fn add_base64_file(&mut self, name: impl Into<String>, base64: String) {
        self.files.push(File::new(name, base64));
    }

    /// Adds a file from raw bytes by automatically encoding it to base64.
    pub fn add_bytes_file(&mut self, name: impl Into<String>, bytes: &[u8]) {
        let base64_string = general_purpose::STANDARD.encode(bytes);
        self.add_base64_file(name, base64_string);
    }
}

/// Represents the complete Interface5 request payload.
///
/// A request must contain at least one document, and each document must contain at least
/// one field or one file to be considered valid.
#[derive(Serialize, Debug)]
pub struct I5Reqeust {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Documents")]
    documents: Vec<Document>,
}

impl I5Reqeust {
    /// Creates a new empty [`I5Reqeust`] with a given name.
    pub fn new(name: impl Into<String>) -> I5Reqeust {
        I5Reqeust {
            name: name.into(),
            documents: Vec::new(),
        }
    }

    /// Adds a new document to the request.
    ///
    /// Returns the index of the newly added document.
    pub fn add_document(&mut self, document_name: impl Into<String>) -> usize {
        self.documents.push(Document::new(document_name));
        self.documents.len() - 1
    }

    /// Gets a mutable reference to a document by index.
    pub fn get_document_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.documents.get_mut(index)
    }

    /// Gets an immutable reference to a document by index.
    pub fn get_document(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)
    }

    /// Checks if the request is structurally valid.
    ///
    /// Requirements:
    /// - At least one document exists.
    /// - Each document has at least one field or file.
    pub fn is_valid(&self) -> bool {
        // Request needs at least one Document.
        if self.documents.is_empty() {
            return false;
        }

        // Each Document needs at least either one field or one file.
        for document in &self.documents {
            if document.fields.is_empty() && document.files.is_empty() {
                return false;
            }
        }

        true
    }

    /// Consumes and validates the request.
    ///
    /// Returns a [`ValidatedI5Request`] on success, or an [`I5RequestError::ValidationError`] if invalid.
    pub fn validate(self) -> Result<ValidatedI5Request, I5RequestError> {
        if self.is_valid() {
            Ok(ValidatedI5Request(self))
        } else {
            Err(I5RequestError::ValidationError)
        }
    }
}

/// A wrapper type representing a validated [`I5Reqeust`] that is guaranteed to be ready for serialization and sending.
pub struct ValidatedI5Request(I5Reqeust);

impl ValidatedI5Request {
    /// Serializes the validated request into a JSON string.
    pub fn to_json_string(&self) -> Result<String, I5RequestError> {
        serde_json::to_string(&self.0).map_err(I5RequestError::SerializeError)
    }
}
