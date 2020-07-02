use bson::doc;
// use bson::ordered::OrderedDocument;
use chrono::Utc;
use mongodb::results::InsertOneResult;
use mongodb::{error::Error, Collection};

// module dependencies// module declarations
use crate::models::NewPerson;

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection,
}

impl UserRepository {
    pub fn new(collection: Collection) -> Self {
        Self { collection }
    }

    pub fn create(&self, person: NewPerson) -> Result<InsertOneResult, Error> {
        // let p = to_bson(&person)?;
        // self.collection
        //     .insert_one(Ok(p.as_document()),  None)
        self.collection.insert_one(
            doc! {
               "firstname": person.firstname,
               "lastname": person.lastname,
               "email": person.email,
               "createdon": Utc::now().timestamp_millis(),
            },
            None,
        )
    }

    // pub fn find(&self) -> Result<Option<OrderedDocument>, Error> {
    //     self.collection.find_one(doc! {}, None)
    // }
}
