//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bot_user_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "accessToken")]
    pub access_token: Option<String>,
    #[sea_orm(column_name = "refreshToken")]
    pub refresh_token: Option<String>,
    pub balance: i32,
    #[sea_orm(column_name = "bookingDonated")]
    pub booking_donated: i32,
    #[sea_orm(column_name = "bookingSpent")]
    pub booking_spent: i32,
    #[sea_orm(column_name = "bookingTime")]
    pub booking_time: i32,
    #[sea_orm(column_name = "prayPoints")]
    pub pray_points: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub inventory: Option<String>,
    #[sea_orm(column_name = "profileBio", column_type = "Text", nullable)]
    pub profile_bio: Option<String>,
    #[sea_orm(column_name = "profileImage")]
    pub profile_image: Option<String>,
    #[sea_orm(column_name = "botBotID")]
    pub bot_bot_id: Option<String>,
    #[sea_orm(column_name = "userDiscordID")]
    pub user_discord_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::booking_activities::Entity")]
    BookingActivities,
    #[sea_orm(has_one = "super::booking_player::Entity")]
    BookingPlayer,
    #[sea_orm(
        belongs_to = "super::bots_info::Entity",
        from = "Column::BotBotId",
        to = "super::bots_info::Column::BotId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BotsInfo,
    #[sea_orm(
        belongs_to = "super::user_info::Entity",
        from = "Column::UserDiscordId",
        to = "super::user_info::Column::DiscordId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    UserInfo,
}

impl Related<super::booking_activities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingActivities.def()
    }
}

impl Related<super::booking_player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingPlayer.def()
    }
}

impl Related<super::bots_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotsInfo.def()
    }
}

impl Related<super::user_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
