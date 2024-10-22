-- Add migration script here

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS tasks;
CREATE TABLE tasks (
    id PRIMARY KEY NOT NULL,
    user_id INT NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);