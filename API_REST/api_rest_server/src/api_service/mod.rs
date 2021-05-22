use bson::{doc, Document};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub author: String,
}

#[derive(Clone)]
pub struct ApiService {
    collection: Collection,
}


/// Transform user to mongo db document
fn data_to_document(data: &Data) -> Document {
    let Data {
        title,
        author,
    } = data;
    doc! {
        "title": title,
        "author": author,
    }
}

impl ApiService {
    pub fn new(collection: Collection) -> ApiService {
        ApiService { collection }
    }

    /// Insert user in mongo db (user)
    pub fn create(&self, _data:&Data) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(data_to_document(_data), None)
    }

    /// Update existing user in mongo db (email)
    pub fn update(&self, _data:&Data, _param: &String) -> Result<UpdateResult, Error> {
        // let Data {
        //     title: _title,
        //     author: _author,
        // } = _data;
        let object_param = bson::oid::ObjectId::with_string(_param).unwrap();
        self.collection.update_one(doc! { "_id": object_param }, data_to_document(_data), None)
    }

    /// Delete existing user in mongo db (email)
    pub fn delete(&self, _title: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "title": _title }, None)
    }

    // Get all documents
    pub fn get_json(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    /// Retrieve parameter
    pub fn get_by(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! { "author": { "$regex": param } }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}
