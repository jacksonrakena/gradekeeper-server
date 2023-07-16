CREATE TABLE `User` (
                        `id` varchar(191) NOT NULL,
                        `gradeMap` json NOT NULL,
                        PRIMARY KEY (`id`)
) CHARSET utf8mb4,
  COLLATE utf8mb4_unicode_ci;

CREATE TABLE `SubjectSubcomponent` (
                                       `id` varchar(191) NOT NULL,
                                       `componentId` varchar(191) NOT NULL,
                                       `numberInSequence` int NOT NULL,
                                       `overrideName` varchar(191),
                                       `isCompleted` tinyint(1) NOT NULL,
                                       `gradeValuePercentage` double NOT NULL,
                                       PRIMARY KEY (`id`),
                                       KEY `SubjectSubcomponent_componentId_idx` (`componentId`)
) CHARSET utf8mb4,
  COLLATE utf8mb4_unicode_ci;

CREATE TABLE `SubjectComponent` (
    `id` varchar(191) NOT NULL,
    `subjectId` varchar(191) NOT NULL,
    `name` varchar(191) NOT NULL,
    `nameOfSubcomponentSingular` varchar(191) NOT NULL,
    `subjectWeighting` double NOT NULL,
    `numberOfSubComponentsToDrop_Lowest` int NOT NULL,
    PRIMARY KEY (`id`),
    KEY `SubjectComponent_subjectId_idx` (`subjectId`)
)
  CHARSET utf8mb4,
  COLLATE utf8mb4_unicode_ci;

CREATE TABLE `Subject`
(
    `id`               varchar(191) NOT NULL,
    `studyBlockId`     varchar(191) NOT NULL,
    `longName`         varchar(191) NOT NULL,
    `courseCodeName`   varchar(191) NOT NULL,
    `courseCodeNumber` varchar(191) NOT NULL,
    `color`            varchar(191) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`),
    KEY `Subject_studyBlockId_idx` (`studyBlockId`)
) CHARSET utf8mb4,
  COLLATE utf8mb4_unicode_ci;

CREATE TABLE `StudyBlock` (
                              `id` varchar(191) NOT NULL,
                              `userId` varchar(191) NOT NULL,
                              `startDate` datetime(3) NOT NULL,
                              `endDate` datetime(3) NOT NULL,
                              `name` varchar(191) NOT NULL DEFAULT '',
                              PRIMARY KEY (`id`),
                              KEY `StudyBlock_userId_idx` (`userId`)
) ENGINE InnoDB,
  CHARSET utf8mb4,
  COLLATE utf8mb4_unicode_ci;