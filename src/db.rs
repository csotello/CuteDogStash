use bcrypt::{hash, verify, DEFAULT_COST};
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    id: u64,
    pub username: String,
    password: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Rating {
    id: u64,
    post_id: u64,
    pub author: String,
    stars: u8,
    pub comment: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    id: u64,
    pub author: String,
    ratings: Vec<Rating>,
    pub description: String,
    image: Vec<u8>,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct DB {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
    pub ratings: Vec<Rating>,
}
impl Default for DB {
    fn default() -> DB {
        DB {
            users: Vec::new(),
            posts: Vec::new(),
            ratings: Vec::new(),
        }
    }
}
impl DB {
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
    pub fn create_user(&mut self, username: String, password: String) {
        let id = random::<u64>();
        let hash = hash(password, DEFAULT_COST).unwrap();
        self.users.push(User {
            id,
            username,
            password: hash,
        })
    }
}
