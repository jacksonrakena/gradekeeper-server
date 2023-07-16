ALTER TABLE `StudyBlock` DROP FOREIGN KEY fk_user_owns_studyblock;
ALTER TABLE `Subject` DROP FOREIGN KEY fk_studyblock_owns_subject;
ALTER TABLE `SubjectComponent` DROP FOREIGN KEY fk_subject_owns_subjectcomponent;
ALTER TABLE `SubjectSubcomponent` DROP FOREIGN KEY fk_subjectcomponent_owns_subcomponent;