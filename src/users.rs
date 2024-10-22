use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

pub async fn find_user_id(pool: &SqlitePool, user_name: &str) -> Option<i64> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", user_name)
        .fetch_one(pool)
        .await
        .ok();
    user.map(|u| u.id)
}

pub async fn list_users(pool: &SqlitePool) -> Vec<User> {
    let users = sqlx::query_as!(User, "SELECT id, username FROM users")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch users");

    for user in users.iter() {
        println!("[{}] {}", user.id, user.username);
    }
    users
}

pub async fn add_user(pool: &SqlitePool, username: &str) {
    sqlx::query!("INSERT INTO users (username) VALUES (?)", username)
        .execute(pool)
        .await
        .expect("Failed to insert user");
}

pub async fn delete_user(pool: &SqlitePool, id: i64) {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await
        .expect("Failed to delete user");
}
