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
    pub image: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
}
impl Default for Data {
    fn default() -> Data {
        Data {
            users: Vec::new(),
            posts: Vec::new(),
        }
    }
}
impl Data {
    pub fn create_post(&mut self, author: String, description: String, image: String) {
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
    pub fn create_rating(&mut self, post_id: u64, author: String, stars: u8, comment: String) {
        let id = random::<u64>();
        let rating = Rating {
            id,
            post_id,
            author,
            stars,
            comment,
        };
        for post in &mut self.posts {
            if post.id == post_id {
                post.ratings.push(rating.clone());
            }
        }
    }
    pub fn get_posts(&self, author: String) -> Vec<Post> {
        let mut posts = Vec::new();
        for post in &self.posts {
            if post.author == author {
                posts.push(post.clone());
            }
        }
        posts
    }
    pub fn delete_account(&mut self, username: String){
        let mut found = false;
        for user in &self.users{
            if user.username == username{
                found = true;
            }
        }
        if found{
            self.users.retain(|user| user.username != username);
            self.posts.retain(|post| post.author != username);
            for post in &mut self.posts{
                post.ratings.retain(|rating| rating.author != username);
            }
        }
    }
}
