use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub(crate) struct User {
    pub(crate) id: u64,
    username: String,
    password: String,
    balance: u32,
}
impl User {
    pub(crate) fn new(id: u64, username: String, password: String, balance: u32) -> Self {
        User {
            id,
            username,
            password,
            balance
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Task {
    task_no: u8,
    give_coin: u8,
    task_name: String,
}
impl Task {
    pub(crate) fn new(task_no: u8, give_coin: u8, task_name: String) -> Self {
        Task {
            task_no,
            give_coin,
            task_name,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Category {
    id: u64,
    title: String,
    sort: u32,
}
impl Category {
    pub(crate) fn new(id: u64, title: String, sort: u32) -> Self {
        Category { id, title, sort }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    id: String,
    title: String,
    pic: String,
    sort: u32,
    price: u32,
    items: Vec<String>,
}
impl Chapter {
    pub(crate) fn new(
        id: String,
        title: String,
        pic: String,
        sort: u32,
        price: u32,
        items: Vec<String>,
    ) -> Self {
        Chapter {
            id,
            title,
            pic,
            sort,
            price,
            items,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Book {
    id: String,
    title: String,
    author: String,
    note: String,
    pic: String,
    big_pic: String,
    praise_count: u64,
    click_count: u64,
    favorite_count: u64,
    over_type: String,
    category_id: u64,
    sort: u32,
    tags: String,
    chapters: Vec<Chapter>,
}
impl Book {
    pub(crate) fn new(
        id: String,
        title: String,
        author: String,
        note: String,
        pic: String,
        big_pic: String,
        praise_count: u64,
        click_count: u64,
        favorite_count: u64,
        over_type: String,
        category_id: u64,
        sort: u32,
        tags: String,
        chapters: Vec<Chapter>,
    ) -> Self {
        Book {
            id,
            title,
            author,
            note,
            pic,
            big_pic,
            praise_count,
            click_count,
            favorite_count,
            over_type,
            category_id,
            sort,
            tags,
            chapters,
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
