CREATE TABLE IF NOT EXISTS users(
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    soy_balance INT DEFAULT 0 NOT NULL,
    is_admin BOOLEAN DEFAULT 0 NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS regions(
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS venues(
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `url` VARCHAR(255) NOT NULL,
    `show_map` BOOLEAN DEFAULT 0 NOT NULL, -- Maybe not needed, debatable
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS events(
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `region_id` INT UNSIGNED,
    `title` VARCHAR(255) NOT NULL,
    `description` TEXT,
    `reward` INT DEFAULT 150 NOT NULL,
    `source` VARCHAR(255) DEFAULT 'local' NOT NULL, -- or 'external'
    `url` VARCHAR(255),
    `image_url` VARCHAR(255),
    `image_srcset` TEXT,
    PRIMARY KEY(id),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS participants(
    user_id INT UNSIGNED NOT NULL,
    event_id INT UNSIGNED NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS merch(
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    image_url VARCHAR(255),
    image_srcset TEXT,
    title VARCHAR(255),
    `description` TEXT,
    price INT UNSIGNED DEFAULT 0 NOT NULL,
    PRIMARY KEY(id)
);