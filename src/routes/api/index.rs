use diesel::{self, prelude::*};
use rocket_contrib::json::Json;

use crate::db::Connection;
use crate::models::user::User;
use crate::ressources::user_ressource::UserRessource;

#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<UserRessource>>, String> {
    use crate::schema::users::dsl::*;
    let mut to_send: Vec<UserRessource> = Vec::new();

    let users_found = users.load::<User>(&*conn).unwrap();

    for user in users_found {
        to_send.push(UserRessource {
            id: user.id,
            username: user.username,
        })
    }
    Ok(Json(to_send))
}
