ALTER TABLE `StudyBlock` ADD CONSTRAINT fk_user_owns_studyblock FOREIGN KEY (`userId`) REFERENCES `user`(id);
ALTER TABLE `Subject` ADD CONSTRAINT fk_studyblock_owns_subject FOREIGN KEY (`studyBlockId`) REFERENCES `StudyBlock`(id);
ALTER TABLE `SubjectComponent` ADD CONSTRAINT fk_subject_owns_subjectcomponent FOREIGN KEY (`subjectId`) REFERENCES `Subject`(id);
ALTER TABLE `SubjectSubcomponent` ADD CONSTRAINT fk_subjectcomponent_owns_subcomponent FOREIGN KEY (`componentId`) REFERENCES `SubjectComponent`(id);