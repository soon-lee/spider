use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::models::entity::{Book, Category, Chapter, Task, User};
use crate::models::transfer::{BookInfo, CategoryInfo, ItemInfo, SnapshotInfo, TaskInfo, UserInfo};
use crate::utils::crypt::{aes_decrypt, aes_encrypt, auth_path};
use crate::utils::datetime::timestamp_str;
use crate::utils::global::NetClient;

pub(crate) struct UserNetAccessor {
    client: Arc<NetClient>,
}
impl UserNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        UserNetAccessor { client }
    }
    pub(crate) async fn register_user(&self) -> Result<User, String> {
        let dev_type = self
            .client
            .options
            .get("dev_type")
            .ok_or("缺少origin参数")?;

        let data = json!({
            "devType": dev_type,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self.client.post_client("/api/user/regUser", &data).await?;

        let user_info: UserInfo = serde_json::from_str(&response_text)
            .map_err(|err| format!("UserInfo序列化失败: {}", err))?;

        let user = User::new(
            user_info
                .id()
                .parse::<u64>()
                .map_err(|err| format!("解析失败，{}", err))?,
            user_info.account().clone(),
            user_info.pwd().clone(),
            0,
        );
        Ok(user)
    }
    pub(crate) async fn user_info(&self, user_id: &u64) -> Result<User, String> {
        let data = json!({
            "userId": &user_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self
            .client
            .post_client("/api/user/getUserInfo", &data)
            .await?;

        let user_info: UserInfo = serde_json::from_str(&response_text)
            .map_err(|e| format!("从Json解析UserInfo失败: {}", e))?;

        let user = User::new(
            user_info
                .id()
                .parse::<u64>()
                .map_err(|err| format!("解析失败，{}", err))?,
            user_info.account().clone(),
            user_info.pwd().clone(),
            user_info.balance().clone(),
        );
        Ok(user)
    }
}

pub(crate) struct TaskNetAccessor {
    pub(crate) client: Arc<NetClient>,
}
impl TaskNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        TaskNetAccessor { client }
    }
    pub(crate) async fn task_list(&self, user_id: &u64) -> Result<Vec<Task>, String> {
        let data = json!({
            "userId": &user_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self
            .client
            .post_client("/api/user/getTaskList", &data)
            .await?;

        let task_info_list: Vec<TaskInfo> = serde_json::from_str(&response_text)
            .map_err(|e| format!("从Json解析TaskInfo列表失败: {}", e))?;

        let task_list = task_info_list
            .into_iter()
            .map(|task_info| {
                Task::new(
                    task_info.task_no().clone(),
                    task_info.give_coin().clone(),
                    task_info.task_name().clone(),
                )
            })
            .collect::<Vec<_>>();

        Ok(task_list)
    }
}

pub(crate) struct CategoryNetAccessor {
    pub(crate) client: Arc<NetClient>,
}
impl CategoryNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        CategoryNetAccessor { client }
    }
    pub(crate) async fn category_list(&self) -> Result<Vec<Category>, String> {
        let data = json!({
            "c": "yml",
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self
            .client
            .post_client("/api/h5/getCategory", &data)
            .await?;

        let category_info_list: Vec<CategoryInfo> = serde_json::from_str(&response_text)
            .map_err(|err| format!("从Json解析CategoryInfo列表失败: {}", err))?;

        let category_list = category_info_list
            .into_iter()
            .map(|category_info| {
                Category::new(
                    category_info
                        .id()
                        .parse::<u64>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                    category_info.title().clone(),
                    category_info
                        .sort()
                        .parse::<u32>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        Ok(category_list)
    }
}

pub(crate) struct BookNetAccessor {
    pub(crate) client: Arc<NetClient>,
}
impl BookNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        BookNetAccessor { client }
    }

    pub(crate) async fn snapshot_list(
        &self,
        category_id: &u64,
        page: &u32,
        limit: &u32,
    ) -> Result<Vec<Book>, String> {
        let data = json!({
            "page": page,
            "limit": limit,
            "category_id": category_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self
            .client
            .post_client("/api/h5/getComicByCategoryId", &data)
            .await?;

        let snapshot_info: SnapshotInfo = serde_json::from_str(&response_text)
            .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
        let book_list = snapshot_info
            .records()
            .iter()
            .map(|snapshot_info| {
                Book::new(
                    snapshot_info
                        .id()
                        .parse::<u64>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                    snapshot_info.title().clone(),
                    snapshot_info.author().clone(),
                    snapshot_info.note().clone(),
                    snapshot_info.pic().clone(),
                    snapshot_info.big_pic().clone(),
                    0,
                    snapshot_info.click_count().clone(),
                    0,
                    snapshot_info.over_type_dict_text().clone(),
                    snapshot_info
                        .category_id()
                        .parse::<u64>()
                        .map_err(|err| format!("解析失败，{}", err))
                        .unwrap(),
                    0,
                    snapshot_info.tags().clone(),
                    vec![],
                )
            })
            .collect::<Vec<_>>();
        Ok(book_list)
    }

    pub(crate) async fn comic_info(&self, comic_id: &u64, limit: &u32) -> Result<Book, String> {
        let data = json!({
            "comicId": comic_id,
            "limit": limit,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let response_text = self
            .client
            .post_client("/api/h5/getComicInfo", &data)
            .await?;

        let book_info: BookInfo = serde_json::from_str(&response_text)
            .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;

        let book = Book::new(
            book_info
                .id()
                .parse::<u64>()
                .map_err(|err| format!("解析失败，{}", err))?,
            book_info.title().clone(),
            book_info.author().clone(),
            book_info.note().clone(),
            book_info.pic().clone(),
            book_info.big_pic().clone(),
            book_info.praise_count().clone(),
            book_info.click_count().clone(),
            book_info.fav_count().clone(),
            "".parse().unwrap(),
            book_info
                .category_id()
                .parse::<u64>()
                .map_err(|err| format!("解析失败，{}", err))
                .unwrap(),
            book_info.sort().clone(),
            book_info.tags().clone(),
            book_info
                .ext()
                .iter()
                .map(|chapter_info| {
                    Chapter::new(
                        chapter_info
                            .id()
                            .parse::<u64>()
                            .map_err(|err| format!("解析失败，{}", err))
                            .unwrap(),
                        chapter_info.title().clone(),
                        chapter_info.pic().clone(),
                        chapter_info.sort().clone(),
                        chapter_info.price().clone(),
                        vec![],
                        book_info
                            .id()
                            .parse::<u64>()
                            .map_err(|err| format!("解析失败，{}", err))
                            .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        );
        Ok(book)
    }
}

pub(crate) struct ChapterNetAccessor {
    pub(crate) client: Arc<NetClient>,
}
impl ChapterNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        ChapterNetAccessor { client }
    }

    pub(crate) async fn chapter_content(
        &self,
        chapter_id: &u64,
        user_id: &u64,
    ) -> Result<Vec<String>, String> {
        let data = json!({
            "chapterId": chapter_id,
            "userId": &user_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();
        let response_text = self
            .client
            .post_client("/api/h5/getChapterContent", &data)
            .await?;
        let item_info: ItemInfo = serde_json::from_str(&response_text)
            .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
        Ok(item_info.content().clone())
    }

    pub(crate) async fn pay_chapter(
        &self,
        user_id: &u64,
        comic_id: &u64,
        chapter_id: &u64,
    ) -> Result<(), String> {
        let data = json!({
            "userId": &user_id,
            "comicId": comic_id,
            "chapterId": chapter_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();
        let _ = self
            .client
            .post_client("/api/user/coinPay", &data)
            .await
            .map_err(|err| format!("用户{}支付失败: {}", user_id, err))?;
        Ok(())
    }
}

pub(crate) struct DailyNetAccessor {
    pub(crate) client: Arc<NetClient>,
}
impl DailyNetAccessor {
    pub(crate) fn new(client: Arc<NetClient>) -> Self {
        DailyNetAccessor { client }
    }
    pub(crate) async fn daily_sign(&self, user_id: &u64) -> Result<(), String> {
        let data = json!({
            "userId": &user_id,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let _ = self
            .client
            .post_client("/api/user/checkSign", &data)
            .await?;
        Ok(())
    }

    pub(crate) async fn daily_work(&self, task_no: &u8, user_id: &u64) -> Result<(), String> {
        let data = json!({
            "userId": &user_id,
            "task_no": task_no,
            "timeStamp": timestamp_str()?
        })
            .to_string();

        let _ = self
            .client
            .post_client("/api/user/getTaskReward", &data)
            .await?;
        Ok(())
    }
}
