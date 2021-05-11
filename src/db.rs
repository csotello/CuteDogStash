use bcrypt::{hash, verify};
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Rating {
    pub id: u64,
    pub post_id: u64,
    pub author: String,
    pub stars: u8,
    pub comment: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: u64,
    pub author: String,
    pub ratings: Vec<Rating>,
    pub description: String,
    pub image: Vec<u8>,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
    pub ratings: Vec<Rating>,
}
impl Default for Data {
    fn default() -> Data {
        Data {
            users: Vec::new(),
            posts: Vec::new(),
            ratings: Vec::new(),
        }
    }
}
impl Data {
    pub fn create_post(&mut self, author: String, description: String, image: Vec<u8>) {
        let id = random::<u64>();
        let post = Post {
            id,
            author,
            description,
            image,
            ratings: vec![],
        };
        self.posts.push(post);
    }
    pub fn check_username(&self, username: String) -> bool {
        for user in self.users.iter() {
            if user.username == username {
                return false;
            }
        }
        true
    }
    pub fn create_user(&mut self, username: String, password: String) {
        let id = random::<u64>();
        let hash = hash(password, 4).unwrap();
        self.users.push(User {
            id,
            username,
            password: hash,
        });
    }
    pub fn login(&self, username: String, password: String) -> Option<User> {
        for user in self.users.iter() {
            if user.username == username && verify(password.clone(), &user.password).unwrap() {
                return Some(user.clone());
            }
        }
        None
    }
}
