#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u32,
    pub username: String,
}
