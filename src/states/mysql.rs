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
    pub(crate) task_no: u8,
    give_coin: u8,
    pub(crate) task_name: String,
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
    pub(crate) id: u64,
    pub(crate) title: String,
    sort: u32,
}
impl Category {
    pub(crate) fn new(id: u64, title: String, sort: u32) -> Self {
        Category { id, title, sort }
    }
}

#[derive(Clone,Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    pub(crate) id: String,
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Book {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) note: String,
    pub(crate) pic: String,
    pub(crate) big_pic: String,
    pub(crate) praise_count: u64,
    pub(crate) click_count: u64,
    pub(crate) favorite_count: u64,
    pub(crate) over_type: String,
    pub(crate) category_id: u64,
    pub(crate) sort: u32,
    pub(crate) tags: String,
    pub(crate) chapters: Vec<Chapter>,
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
