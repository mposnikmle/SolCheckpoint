use std::env;
extern crate argon2;
use argon2::{Config, verify_encoded};
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

fn hash_password(password: &str) -> String {
    let config = Config::default();
    let password = b"password";
    let salt = b"randomsalt";
    argon2::hash_encoded(password, salt, &config).unwrap()
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

        let new_doc = User {
            id: None,
            email: new_user.email,
            password: hash_password(&new_user.password),
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

    pub async fn signin(&self, email: &str, password: &str) -> Option<User> {
        let filter = doc! { "email": email };
        let user = self.col.find_one(filter, None).await.ok()?;

        if let Some(user) = user {
            // Verify the provided password against the stored hashed password
            if verify_encoded(&user.password, password.as_bytes()).unwrap_or(false) {
                // Password is correct, return the user
                Some(user)
            } else {
                // Incorrect password
                None
            }
        } else {
            // User not found
            None
        }
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

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "email": new_user.email,
                    "password": hash_password(&new_user.password),
                    "username": new_user.username,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
}

