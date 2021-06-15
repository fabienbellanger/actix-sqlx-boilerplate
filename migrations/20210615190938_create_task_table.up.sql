-- Add up migration script here
CREATE TABLE IF NOT EXISTS `task` (
                                       `id` VARCHAR(36) NOT NULL,
                                       `name` VARCHAR(63) NOT NULL,
                                       `description` VARCHAR(63),
                                       `created_at` DATETIME NOT NULL,
                                       `updated_at` DATETIME NOT NULL,
                                       `deleted_at` DATETIME,
                                       PRIMARY KEY (`id`)
);

CREATE INDEX `idx_deleted_at` ON `task` (`deleted_at`);
