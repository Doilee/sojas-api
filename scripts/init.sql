USE sqlx_demo;

CREATE OR REPLACE USER 'user'@'localhost' IDENTIFIED BY 'password';
CREATE OR REPLACE USER 'user'@'172.17.0.1' IDENTIFIED BY 'password'; -- docker
GRANT ALL PRIVILEGES ON sqlx_demo.* TO 'user'@'localhost';
GRANT ALL PRIVILEGES ON sqlx_demo.* TO 'user'@'172.17.0.1'; -- docker
FLUSH PRIVILEGES;

CREATE TABLE IF NOT EXISTS users(
       id INT AUTO_INCREMENT,
       username VARCHAR(15),
       email VARCHAR(100),
       PRIMARY KEY(id)
);

INSERT INTO users(username, email)
VALUES ('test', 'test@mail.de'),
       ('test', 'test@mail.de');
