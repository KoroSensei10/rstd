use crate::users::*;
use sqlx::sqlite::{SqlitePool, SqliteQueryResult};

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
    pub user_id: Option<i64>,
}

pub async fn list_tasks(
    pool: &SqlitePool,
    user: Option<i64>,
    user_name: Option<String>,
) -> Vec<Task> {
    let tasks = match (user, user_name) {
        (Some(user_id), _) => sqlx::query_as!(
            Task,
            "SELECT id, description, completed, user_id FROM tasks WHERE user_id = ?",
            user_id
        )
        .fetch_all(pool)
        .await
        .expect("Failed to fetch tasks"),
        (None, Some(name)) => {
            let user_id = find_user_id(pool, &name).await;
            sqlx::query_as!(
                Task,
                "SELECT id, description, completed, user_id FROM tasks WHERE user_id = ?",
                user_id
            )
            .fetch_all(pool)
            .await
            .expect("Failed to fetch tasks")
        }
        (None, None) => sqlx::query_as!(
            Task,
            "SELECT id, description, completed, user_id FROM tasks"
        )
        .fetch_all(pool)
        .await
        .expect("Failed to fetch tasks"),
    };

    for task in tasks.iter() {
        let status = if task.completed {
            "completed"
        } else {
            "pending"
        };
        let username = if let Some(user_id) = task.user_id {
            let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id)
                .fetch_one(pool)
                .await
                .expect("Failed to fetch username");
            format!(" (user: {})", user.username)
        } else {
            "".to_string()
        };
        println!(
            "[{}] {}{} - {}",
            task.id, task.description, username, status
        );
    }
    tasks
}

pub async fn add_task(
    pool: &SqlitePool,
    description: &str,
    user_id: Option<i64>,
    user_name: Option<String>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let user_id = match (user_id, user_name) {
        (Some(id), _) => Some(id),
        (None, Some(name)) => find_user_id(pool, &name).await,
        (None, None) => None,
    };

    if let Some(user_id) = user_id {
        sqlx::query!(
            "INSERT INTO tasks (description, user_id) VALUES (?, ?)",
            description,
            user_id
        )
        .execute(pool)
        .await
    } else {
        sqlx::query!("INSERT INTO tasks (description) VALUES (?)", description)
            .execute(pool)
            .await
    }
}

pub async fn complete_task(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!("UPDATE tasks SET completed = true WHERE id = ?", id)
        .execute(pool)
        .await
}

pub async fn delete_task(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM tasks WHERE id = ?", id)
        .execute(pool)
        .await
}
