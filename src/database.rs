use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
};

#[derive(Debug)]
pub struct Storage {
    documents: RefCell<HashMap<usize, String>>,
    tokens: RefCell<HashMap<usize, String>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            documents: RefCell::new(HashMap::new()),
            tokens: RefCell::new(HashMap::new()),
        }
    }

    fn unique_id(&self, title: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write(title.as_bytes());
        hasher.finish() as usize
    }

    pub fn add_document(&self, title: &str, title_size: usize, body: &str, body_size: usize) {
        let document_id = self.get_document_id(title);
        self.documents
            .borrow_mut()
            .insert(document_id, title.to_string());
    }

    pub fn get_document_id(&self, title: &str) -> usize {
        self.unique_id(title)
    }

    pub fn get_token_id(&self, token: &str) -> usize {
        self.create_token_id(token)
    }

    pub fn create_token_id(&self, token: &str) -> usize {
        let token_id = self.unique_id(token);
        self.tokens.borrow_mut().insert(token_id, token.to_string());
        token_id
    }
}
