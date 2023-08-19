// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        #[max_length = 25]
        id -> Varchar,
        #[max_length = 25]
        block_id -> Varchar,
        #[max_length = 191]
        long_name -> Varchar,
        #[max_length = 10]
        course_code_name -> Varchar,
        #[max_length = 10]
        course_code_number -> Varchar,
        #[max_length = 7]
        color -> Varchar,
    }
}

diesel::table! {
    course_component (id) {
        #[max_length = 25]
        id -> Varchar,
        #[max_length = 25]
        course_id -> Varchar,
        #[max_length = 191]
        name -> Varchar,
        #[max_length = 191]
        name_of_subcomponent_singular -> Varchar,
        subject_weighting -> Numeric,
        number_of_subcomponents_to_drop_lowest -> Int4,
    }
}

diesel::table! {
    course_subcomponent (id) {
        #[max_length = 25]
        id -> Varchar,
        #[max_length = 25]
        component_id -> Varchar,
        number_in_sequence -> Int4,
        #[max_length = 191]
        override_name -> Nullable<Varchar>,
        is_completed -> Bool,
        grade_value_percentage -> Numeric,
    }
}

diesel::table! {
    gk_user (id) {
        #[max_length = 25]
        id -> Varchar,
        grade_map -> Json,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    study_block (id) {
        #[max_length = 25]
        id -> Varchar,
        #[max_length = 25]
        user_id -> Varchar,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        #[max_length = 191]
        name -> Varchar,
    }
}

diesel::table! {
    timetable_entry (id) {
        #[max_length = 25]
        id -> Varchar,
        #[max_length = 25]
        course_id -> Varchar,
        #[max_length = 191]
        user_type -> Varchar,
        day_index -> Array<Nullable<Int2>>,
        start_time -> Time,
        end_time -> Time,
        #[max_length = 50]
        room -> Nullable<Varchar>,
        #[max_length = 50]
        lecturer -> Nullable<Varchar>,
    }
}

diesel::joinable!(course -> study_block (block_id));
diesel::joinable!(course_component -> course (course_id));
diesel::joinable!(course_subcomponent -> course_component (component_id));
diesel::joinable!(study_block -> gk_user (user_id));
diesel::joinable!(timetable_entry -> course (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    course,
    course_component,
    course_subcomponent,
    gk_user,
    study_block,
    timetable_entry,
);
