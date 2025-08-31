use sqlx::{sqlite::SqliteRow, Row};

use crate::core::data_model::{implementations::Invite, traits::IInvite};

pub fn row_to_invite(row : &SqliteRow) -> Invite {
    let id : &str = row.get("id");
    let invite_code : &str = row.get("invite_code");
    let one_use_str : &str = row.get("one_use");
    let one_use = one_use_str.to_lowercase() == "true";
    return Invite::new(id, invite_code, one_use);
}
