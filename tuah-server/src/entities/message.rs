use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    pub id: Uuid,
    pub room_id: Uuid,
    pub content: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(CLone, Copy, Debug, EnumIter)]
pub enum Relation {
    Room,
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}