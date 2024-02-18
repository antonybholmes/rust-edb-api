use auth::{jwt::{create_jwt, JWTResp}, AuthResult, AuthUser, LoginUser, UserDb};
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use super::{unwrap_bad_req,JsonResult};



static USER_DB: Lazy<UserDb> = Lazy::new(|| 
    UserDb::new("data/users.db").unwrap()
    );

pub fn register(user: &LoginUser) -> AuthResult<String> {
    //let userdb: UserDb = create_userdb()?;

    let auth_user: AuthUser = USER_DB.create_user(user)?;

    let jwt: String = create_jwt(&auth_user)?;

    Ok(jwt)
}

pub fn login(user: &LoginUser) -> AuthResult<String> {
    //let userdb: UserDb = create_userdb()?;

    let auth_user: AuthUser = USER_DB.find_user_by_email(user)?;

    let jwt: String = create_jwt(&auth_user)?;

    Ok(jwt)
}

#[post("/register", format = "application/json", data = "<user>")]
pub fn register_route(user: Json<LoginUser>) -> JsonResult<JWTResp> {
    let jwt: String = unwrap_bad_req(register(&user))?;

    Ok(Json(JWTResp { jwt }))
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_route(user: Json<LoginUser>) -> JsonResult<JWTResp> {
    let jwt: String = unwrap_bad_req(login(&user))?;

    Ok(Json(JWTResp { jwt }))
}

