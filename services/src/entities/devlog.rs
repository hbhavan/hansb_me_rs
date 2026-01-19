use sea_orm::entity::prelude::*;
use chrono::NaiveDate;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="devlog_listing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub title: String,
    pub markdown: String,
    pub created_date: NaiveDate
}


impl ActiveModelBehavior for ActiveModel {}
