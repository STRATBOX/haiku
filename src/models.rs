use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    // #[serde(rename="_id")]
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPerson {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub id: String,
    pub service: String,
    pub message: String,
    pub created_on: i64,
}

// pub trait Service {
//     fn create(data: &NewPerson) -> Person;
//     fn find() -> Vec<Person>;
// }

// pub trait Repository {
//     fn create(data: &NewPerson) -> Person;
//     fn find() -> Vec<Person>;
// }
