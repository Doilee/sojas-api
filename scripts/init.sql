CREATE OR REPLACE USER 'user'@'localhost' IDENTIFIED BY 'password';
CREATE OR REPLACE USER 'user'@'172.17.0.1' IDENTIFIED BY 'password'; -- docker
GRANT ALL PRIVILEGES ON sqlxdemo.* TO 'user'@'localhost' WITH GRANT OPTION;
GRANT ALL PRIVILEGES ON sqlxdemo.* TO 'user'@'172.17.0.1' WITH GRANT OPTION; -- docker
FLUSH PRIVILEGES;

CREATE TABLE IF NOT EXISTS users(
       id INT AUTO_INCREMENT,
       username VARCHAR(15) NOT NULL,
       email VARCHAR(100) NOT NULL,
       PRIMARY KEY(id)
);

INSERT INTO users(username, email)
VALUES ('marc', 'marc@mail.de'),
       ('thomas', 'thomas@mail.de');
