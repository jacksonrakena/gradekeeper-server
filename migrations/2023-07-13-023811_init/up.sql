CREATE TABLE gk_user
(
    id         varchar(191) NOT NULL,
    grade_map  json         NOT NULL,
    created_at timestamptz  NOT NULL DEFAULT now(),
    PRIMARY KEY (id)
);

CREATE TABLE study_block
(
    id         varchar(25) NOT NULL,
    user_id    varchar(191) NOT NULL,
    start_date timestamptz  NOT NULL,
    end_date   timestamptz  NOT NULL,
    name       varchar(191) NOT NULL DEFAULT '',
    PRIMARY KEY (id),
    CONSTRAINT fk_user_owns_study_block FOREIGN KEY (user_id) REFERENCES gk_user (id)
);

CREATE TABLE course
(
    id                 varchar(25) NOT NULL,
    block_id           varchar(25) NOT NULL,
    long_name          varchar(191) NOT NULL,
    course_code_name   varchar(10) NOT NULL,
    course_code_number varchar(10) NOT NULL,
    color              varchar(7)   NOT NULL DEFAULT '',
    PRIMARY KEY (id),
    CONSTRAINT fk_block_owns_course FOREIGN KEY (block_id) REFERENCES study_block (id) ON DELETE CASCADE
);

CREATE TABLE course_component
(
    id                                     varchar(25) NOT NULL,
    course_id                              varchar(25) NOT NULL,
    name                                   varchar(191) NOT NULL,
    name_of_subcomponent_singular          varchar(191) NOT NULL,
    subject_weighting                      numeric(5,4)       NOT NULL,
    number_of_subcomponents_to_drop_lowest int          NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_course_owns_component FOREIGN KEY (course_id) REFERENCES course (id) ON DELETE CASCADE
);

CREATE TABLE course_subcomponent
(
    id                     varchar(25) NOT NULL,
    component_id           varchar(25) NOT NULL,
    number_in_sequence     int          NOT NULL,
    override_name          varchar(191),
    is_completed           bool   NOT NULL,
    grade_value_percentage numeric(5,4)       NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_component_owns_subcomponent FOREIGN KEY (component_id) REFERENCES course_component (id) ON DELETE CASCADE
);
