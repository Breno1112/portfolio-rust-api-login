use crate::daos::user::user_dao::{User, UserDAO};

pub struct UserDAOMongoDB {

}

impl UserDAO for UserDAOMongoDB {

    async fn find_all(&self) -> Vec<User> {
        let mut v = Vec::new();
        v.push(User::new(String::from("teste"), String::from("teste nome"), 23, true));
        return v;
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        return Option::Some(User::new(String::from("teste"), String::from("nome"), 23, true));
    }

    async fn update(&self, new_value: &User) -> bool {
        return true;
    }

    async fn delete(&self, id: &str) -> bool {
        return true;
    }

    async fn insert_one(&self, user: &User) -> bool {
        return true;
    }

    async fn insert_many(&self, users: &Vec<User>) -> bool {
        return true;
    }
}