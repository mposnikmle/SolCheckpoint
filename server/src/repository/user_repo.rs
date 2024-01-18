use std::env;
extern crate argon2;
use argon2::Config;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, 
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use futures::stream::TryStreamExt; 
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("SolCheckpoint");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        
        // Hash the password using argon2
        let password = b"password";
        let salt = b"randomsalt";
        let config = Config::default();
        let hashed_password = argon2::hash_encoded(password, salt, &config).unwrap();

        let new_doc = User {
            id: None,
            email: new_user.email,
            password: hashed_password,
            username: new_user.username,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)


    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
}

