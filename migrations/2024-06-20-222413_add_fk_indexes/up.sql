CREATE INDEX idx_fk_course_component ON course_subcomponent (component_id);
CREATE INDEX idx_fk_course ON course_component (course_id);
CREATE INDEX idx_fk_study_block ON course (block_id);
CREATE INDEX idx_fk_user ON study_block (user_id);