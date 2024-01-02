use anyhow::Result;
use deadpool_postgres::Object;
use std::str::FromStr;
use time::format_description::well_known::Iso8601;
use tokio_postgres::types::Timestamp;
use uuid::Uuid;

use crate::proto::Profile;

impl TryFrom<tokio_postgres::Row> for Profile {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let updated: time::OffsetDateTime = value.try_get("updated")?;
        let updated: String = updated.format(&Iso8601::DEFAULT)?.to_string();
        let deleted: Timestamp<time::OffsetDateTime> = value.try_get("deleted")?;
        let deleted: String = match deleted {
            Timestamp::PosInfinity => "infinity".to_string(),
            Timestamp::NegInfinity => "-infinity".to_string(),
            Timestamp::Value(date) => date.format(&Iso8601::DEFAULT)?.to_string(),
        };

        let user_id: Uuid = value.try_get("user_id")?;
        let user_id: String = user_id.to_string();
        let name: String = value.try_get("name")?;
        let about: String = value.try_get("about")?;
        let avatar_id: String = value.try_get("avatar_id")?;
        let avatar_url: String = value.try_get("avatar_url")?;
        let cover_id: String = value.try_get("cover_id")?;
        let cover_url: String = value.try_get("cover_url")?;
        let resume_id: String = value.try_get("resume_id")?;

        Ok(Profile {
            id: id.to_string(),
            created,
            updated,
            deleted,
            user_id,
            name,
            about,
            avatar_id,
            avatar_url,
            cover_id,
            cover_url,
            resume_id,
        })
    }
}

pub async fn select_profile_by_user_id(conn: &Object, user_id: &str) -> Result<Option<Profile>> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let profile = conn
        .query_opt(
            "select * from profiles where user_id = $1 and deleted = 'infinity'",
            &[&user_id],
        )
        .await?;

    let profile = match profile {
        Some(profile) => {
            let profile: Profile = Profile::try_from(profile)?;
            Some(profile)
        }
        None => None,
    };
    Ok(profile)
}

pub async fn insert_profile(conn: &Object, user_id: &str, profile: &Profile) -> Result<Profile> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let profile: tokio_postgres::Row = conn.query_one(
        "insert into profiles (id, user_id, name, about, avatar_id, avatar_url, cover_id, cover_url, resume_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
        &[&Uuid::now_v7(), &user_id, &profile.name, &profile.about, &profile.avatar_id, &profile.avatar_url, &profile.cover_id, &profile.cover_url, &profile.resume_id]
    ).await?;
    let profile: Profile = Profile::try_from(profile)?;
    Ok(profile)
}

pub async fn update_profile(conn: &Object, user_id: &str, profile: &Profile) -> Result<Profile> {
    let id = Uuid::from_str(&profile.id)?;
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let profile: tokio_postgres::Row = conn.query_one(
        "update profiles set name = $1, about = $2, avatar_id = $3, avatar_url = $4, cover_id = $5, cover_url = $6, resume_id = $7 where id = $8 and user_id = $9 returning *",
        &[&profile.name, &profile.about, &profile.avatar_id, &profile.avatar_url, &profile.cover_id, &profile.cover_url, &profile.resume_id, &id, &user_id]
    ).await?;
    let profile: Profile = Profile::try_from(profile)?;
    Ok(profile)
}
