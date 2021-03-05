-- Add up migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS `users` (
                                       `id` VARCHAR(36) NOT NULL,
                                       `lastname` VARCHAR(63) NOT NULL,
                                       `firstname` VARCHAR(63) NOT NULL,
                                       `email` VARCHAR(255) NOT NULL,
                                       `password` VARCHAR(128) NOT NULL,
                                       `created_at` DATETIME NOT NULL,
                                       `updated_at` DATETIME NOT NULL,
                                       `deleted_at` DATETIME,
                                       PRIMARY KEY (`id`)
);

ALTER TABLE `users` ADD CONSTRAINT unique_email UNIQUE (`email`);
CREATE INDEX `idx_password` ON `users` (`password`);
CREATE INDEX `idx_deleted_at` ON `users` (`deleted_at`);
