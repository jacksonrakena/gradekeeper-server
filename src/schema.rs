// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        studyBlockId -> Varchar,
        #[max_length = 191]
        longName -> Varchar,
        #[max_length = 191]
        courseCodeName -> Varchar,
        #[max_length = 191]
        courseCodeNumber -> Varchar,
        #[max_length = 191]
        color -> Varchar,
    }
}

diesel::table! {
    course_component (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        subjectId -> Varchar,
        #[max_length = 191]
        name -> Varchar,
        #[max_length = 191]
        nameOfSubcomponentSingular -> Varchar,
        subjectWeighting -> Double,
        numberOfSubComponentsToDrop_Lowest -> Integer,
    }
}

diesel::table! {
    course_subcomponent (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        componentId -> Varchar,
        numberInSequence -> Integer,
        #[max_length = 191]
        overrideName -> Nullable<Varchar>,
        isCompleted -> Bool,
        gradeValuePercentage -> Double,
    }
}

diesel::table! {
    study_block (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        userId -> Varchar,
        startDate -> Datetime,
        endDate -> Datetime,
        #[max_length = 191]
        name -> Varchar,
    }
}

diesel::table! {
    user (id) {
        #[max_length = 191]
        id -> Varchar,
        gradeMap -> Longtext,
    }
}

diesel::joinable!(course -> study_block (studyBlockId));
diesel::joinable!(course_component -> course (subjectId));
diesel::joinable!(course_subcomponent -> course_component (componentId));
diesel::joinable!(study_block -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(
    course,
    course_component,
    course_subcomponent,
    study_block,
    user,
);
