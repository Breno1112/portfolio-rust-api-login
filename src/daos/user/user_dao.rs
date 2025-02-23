use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub active: bool
}

impl User {
    pub fn new(id: String, name: String, age: u32, active: bool) -> User {
        return User {
            id: id,
            name: name,
            age: age,
            active: active
        };
    }
}

pub trait UserDAO {
    async fn find_all(&self) -> Vec<User>;
    async fn find_by_id(&self, id: &str) -> Option<User>;
    async fn update(&self, new_value: &User) -> bool;
    async fn delete(&self, id: &str) -> bool;
    async fn insert_one(&self, user: &User) -> bool;
    async fn insert_many(&self, users: &Vec<User>) -> bool;
}