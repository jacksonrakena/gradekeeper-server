ALTER TABLE `study_block` DROP CONSTRAINT fk_user_owns_studyblock;
ALTER TABLE `course` DROP CONSTRAINT fk_studyblock_owns_subject;
ALTER TABLE `course_component` DROP CONSTRAINT fk_subject_owns_subjectcomponent;
ALTER TABLE `course_subcomponent` DROP CONSTRAINT fk_subjectcomponent_owns_subcomponent;

ALTER TABLE `study_block` ADD CONSTRAINT fk_user_owns_studyblock FOREIGN KEY (`userId`) REFERENCES `user`(id);
ALTER TABLE `course` ADD CONSTRAINT fk_studyblock_owns_subject FOREIGN KEY (`studyBlockId`) REFERENCES `study_block`(id);
ALTER TABLE `course_component` ADD CONSTRAINT fk_subject_owns_subjectcomponent FOREIGN KEY (`subjectId`) REFERENCES `course`(id);
ALTER TABLE `course_subcomponent` ADD CONSTRAINT fk_subjectcomponent_owns_subcomponent FOREIGN KEY (`componentId`) REFERENCES `course_component`(id);