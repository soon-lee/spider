use sqlx::{FromRow, MySqlPool, Pool};

#[derive(FromRow)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://username:password@localhost/dbname")
        .await?;

    // 查询示例
    let users: Vec<User> = sqlx::query_as("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users);

    Ok(())
}
