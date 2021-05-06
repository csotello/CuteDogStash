use db::*;

#[test]
fn create_post() {
    let mut db = DB::default();
    assert_eq!(db.posts.is_empty(), true);
    db.create_post("me".to_string(), "okay".to_string(), vec![1, 2, 3]);
    assert_eq!(db.posts.is_empty(), false);
    let mut posts = db.posts.clone();
    let item = posts.pop().unwrap();
    assert_eq!(item.author, "me".to_string());
    assert_eq!(item.description, "okay".to_string());
}

#[test]
fn create_user() {
    let mut db = DB::default();
    assert_eq!(db.users.is_empty(), true);
    db.create_user("name".to_string(), "pass".to_string());
    assert_eq!(db.users.is_empty(), false);
}
