use std::env::var;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, MySqlPool, query, query_as, Row};

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) balance: u32,
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
#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Task {
    pub(crate) task_no: u8,
    pub(crate) give_coin: u8,
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
#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Category {
    pub(crate) id: u64,
    pub(crate) title: String,
    pub(crate) sort: u32,
}
impl Category {
    pub(crate) fn new(id: u64, title: String, sort: u32) -> Self {
        Category { id, title, sort }
    }
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Chapter {
    pub(crate) id: String,
    pub(crate) book_id: String,
    pub(crate) title: String,
    pub(crate) pic: String,
    pub(crate) sort: u32,
    pub(crate) price: u32,
    pub(crate) items: Vec<String>,
}
impl Chapter {
    pub(crate) fn new(
        id: String,
        book_id: String,
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
#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
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

pub(crate) async fn db_pool() -> Result<MySqlPool, String> {
    let host = var("DATABASE_HOST").map_err(|err| format!("缺少DATABASE_HOST环境变量:{}", err))?;
    let port = var("DATABASE_PORT").map_err(|err| format!("缺少DATABASE_PORT环境变量:{}", err))?;
    let user = var("DATABASE_USER").map_err(|err| format!("缺少DATABASE_USER环境变量:{}", err))?;
    let password =
        var("DATABASE_PASSWORD").map_err(|err| format!("缺少DATABASE_PASSWORD环境变量:{}", err))?;
    let database =
        var("DATABASE_NAME").map_err(|err| format!("缺少DATABASE_NAME环境变量:{}", err))?;
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );
    MySqlPool::connect(&*url)
        .await
        .map_err(|err| format!("数据库连接失败:{}", err))
}
pub(crate) async fn get_users(pool: MySqlPool) -> Result<Vec<User>, String> {
    sqlx::query_as::<MySql, User>("SELECT * FROM `db_spider`.`tb_comic_xxmh_user`")
        .fetch_all(&pool)
        .await
        .map_err(|err| format!("查询用户列表失败:{}", err))
}
pub(crate) async fn get_user_by_id(pool: MySqlPool, id: u64) -> Result<Vec<User>, String> {
    sqlx::query_as::<MySql, User>("SELECT * FROM `db_spider`.`tb_comic_xxmh_user` WHERE `id` = ?")
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|err| format!("查询用户{}失败:{}", id, err))
}
pub(crate) async fn add_user(pool: MySqlPool, user: User) -> Result<u64, String> {
    sqlx::query("INSERT INTO `db_spider`.`tb_comic_xxmh_user` (`id`, `username`, `password`, `balance`) VALUES (?, ?, ?, ?)")
        .bind(user.id)
        .bind(user.username)
        .bind(user.password)
        .bind(user.balance)
        .execute(&pool)
        .await
        .map(|res| res.last_insert_id())
        .map_err(|err| format!("添加用户{}失败:{}", user.id, err))
}
pub(crate) async fn add_users(pool: MySqlPool, users: Vec<User>) -> Result<u64, String> {
    let values = users
        .iter()
        .map(|user| format!("({}, {}, {})", user.username, user.password, user.balance))
        .collect::<Vec<_>>()
        .join(", ");
    query(&format!("INSERT INTO `db_spider`.`tb_comic_xxmh_user` (`id`, `username`, `password`, `balance`) VALUES {}", values)).execute(&pool).await.map(|res| res.last_insert_id())
        .map_err(|err| format!("批量添加用户失败:{}", err))
}
pub(crate) async fn set_user(pool: MySqlPool, user: User) -> Result<u64, String> {
    sqlx::query("UPDATE `db_spider`.`tb_comic_xxmh_user` SET `username` = ?, `password` = ?, `balance` = ? WHERE `id` = ?")
        .bind(user.username)
        .bind(user.password)
        .bind(user.balance)
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|res| res.rows_affected())
        .map_err(|err| format!("更新用户{}失败:{}", user.id, err))
}
pub(crate) async fn del_user(pool: MySqlPool, id: u64) -> Result<u64, String> {
    sqlx::query("DELETE FROM `db_spider`.`tb_comic_xxmh_user` WHERE `id` = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map(|res| res.rows_affected())
        .map_err(|err| format!("删除用户{}失败:{}", id, err))
}
pub(crate) async fn get_tasks(pool: MySqlPool) -> Result<Vec<Task>, String> {
    query_as::<MySql, Task>("SELECT * FROM `db_spider`.`tb_comic_xxmh_task`")
        .fetch_all(&pool)
        .await
        .map_err(|err| format!("查询任务列表失败:{}", err))
}
pub(crate) async fn get_categories(pool: MySqlPool) -> Result<Vec<Category>, String> {
    query_as::<MySql, Category>("SELECT * FROM `db_spider`.`tb_comic_xxmh_category`")
        .fetch_all(&pool)
        .await
        .map_err(|err| format!("查询分类列表失败:{}", err))
}
pub(crate) async fn get_books(pool: MySqlPool) -> Result<Vec<Book>, String> {
    query("SELECT `id`,`title`,`author`,`note`,`pic`,`big_pic`,`praise_count`,`click_count`,`favorite_count`,`over_type`,`category_id`,`sort`,`tags` FROM `db_spider`.`tb_comic_xxmh_book`")
        .fetch_all(&pool)
        .await.map(|res|
    res.into_iter()
        .map(|row|
        Book::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5), row.get(6), row.get(7), row.get(8), row.get(9), row.get(10), row.get(11), row.get(12),vec![]))
        .collect::<Vec<_>>()).map_err(|err| format!("查询书籍列表失败:{}", err))
}
pub(crate) async fn get_book_by_id(pool: MySqlPool, book_id: String) -> Result<Book, String> {
    let rows = query("SELECT `book`.*,`chapter`.`id` `chapter_id`,`chapter`.`title` `chapter_title`,`chapter`.`pic` `chapter_pic`,`chapter`.`sort` `chapter_sort`,`chapter`.`price`,`chapter`.`items`,`chapter`.`book_id`  FROM `db_spider`.`tb_comic_xxmh_book` `book` LEFT JOIN `db_spider`.`tb_comic_xxmh_chapter` `chapter` ON `book`.`id` = `chapter`.`book_id` WHERE `book`.`id` = ?")
        .bind(&book_id)
        .fetch_all(&pool)
        .await.map_err(|err| format!("查询书籍{}失败:{}", &book_id, err))?;
    let mut chapters = Vec::new();
    let mut book : Option<Book> = None;
    for row in rows{
        if book.is_none(){
            book = Some(Book::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5), row.get(6), row.get(7), row.get(8), row.get(9), row.get(10), row.get(11), row.get(12),vec![]));
        }
        let items_str:String = row.get(19);
        let items: Vec<String> = items_str.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
        chapters.push(Chapter::new(row.get(13), row.get(14), row.get(15), row.get(16), row.get(17), row.get(18), items));
    }
    book.ok_or(format!("书籍{}不存在", book_id)).map_err(|err| format!("查询书籍{}失败:{}", book_id, err))
}
pub(crate) async fn add_book(pool: MySqlPool, book: Book) -> Result<u64, String> {
    sqlx::query("INSERT INTO `db_spider`.`tb_comic_xxmh_book` (`id`, `title`, `author`, `note`, `pic`, `big_pic`, `praise_count`, `click_count`, `favorite_count`, `over_type`, `category_id`, `sort`, `tags`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&book.id)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.note)
        .bind(&book.pic)
        .bind(&book.big_pic)
        .bind(&book.praise_count)
        .bind(&book.click_count)
        .bind(&book.favorite_count)
        .bind(&book.over_type)
        .bind(&book.category_id)
        .bind(&book.sort)
        .bind(&book.tags)
        .execute(&pool)
        .await
        .map(|res| res.last_insert_id())
        .map_err(|err| format!("添加书籍{}失败:{}", book.id, err))
}
pub(crate) async fn add_books(pool: MySqlPool, books: Vec<Book>) -> Result<u64, String> {
    let values = books
        .iter()
        .map(|book| {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                book.id,
                book.title,
                book.author,
                book.note,
                book.pic,
                book.big_pic,
                book.praise_count,
                book.click_count,
                book.favorite_count,
                book.over_type,
                book.category_id,
                book.sort,
                book.tags
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    query(&format!("INSERT INTO `db_spider`.`tb_comic_xxmh_book` (`id`, `title`, `author`, `note`, `pic`, `big_pic`, `praise_count`, `click_count`, `favorite_count`, `over_type`, `category_id`, `sort`, `tags`, `chapters`) VALUES {}", values)).execute(&pool).await.map(|res| res.last_insert_id())
        .map_err(|err| format!("批量添加书籍失败:{}", err))
}
pub(crate) async fn get_chapters_by_book_id(pool: MySqlPool, book_id: u64) -> Result<Vec<Chapter>, String> {
    query("SELECT * FROM `db_spider`.`tb_comic_xxmh_chapter` WHERE `book_id` = ?")
        .bind(&book_id)
        .fetch_all(&pool)
        .await
        .map(|res|res.into_iter().map(|row|{
            let items_str:String = row.get(6);
            let items: Vec<String> = items_str.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
            Chapter::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5), items)}).collect::<Vec<_>>())
        .map_err(|err| format!("查询书籍{}的章节列表失败:{}", book_id, err))
}
pub(crate) async fn get_chapter_by_id(
    pool: MySqlPool,
    book_id: u64,
    chapter_id: u64,
) -> Result<u64, String> {
    sqlx::query(
        "SELECT * FROM `db_spider`.`tb_comic_xxmh_chapter` WHERE `book_id` = ? AND `id` = ?",
    )
    .bind(&book_id)
    .bind(&chapter_id)
    .fetch_all(&pool)
    .await
    .map(|res| res.len() as u64)
    .map_err(|err| format!("查询书籍{}的章节{}失败:{}", book_id, chapter_id, err))
}
pub(crate) async fn add_chapter(pool: MySqlPool, chapter: Chapter) -> Result<u64, String> {
    sqlx::query("INSERT INTO `db_spider`.`tb_comic_xxmh_chapter` (`id`,`title`,`pic`,`sort`,`price`,`items`,`book_id`) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&chapter.id)
        .bind(&chapter.title)
        .bind(&chapter.pic)
        .bind(&chapter.sort)
        .bind(&chapter.price)
        .bind(&chapter.items.join(","))
        .bind(&chapter.book_id)
        .execute(&pool)
        .await
        .map(|res| res.last_insert_id())
        .map_err(|err| format!("添加章节{}失败:{}", chapter.id, err))
}
pub(crate) async fn add_chapters(pool: MySqlPool, chapters: Vec<Chapter>) -> Result<u64, String> {
    let values = chapters
        .iter()
        .map(|chapter|
            format!(
                "({}, {}, {}, {}, {}, {}, {})",
                chapter.id,
                chapter.title,
                chapter.pic,
                chapter.sort,
                chapter.price,
                chapter.items.join(","),
               chapter.book_id
            ))
            .collect::<Vec<_>>()
            .join(", ");
    query(&format!("INSERT INTO `db_spider`.`tb_comic_xxmh_chapter` (`id`, `title`, `pic`, `sort`, `price`, `items`, `book_id`) VALUES {}", values)).execute(&pool).await.map(|res| res.last_insert_id())
        .map_err(|err| format!("批量添加章节失败:{}", err))
}
