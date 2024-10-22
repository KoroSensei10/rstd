-- Add migration script here

CREATE TABLE users (
    id PRIMARY KEY,
    username VARCHAR(255) NOT NULL
);

CREATE TABLE tasks (
    id PRIMARY KEY,
    user_id INT NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);