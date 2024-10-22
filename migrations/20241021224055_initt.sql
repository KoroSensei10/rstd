-- Add migration script here

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id INT PRIMARY KEY,
    username VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS tasks;
CREATE TABLE tasks (
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);