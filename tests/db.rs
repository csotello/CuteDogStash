use db::*;

#[test]
/// Test post related functions
fn post_test() {
    let mut db = Data::default();
    assert_eq!(db.posts.is_empty(), true);
    db.create_post("me".to_string(), "okay".to_string(), "123".to_string());
    assert_eq!(db.posts.is_empty(), false);
    let item = &db.posts[0];
    assert_eq!(item.author, "me".to_string());
    assert_eq!(item.description, "okay".to_string());
    let id = item.id.clone();
    let posts = db.get_posts("me".to_string());
    assert_eq!(posts.is_empty(), false);
    assert_eq!(posts[0].author, "me".to_string());
    let post = db.get_post(id);
    assert_eq!(post.is_some(), true);
    if let Some(post) = post{
        assert_eq!(post.author, "me".to_string());
    }
    db.update_post(id, "updated".to_string(), "123".to_string());
    let item = &db.posts[0];
    assert_eq!(item.author, "me".to_string());
    assert_eq!(item.description, "updated".to_string());
    db.create_rating(id, "you".to_string(), 2,"great".to_string());
    let item = &db.posts[0];
    assert_eq!(item.ratings.is_empty(), false);
    let rating = &item.ratings[0];
    assert_eq!(rating.comment, "great".to_string());
    assert_eq!(rating.stars, 2);
    db.delete_post(id);
    assert_eq!(db.posts.is_empty(), true);
}

#[test]
/// Test user related functions
fn user_test() {
    let mut db = Data::default();
    assert_eq!(db.users.is_empty(), true);
    db.create_user("name".to_string(), "pass".to_string());
    assert_eq!(db.users.is_empty(), false);
    let user = &db.users[0];
    assert_eq!(user.username.clone(), "name".to_string());
    assert_eq!(db.check_username("name".to_string()), false);
    let id = user.id.clone();
    db.update_account(id, "new_name".to_string(), "new_password".to_string());
    assert_eq!(db.check_username("name".to_string()), true);
    let user = &db.users[0];
    assert_eq!(user.username.clone(), "new_name".to_string());
    db.delete_account("new_name".to_string());
    assert_eq!(db.users.is_empty(),true);
}
