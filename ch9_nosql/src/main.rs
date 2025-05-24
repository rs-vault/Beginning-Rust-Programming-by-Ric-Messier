use mongodb::{sync::Client};
use mongodb::bson::doc;
use serde_json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
    occupation: String,
    location: String,
    phone: String
}

fn read_records(filename: &str)  {

    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);

    let deserializer = serde_json::Deserializer::from_reader(buf_reader);
    let iterator = deserializer.into_iter::<serde_json::Value>();
    for item in iterator {
        let p: Person = serde_json::from_str(&item.unwrap().to_string()).unwrap();
        match db_populate(p) {
            Ok(_o) => (),
            Err(e) => println!("Unable to insert data because of {}", e)
        };
    }
}

fn db_populate(record: Person) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let collection = client.database("customer_info").collection("people");

    let data = bson::to_bson(&record).unwrap();
    let document = data.as_document().unwrap();
    let insert_result = collection.insert_one(document.to_owned(), None)?;

    let data_insert_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("Inserted ID is {}", data_insert_id);

    Ok(())
}

fn main() {

    const FILENAME: &str = "people.json";

    read_records(FILENAME);

}
