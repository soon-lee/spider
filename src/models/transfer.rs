use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserInfo {
    id: String,
    account: String,
    pwd: String,
    nick_name: String,
    dev_type: u8,
    dev_code: String,
    last_login_ip: String,
    last_login_time: String,
    create_time: String,
    app_id: String,
    #[serde(default)]
    balance: u32,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct TaskInfo {
    id: String,
    task_no: u8,
    task_type: u8,
    trigger_value: u8,
    give_coin: u8,
    give_vip: u8,
    href_url: String,
    create_time: String,
    ext: u8,
    task_name: String,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct CategoryInfo {
    id: String,
    title: String,
    status: u8,
    sort: String,
    create_by: String,
    create_time: String,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnapshotBook {
    note: String,
    click_count: u64,
    is_syn: u8,
    pic: String,
    title: String,
    #[serde(rename = "overType_dictText")]
    over_type_dict_text: String,
    #[serde(rename = "categoryId_dictText")]
    category_id_dict_text: String,
    big_pic: String,
    id: String,
    author: String,
    over_type: u8,
    tags: String,
    category_id: String,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnapshotInfo {
    records: Vec<SnapshotBook>,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChapterInfo {
    id: String,
    title: String,
    pic: String,
    sort: u32,
    #[serde(default)]
    price: u32,
    is_syn: u8,
    create_time: String,
    feel: u8,
    pay_mode: u8,
    format_time: String,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct BookInfo {
    id: String,
    title: String,
    pic: String,
    big_pic: String,
    author: String,
    note: String,
    pay_mode: u8,
    feel_count: u8,
    pay_coin: u8,
    praise_count: u64,
    click_count: u64,
    fav_count: u64,
    sales: u8,
    pay_total: u8,
    over_type: u8,
    category_id: String,
    is_syn: u8,
    sort: u32,
    status: u8,
    tags: String,
    create_time: String,
    update_time: String,
    ext: Vec<ChapterInfo>,
}

#[derive(Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub(crate) struct ItemInfo {
    content: Vec<String>,
}