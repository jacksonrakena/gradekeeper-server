use std::collections::HashMap;
use std::env;
use axum::Json;
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use serde::Serialize;
use crate::models::{StudyBlock, Course, User, CourseComponent, CourseSubcomponent};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUser {
    grade_map: HashMap<String, String>,
    study_blocks: Vec<GetUserStudyBlock>
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserStudyBlock {
    #[serde(flatten)]
    study_block: StudyBlock,
    #[serde(rename = "subjects")]
    courses: Vec<GetUserCourse>
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserCourse {
    #[serde(flatten)]
    course: Course,

    components: Vec<GetUserComponent>
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserComponent {
    #[serde(flatten)]
    component: crate::models::CourseComponent,

    subcomponents: Vec<crate::models::CourseSubcomponent>
}
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_user() -> Result<Json<GetUser>, String> {
    let con = &mut establish_connection();
    use diesel::prelude::*;
    let user = crate::schema::user::dsl::user
        .find("jackson.rakena@gmail.com")
        .select(User::as_select())
        .first(con)
        .unwrap();

    let study_blocks = StudyBlock::belonging_to(&user)
        .select(StudyBlock::as_select())
        .load(con).unwrap();
    let courses = Course::belonging_to(&study_blocks)
        .select(Course::as_select())
        .load(con)
        .unwrap();
    let components = CourseComponent::belonging_to(&courses)
        .select(CourseComponent::as_select())
        .load(con)
        .unwrap();
    let subcomponents = CourseSubcomponent::belonging_to(&components)
        .select(CourseSubcomponent::as_select())
        .load(con)
        .unwrap();

    println!("{}",user.id);
    Ok(Json(GetUser {
        grade_map: serde_json::from_str(user.grade_map.as_str()).unwrap(),
        study_blocks: courses
            .grouped_by(&study_blocks)
            .into_iter()
            .zip(study_blocks)
            .map(|(vcourses, block)| {
                GetUserStudyBlock {
                    study_block: block,
                    courses: components
                        .clone()
                        .grouped_by(&vcourses)
                        .into_iter()
                        .zip(vcourses)
                        .map(|(vcomponents, ncourse)| {
                            GetUserCourse {
                                course: ncourse,
                                components: subcomponents
                                    .clone()
                                    .grouped_by(&vcomponents)
                                    .into_iter()
                                    .zip(vcomponents)
                                    .map(move |(subcomponents,ncomponent)| {
                                        GetUserComponent {
                                            component: ncomponent,
                                            subcomponents
                                        }
                                    })
                                    .collect()
                            }
                        })
                        .collect()
                }
            })
            .collect(),
    }))
}