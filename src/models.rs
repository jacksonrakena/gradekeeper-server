use diesel::prelude::*;

use serde::Serialize;
use time::OffsetDateTime;

#[derive(Queryable, Serialize, Selectable, Insertable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::gk_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub grade_map: serde_json::Value,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
#[derive(
    Queryable, Selectable, Serialize, Associations, Identifiable, Insertable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::study_block)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct StudyBlock {
    #[serde(with = "time::serde::rfc3339")]
    pub end_date: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub start_date: OffsetDateTime,
    pub id: String,
    pub name: String,
    pub user_id: String,
}
#[derive(
    Queryable, Selectable, Serialize, Associations, Insertable, Identifiable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::course)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(StudyBlock, foreign_key=block_id))]
pub struct Course {
    pub id: String,
    pub long_name: String,
    pub course_code_name: String,
    pub course_code_number: String,
    #[serde(rename = "studyBlockId")]
    pub block_id: String,
    pub color: String,
}
#[derive(
    Queryable, Selectable, Serialize, Associations, Insertable, Identifiable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::course_component)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Course))]
pub struct CourseComponent {
    pub id: String,
    pub name: String,
    pub name_of_subcomponent_singular: String,
    #[serde(rename = "numberOfSubComponentsToDrop_Lowest")]
    pub number_of_subcomponents_to_drop_lowest: i32,
    #[serde(rename = "subjectId")]
    pub course_id: String,
    pub subject_weighting: bigdecimal::BigDecimal,
}

#[derive(
    Queryable, Selectable, Serialize, Associations, Insertable, Identifiable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::course_subcomponent)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(CourseComponent, foreign_key=component_id))]
pub struct CourseSubcomponent {
    pub id: String,
    pub component_id: String,
    pub grade_value_percentage: bigdecimal::BigDecimal,
    pub is_completed: bool,
    pub number_in_sequence: i32,
    pub override_name: Option<String>,
}
