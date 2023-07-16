use diesel::prelude::*;
use rust_decimal::Decimal;
use time::OffsetDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize, Selectable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: String,
    #[diesel(column_name="gradeMap")]
    pub grade_map: String
}
#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::study_block)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(User, foreign_key=userId))]
pub struct StudyBlock {
    #[serde(with = "time::serde::rfc3339")]
    #[diesel(column_name="endDate")]
    pub end_date: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[diesel(column_name="startDate")]
    pub start_date: OffsetDateTime,
    pub id: String,
    pub name: String,
    #[diesel(column_name="userId")]
    pub user_id: String,
}
#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::course)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(StudyBlock, foreign_key=studyBlockId))]
pub struct Course {
    pub id: String,
    #[diesel(column_name="longName")]
    pub long_name: String,
    #[diesel(column_name="courseCodeName")]
    pub course_code_name: String,
    #[diesel(column_name="courseCodeNumber")]
    pub course_code_number: String,
    #[diesel(column_name="studyBlockId")]
    pub study_block_id: String,
    pub color: String,
}
#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::course_component)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Course, foreign_key=subjectId))]
pub struct CourseComponent {
    pub id: String,
    pub name: String,
    #[diesel(column_name="nameOfSubcomponentSingular")]
    pub name_of_subcomponent_singular: String,
    #[diesel(column_name="numberOfSubComponentsToDrop_Lowest")]
    pub number_of_subcomponents_to_drop_lowest: i32,
    #[diesel(column_name="subjectId")]
    pub subject_id: String,
    #[diesel(column_name="subjectWeighting")]
    pub subject_weighting: f64
}

#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::course_subcomponent)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(CourseComponent, foreign_key=componentId))]
pub struct CourseSubcomponent {
    pub id: String,
    #[diesel(column_name="componentId")]
    pub component_id: String,
    #[diesel(column_name="gradeValuePercentage")]
    pub grade_value_percentage: f64,
    #[diesel(column_name="isCompleted")]
    pub is_completed: bool,
    #[diesel(column_name="numberInSequence")]
    pub number_in_sequence: i32,
    #[diesel(column_name="overrideName")]
    pub override_name: Option<String>
}