use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::ApiService;

mod api_router;
mod api_service;

pub struct ServiceManager {
    api: ApiService,
}

impl ServiceManager {
    pub fn new(api: ApiService) -> Self {
        ServiceManager { api }
    }
}

pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init env
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Parse a connection string into an options struct.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let client_options = ClientOptions::parse(&database_url).unwrap();

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // Get a handle to a database.
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let db = client.database(&database_name);

    // Get a handle to a collection in the database.
    let user_collection_name = env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let user_collection = db.collection(&user_collection_name);

    // server url
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    // start server
    HttpServer::new(move || {
        let user_service_worker = ApiService::new(user_collection.clone());
        let service_manager = ServiceManager::new(user_service_worker);

        // cors
        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        // launch http server
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(api_router::init)
    })
    .bind(server_url)?
    .run()
    .await
}
