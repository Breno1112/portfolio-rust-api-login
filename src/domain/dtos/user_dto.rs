use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub age: u32,
}