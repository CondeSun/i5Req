use base64::{Engine, engine::general_purpose};
use serde::Serialize;

use crate::types::i5_error::I5RequestError;

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
    /// Create a new Field Object
    fn new(name: impl Into<String>, value: impl Into<String>, item_number: i32) -> Field {
        Field {
            name: name.into(),
            value: value.into(),
            item_number,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct File {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Key")]
    key: Option<String>,
    #[serde(rename = "Data")]
    data_base64: String,
}

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
    /// Create a new Document Object
    fn new(document_name: impl Into<String>) -> Document {
        Document {
            name: document_name.into(),
            fields: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Add a new Header Field to a Document
    pub fn add_header_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.fields.push(Field::new(name, value, 0));
        self
    }

    /// Add a new Item Field to a Document
    pub fn add_item_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        item_number: i32,
    ) -> &mut Self {
        self.fields.push(Field::new(name, value, item_number));
        self
    }

    /// Add File from base64 String
    pub fn add_base64_file(&mut self, name: impl Into<String>, base64: String) {
        self.files.push(File::new(name, base64));
    }

    /// Add File from Byte Slice
    pub fn add_bytes_file(&mut self, name: impl Into<String>, bytes: &[u8]) {
        let base64_string = general_purpose::STANDARD.encode(bytes);
        self.add_base64_file(name, base64_string);
    }
}

#[derive(Serialize, Debug)]
pub struct I5Reqeust {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Documents")]
    documents: Vec<Document>,
}

impl I5Reqeust {
    /// Create a new I5Reqeust Object
    pub fn new(name: impl Into<String>) -> I5Reqeust {
        I5Reqeust {
            name: name.into(),
            documents: Vec::new(),
        }
    }

    /// Add a new document to I5Reqeust Request Struct
    pub fn add_document(&mut self, document_name: impl Into<String>) -> usize {
        self.documents.push(Document::new(document_name));
        self.documents.len() - 1
    }

    /// Get Document Mutable Reference
    pub fn get_document_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.documents.get_mut(index)
    }

    /// Get Document Reference
    pub fn get_document(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)
    }

    /// Check if current Request Object is valid.
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

    /// Validate Request Object.
    pub fn validate(self) -> Result<ValidatedI5Request, I5RequestError> {
        if self.is_valid() {
            Ok(ValidatedI5Request(self))
        } else {
            Err(I5RequestError::ValidationError)
        }
    }
}

pub struct ValidatedI5Request(I5Reqeust);

impl ValidatedI5Request {
    pub fn to_json_string(&self) -> Result<String, I5RequestError> {
        serde_json::to_string(&self.0).map_err(|e| I5RequestError::SerializeError)
    }
}
