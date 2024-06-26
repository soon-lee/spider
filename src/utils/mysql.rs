use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub(crate) struct User {
    id: u64,
    username: String,
    password: String,
    balance: u64,
}
impl User {
    pub(crate) fn new(id: u64, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
            balance: 0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let url =
    // let pool = MySqlPool::connect("mysql://username:password@localhost/dbname")
    //     .await?;
    //
    // // 查询示例
    // let users: Vec<User> = sqlx::query_as("SELECT id, name FROM users")
    //     .fetch_all(&pool)
    //     .await?;
    //
    // println!("{:?}", users);

    Ok(())
}
