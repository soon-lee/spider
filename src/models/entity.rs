use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;

#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub(crate) struct User {
    id: u64,
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
            balance,
        }
    }
}
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
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
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
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

#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub(crate) struct Chapter {
    id: u64,
    book_id: u64,
    title: String,
    pic: String,
    sort: u32,
    price: u32,
    items: Vec<String>,
}
impl Chapter {
    pub(crate) fn new(
        id: u64,
        title: String,
        pic: String,
        sort: u32,
        price: u32,
        items: Vec<String>,
        book_id: u64,
    ) -> Self {
        Chapter {
            id,
            title,
            pic,
            sort,
            price,
            items,
            book_id,
        }
    }
    pub(crate) fn from_row(row: &MySqlRow, offset: usize) -> Self {
        let items_str: String = row.get(offset + 5);
        let items: Vec<String> = items_str
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Chapter::new(
            row.get::<u64, _>(offset),
            row.get::<String, _>(offset + 1),
            row.get::<String, _>(offset + 2),
            row.get::<u32, _>(offset + 3),
            row.get::<u32, _>(offset + 4),
            items,
            row.get::<u64, _>(offset + 6),
        )
    }
}
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub(crate) struct Book {
    id: u64,
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
        id: u64,
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
    pub(crate) fn from_row(row: &MySqlRow, offset: usize) -> Self {
        Book::new(
            row.get::<u64, _>(offset),
            row.get::<String, _>(offset + 1),
            row.get::<String, _>(offset + 2),
            row.get::<String, _>(offset + 3),
            row.get::<String, _>(offset + 4),
            row.get::<String, _>(offset + 5),
            row.get::<u64, _>(offset + 6),
            row.get::<u64, _>(offset + 7),
            row.get::<u64, _>(offset + 8),
            row.get::<String, _>(offset + 9),
            row.get::<u64, _>(offset + 10),
            row.get::<u32, _>(offset + 11),
            row.get::<String, _>(offset + 12),
            vec![],
        )
    }
}
