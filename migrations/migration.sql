CREATE TABLE IF NOT EXISTS users(
    id INT UNSIGNED,
    soy_balance INT DEFAULT 0,
    is_admin BOOLEAN DEFAULT 0,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS events(
    id INT UNSIGNED,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS participants(
    user_id INT UNSIGNED,
    event_id INT UNSIGNED,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS merch(
    id INT UNSIGNED,
    image_url VARCHAR(255) NULL,
    image_srcset TEXT NULL,
    title VARCHAR(255),
    `description` TEXT NULL,
    price INT UNSIGNED DEFAULT 0,
    PRIMARY KEY(id)
);