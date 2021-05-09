// External libraries
use tokio::stream::StreamExt;
use mongodb::bson::{doc};
use mongodb::Client;
use tokio;
// Native libraries
use std::env;

#[tokio::main]  // Necesary to implement Mongo Client
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // System parameters
    let args: Vec<String> = env::args().collect(); // Access to arg collection
    let env : String = args[1].to_string();
    let input : String = args[2].to_string();
    // Global variable
    let client_url : &str;

    // Env selector
    if env == "dev" || env == "qa" {
        client_url = "mongodb://localhost:27017";
    }
    else if env == "test" || env == "prod" {
        client_url = "mongodb://mongo-0.mongo:27017,mongo-1.mongo:27017,mongo-2.mongo:27017/admin?replicaSet=rs0";
    }
    else {
        client_url = "mongodb://mongo-0.mongo:27017,mongo-1.mongo:27017,mongo-2.mongo:27017/admin?replicaSet=rs0";
    }

	// Action
    if input == "find" {
    	// Find something in mongo
        _find(client_url).await?;
    }
    else if input == "insert" {
    	// Insert many thing in mongo
        _insert(client_url).await?;
    }
    else if input == "delete" {
    	// Delete something in mongo
        _delete(client_url).await?;
    }
    else {
    	// Default
    	println!("Action needed");
    }

    // Return fine
    Ok(())
}

// Find function
async fn _find(_ref: &str) -> Result<(), Box<dyn std::error::Error>> {
	// Mongo conection to database
    let _client = Client::with_uri_str(_ref).await?;    
    // Select database
    let _db = _client.database("test");
    // Selcet collection
    let _collection = _db.collection("test");
    // Query find
    let mut cursor = _collection.find(doc! { "author": "George Orwell" }, None).await?;
	// Iterate over the results of the cursor.
	while let Some(result) = cursor.next().await {
	    match result {
	        Ok(document) => {
	        	// Pint result
	            println!("{:?}", document);
	        }
	        // Error catch
	        Err(e) => return Err(e.into()),
	    }
	}
	// All fine
    Ok(())
}

// Insert function
async fn _insert(_ref: &str) -> Result<(), Box<dyn std::error::Error>> {
	// Mongo conection to database
    let _client = Client::with_uri_str(_ref).await?;    
    // Selcet database
    let _db = _client.database("test");
    // Selcet collection
    let _collection = _db.collection("test");
    // Document construction
    let docs = vec![
	    doc! { "title": "1984", "author": "George Orwell" },
	    doc! { "title": "Animal Farm", "author": "George Orwell" },
	    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
	];
	// Insert some documents
	_collection.insert_many(docs, None).await?;

    Ok(())
}

// Delete function
async fn _delete(_ref: &str) -> Result<(), Box<dyn std::error::Error>> {
	// Mongo conection to database
    let _client = Client::with_uri_str(_ref).await?;    
    // Selcet database
    let _db = _client.database("test");
    // Selcet collection
    let _collection = _db.collection("test");
    // Delete quiery
    _collection.delete_one(doc! { "author": "George Orwell" }, None).await?;

    Ok(())
}