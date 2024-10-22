-- Add migration script here
-- Add new columns or modify existing ones in the users table
PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

-- Create a new table with the desired schema
CREATE TABLE new_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(255) NOT NULL
);

-- Copy data from the old table to the new table
INSERT INTO new_users (id, username)
SELECT id, username FROM users;

-- Drop the old table
DROP TABLE users;

-- Rename the new table to the old table name
ALTER TABLE new_users RENAME TO users;

-- Add new columns or modify existing ones in the tasks table
CREATE TABLE new_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INT,
    description TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Copy data from the old table to the new table
INSERT INTO new_tasks (id, user_id, description, completed)
SELECT id, user_id, description, completed FROM tasks;

-- Drop the old table
DROP TABLE tasks;

-- Rename the new table to the old table name
ALTER TABLE new_tasks RENAME TO tasks;

COMMIT;

PRAGMA foreign_keys=on;