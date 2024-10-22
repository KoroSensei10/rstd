use clap::{Parser, Subcommand};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
mod tasks;
use tasks::{add_task, complete_task, delete_task, list_tasks};
mod users;
use users::{add_user, delete_user, list_users};

#[derive(Parser)]
#[command(name = "RSTD")]
#[command(about = "A simple CLI to manage your tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new task")]
    Add {
        description: String,
        #[arg(short('i'), long)]
        user_id: Option<i64>,
        #[arg(short, long)]
        user_name: Option<String>,
    },
    #[command(about = "List all tasks")]
    List {
        #[arg(short('i'), long)]
        user_id: Option<i64>,
        #[arg(short, long)]
        user_name: Option<String>,
    },
    #[command(about = "Mark a task as completed")]
    Complete { id: i64 },
    #[command(about = "Delete a task")]
    Delete { id: i64 },
    #[command(about = "Add a new user")]
    AddUser { username: String },
    #[command(about = "List all users")]
    ListUsers,
    #[command(about = "Delete a user")]
    DeleteUser { id: i64 },
}

async fn init_db() -> SqlitePool {
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "sqlite::memory:".to_string(),
    };
    match SqlitePool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let cli = Cli::parse();
    let pool = init_db().await;

    match cli.command {
        Commands::List { user_id, user_name } => {
            list_tasks(&pool, user_id, user_name).await;
        }
        Commands::Add {
            description,
            user_id,
            user_name,
        } => {
            add_task(&pool, &description, user_id, user_name).await?;
            println!("Task added successfully");
            list_tasks(&pool, None, None).await;
        }
        Commands::Complete { id } => {
            complete_task(&pool, id).await?;
            println!("Task completed successfully");
            list_tasks(&pool, None, None).await;
        }
        Commands::Delete { id } => {
            delete_task(&pool, id).await?;
            println!("Task deleted successfully");
            list_tasks(&pool, None, None).await;
        }
        Commands::ListUsers => {
            list_users(&pool).await;
        }
        Commands::AddUser { username } => {
            add_user(&pool, &username).await;
            println!("User added successfully");
            list_users(&pool).await;
        }
        Commands::DeleteUser { id } => {
            delete_user(&pool, id).await;
            println!("User deleted successfully");
            list_users(&pool).await;
        }
    }
    Ok(())
}
