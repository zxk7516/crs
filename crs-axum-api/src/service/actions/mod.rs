use crate::errors::{OError, OResult};
use crate::service::db;
use crate::service::dto::User;
use crate::service::requests::RegisterRequest;
use crate::RB;

pub async fn register_action(register_form: &RegisterRequest) -> OResult<User> {
    let user_exists = db::find_user_by_name_or_email(
        &mut (&*RB).into(),
        &register_form.email,
        &register_form.username,
    )
    .await?;
    if user_exists.is_some() {
        return Err(OError::CustomedError(400, "User already exists"));
    }

    todo!()
}
