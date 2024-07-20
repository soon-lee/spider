use getset::Getters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize)]
#[getset(get = "pub")]
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
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize)]
#[getset(get = "pub")]
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
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize)]
#[getset(get = "pub")]
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

#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize)]
#[getset(get = "pub")]
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
        book_id: u64,
        title: String,
        pic: String,
        sort: u32,
        price: u32,
        items: Vec<String>,
    ) -> Self {
        Chapter {
            id,
            book_id,
            title,
            pic,
            sort,
            price,
            items,
        }
    }
}
#[derive(Clone, Debug, Deserialize, FromRow, Getters, Serialize)]
#[getset(get = "pub")]
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
}