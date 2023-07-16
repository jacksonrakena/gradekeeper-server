use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use diesel::prelude::*;
use serde::Serialize;
use crate::models::{StudyBlock, Course, User, CourseComponent, CourseSubcomponent};
use crate::ServerState;

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
pub async fn get_user(Extension(state): Extension<Arc<ServerState>>) -> Result<Json<GetUser>, String> {
    let con = &mut state.db_pool.get().unwrap();
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
        study_blocks: study_blocks.into_iter().map(|s|{
            GetUserStudyBlock {
                study_block: s.clone(),
                courses: courses
                    .clone()
                    .into_iter()
                    .filter(|x|x.study_block_id==s.id)
                    .map(|c|{
                        GetUserCourse{
                            course: c.clone(),
                            components: components
                                .clone()
                                .into_iter()
                                .filter(|component|component.subject_id==c.id)
                                .map(|component|GetUserComponent {
                                    component: component.clone(),
                                    subcomponents: subcomponents
                                        .clone()
                                        .into_iter()
                                        .filter(|subc|subc.component_id==component.id)
                                        .collect()
                                })
                                .collect()
                        }
                    })
                    .collect()
            }
        }).collect()
    }))
}