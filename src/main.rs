// dependencies
use log::{info, debug};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compress, Logger};
use mongodb::{Client, options::ClientOptions};
use listenfd::ListenFd;
use std::{env, io};
use dotenv::dotenv;

// module declarations
mod api;
mod models;
mod repository;

use repository::UserRepository;
// create serices container
pub struct Services {
    users: UserRepository
}

// create servoce instantiation methods
impl Services {
    // instatiates a service with new database repo
    fn new(users: UserRepository) -> Self {
        Self { users }
    }
}

// create app state to hold Services
pub struct AppState {
    services: Services
}


// create config struct for debugging 
#[derive(Debug)]
pub struct Config {
    host: String,
    port: String,
    mongo_uri: String,
    database: String,
    collection: String
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let mongo_uri = env::var("MONGO_LOCAL").expect("Database URI not set");
    let database = env::var("DATABASE").expect("Database not set");
    let collection = env::var("COLLECTION").expect("Collection name not set");

    // create and output d
    let c = Config {
        host: host.to_string(),
        port: port.to_string(),
        mongo_uri: mongo_uri.to_string(),
        database: database.to_string(),
        collection: collection.to_string()
    };

    debug!("DEBUG {:#?}", c);
    // systemd/catflap socket activation
    let mut listenfd = ListenFd::from_env();
    
    let options = ClientOptions::parse(mongo_uri.as_str()).unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(database.as_str());
    let users = db.collection(collection.as_str());
    // println!("[INFO]: {:?}", mongo_uri.as_str());
    
    // setup actix-web server
    let mut server = HttpServer::new(move || {
        let services = Services::new(UserRepository::new(users.clone()));
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .data(AppState { services })
            .route("/", web::get().to(api::ping))
            .route("/signup", web::post().to(api::signup))
    });
    
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", host, port))?
    };

    info!("Starting server");
    server.run().await
}
