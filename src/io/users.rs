use auth::{jwt::create_jwt, AuthResult, AuthUser, LoginUser, UserDb};
use once_cell::sync::Lazy;

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

