use serde::{Deserialize, Serialize};

/**
 * @locale: zh-CN
 * # `UserInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::UserInfo`
 * - **功能**：该结构体用于存储用户的基本信息，包括账户登录凭证、昵称、设备信息以及账户余额等相关数据。
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 * - `Serialize`：用于序列化结构体。
 * 
 * # 字段
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**：用户唯一标识符。
 * ## account
 * - **类型**：`alloc::string::String`
 *  - **描述**: 用户账号，用于登录。
 * ## pwd
 * - **类型**：`alloc::string::String`
 *   - **描述**: 用户密码，需妥善保管。
 * ## nickName
 * - **类型**：`alloc::string::String`
 *   - **描述**: 用户昵称，展示用。
 *  ## devType
 * - **类型**：`std::u8`
 *   - **描述**: 设备类型编码。
 *  ## devCode
 * - **类型**：`alloc::string::String`
 *   - **描述**: 设备唯一编码，与设备类型一起标识用户使用的设备。
 * ## lastLoginIp
 * - **类型**：`alloc::string::String`
 *   - **描述**: 最近一次登录的IP地址。
 * ## lastLoginTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 最后一次成功登录的时间戳。
 * ## createTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 账户创建时间。
 * ## appId
 * - **类型**：`alloc::string::String`
 * - **描述**: 应用ID，标识用户属于哪个应用。
 * ## balance
 * - **类型**：`alloc::string::String`
 * - **描述**: 用户账户余额，默认值由`default_balance`函数提供。
 *
 * @locale: en-US
 * # `UserInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::UserInfo`
 * - **Function**: This structure is used to store the basic information of the user, including the login credentials, nickname, device information, and account balance.
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: User unique identifier.
 * ## account
 * - **Type**: `alloc::string::String`
 *  - **Description**: User account, used for login.
 * ## pwd
 * - **Type**: `alloc::string::String`
 *   - **Description**: User password, need to keep it secure.
 * ## nickName
 * - **Type**: `alloc::string::String`
 *   - **Description**: User nickname, used for display.
 *  ## devType
 * - **Type**: `std::u8`
 *   - **Description**: Device type code.
 *  ## devCode
 * - **Type**: `alloc::string::String`
 *   - **Description**: Unique device code, together with the device type identifies the device used by the user.
 * ## lastLoginTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Last login IP address.
 * ## createTime
 * - **Type**: `alloc::string::String`
 * ## appId
 * - **Type**: `alloc::string::String`
 * ## balance
 * - **Type**: `alloc::string::String`
 * - **Description**: User account balance, default value provided by `default_balance` function.
*/
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UserInfo {
    pub(crate) id: String,
    pub(crate) account: String,
    pub(crate) pwd: String,
    nickName: String,
    devType: u8,
    devCode: String,
    lastLoginIp: String,
    lastLoginTime: String,
    createTime: String,
    appId: String,
    #[serde(default = "default_balance")]
    pub(crate) balance: u32,
}
/**
 * @locale: zh-CN
 * # 默认余额
 * - **功能**：返回默认余额，默认值为0。
 *
 * @locale: en-US
 * # Default balance
 * - **Function**: Returns the default balance, default value is 0.
 */
fn default_balance() -> u32 {
    0
}
/**
 * @locale: zh-CN
 * # `TaskInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::TaskInfo`
 * - **功能**：该结构体用于存储任务的基本信息，包括任务编号、任务类型、触发值、奖励金币、奖励VIP等级、任务链接、创建时间、扩展字段和任务名称。
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 *- `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**: 任务编号。
 * ## taskNo
 * - **类型**：`std::u8`
 *  - **描述**: 任务类型编码。
 * ## taskType
 * - **类型**：`std::u8`
 *  - **描述**: 触发值。
 * ## giveCoin
 * - **类型**：`std::u8`
 *  - **描述**: 奖励金币。
 * ## giveVip
 * - **类型**：`std::u8`
 *  - **描述**: 奖励VIP等级。
 * ## hrefUrl
 * - **类型**：`alloc::string::String`
 *  - **描述**: 任务链接。
 * ## createTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 创建时间。
 * ## ext
 * - **类型**：`std::u8`
 * - **描述**: 扩展字段。
 * ## taskName
 * - **类型**：`alloc::string::String`
 * - **描述**: 任务名称。
 *
 * @locale: en-US
 * # `TaskInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::TaskInfo`
 * - **Function**: This structure is used to store the basic information of the task, including the task number, task type, trigger value, reward coins, reward VIP level, task link, creation time, extension field, and task name.
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: Task number.
 * ## taskNo
 * - **Type**: `std::u8`
 *  - **Description**: Task type code.
 * ## taskType
 * - **Type**: `std::u8`
 *  - **Description**: Trigger value.
 * ## giveCoin
 * - **Type**: `std::u8`
 *  - **Description**: Reward coins.
 * ## giveVip
 * - **Type**: `std::u8`
 *  - **Description**: Reward VIP level.
 * ## hrefUrl
 * - **Type**: `alloc::string::String`
 *  - **Description**: Task link.
 * ## createTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Creation time.
 * ## ext
 * - **Type**: `std::u8`
 * - **Description**: Extension field.
 * ## taskName
 * - **Type**: `alloc::string::String`
 * - **Description**: Task name.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TaskInfo {
    id: String,
    pub(crate) taskNo: u8,
    taskType: u8,
    triggerValue: u8,
    pub(crate) giveCoin: u8,
    giveVip: u8,
    hrefUrl: String,
    createTime: String,
    ext: u8,
    pub(crate) taskName: String,
}
/**
 * @locale: zh-CN
 * # `CategoryInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::CategoryInfo`
 * - **功能**：该结构体用于存储分类信息，包括分类ID、分类标题、状态、排序、创建人、创建时间。
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 *- `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**: 分类ID。
 * ## title
 * - **类型**：`alloc::string::String`
 *  - **描述**: 分类标题。
 * ## status
 * - **类型**：`std::u8`
 *  - **描述**: 状态。
 * ## sort
 * - **类型**：`alloc::string::String`
 *  - **描述**: 排序。
 * ## createBy
 * - **类型**：`alloc::string::String`
 *  - **描述**: 创建人。
 * ## createTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 创建时间。
 *
 * @locale: en-US
 * # `CategoryInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::CategoryInfo`
 * - **Function**: This structure is used to store the category information, including the category ID, category title, status, sort, creator, and creation time.
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: Category ID.
 * ## title
 * - **Type**: `alloc::string::String`
 *  - **Description**: Category title.
 * ## status
 * - **Type**: `std::u8`
 *  - **Description**: Status.
 * ## sort
 * - **Type**: `alloc::string::String`
 *  - **Description**: Sort.
 * ## createBy
 * - **Type**: `alloc::string::String`
 *  - **Description**: Creator.
 * ## createTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Creation time.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CategoryInfo {
    pub(crate) id: String,
    pub(crate) title: String,
    status: u8,
    pub(crate) sort: String,
    createBy: String,
    createTime: String,
}
/**
 * @locale: zh-CN
 * # `SnapshotBook` 结构体
 * - **全限定名称**：`crate::tasks::dto::SnapshotBook`
 * - **功能**：该结构体用于存储书籍信息，包括书籍ID、书籍标题、书籍图片、书籍分类、书籍简介、书籍状态、书籍标签、书籍作者、书籍价格、书籍收费、书籍收费类型、书籍收费金额、书籍收费时间、书籍收费状态
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 * - `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## note
 * - **类型**：`alloc::string::String`
 * - **描述**: 笔记。
 * ## clickCount
 * - **类型**：`std::u64`
 * - **描述**: 点击次数。
 * ## isSyn
 * - **类型**：`std::u8`
 * - **描述**: 是否同步。
 * ## pic
 * - **类型**：`alloc::string::String`
 * - **描述**: 图片。
 * ## title
 * - **类型**：`alloc::string::String`
 * - **描述**: 标题。
 * ## overType_dictText
 * - **类型**：`alloc::string::String`
 * - **描述**: 分类名称。
 * ## categoryId_dictText
 * - **类型**：`alloc::string::String`
 * - **描述**: 分类名称。
 * ## bigPic
 * - **类型**：`alloc::string::String`
 * - **描述**: 大图。
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**: ID。
 * ## author
 * - **类型**：`alloc::string::String`
 * - **描述**: 作者。
 * ## overType
 * - **类型**：`std::u8`
 * - **描述**: 完结状态。
 * ## tags
 * - **类型**：`alloc::string::String`
 * - **描述**: 标签。
 * ## categoryId
 * - **类型**：`alloc::string::String`
 * - **描述**: 分类ID。
 *
 * @locale: en-US
 * # `SnapshotBook` structure
 * - **Full Qualified Name**: `crate::tasks::dto::SnapshotBook`
 * - **Function**: This structure is used to store book information, including book ID, book title, book image, book category, book introduction, book status, book tags, book author, book price, book charge, book charge type, book charge amount, book charge time, book charge status
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## note
 * - **Type**: `alloc::string::String`
 * - **Description**: Note.
 * ## clickCount
 * - **Type**: `std::u64`
 * - **Description**: Click count.
 * ## isSyn
 * - **Type**：`std::u8`
 * - **Description**: Whether to synchronize.
 * ## pic
 * - **Type**: `alloc::string::String`
 * - **Description**: Picture.
 * ## title
 * - **Type**: `alloc::string::String`
 * - **Description**: Title.
 * ## overType_dictText
 * - **Type**: `alloc::string::String`
 * - **Description**: Category name.
 * ## categoryId_dictText
 * - **Type**: `alloc::string::String`
 * - **Description**: Category name.
 * ## bigPic
 * - **Type**: `alloc::string::String`
 * - **Description**: Big picture.
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: ID.
 * ## author
 * - **Type**: `alloc::string::String`
 * - **Description**: Author.
 * ## overType
 * - **Type**: `std::u8`
 * - **Description**: Finish status.
 * ## tags
 * - **Type**: `alloc::string::String`
 * - **Description**: Tags.
 * ## categoryId
 * - **Type**: `alloc::string::String`
 * - **Description**: Category ID.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SnapshotBook {
    pub(crate) note: String,
    pub(crate) clickCount: u64,
    isSyn: u8,
    pub(crate) pic: String,
    pub(crate) title: String,
    pub(crate) overType_dictText: String,
    categoryId_dictText: String,
    pub(crate) bigPic: String,
    pub(crate) id: String,
    pub(crate) author: String,
    overType: u8,
    pub(crate) tags: String,
    pub(crate) categoryId: String,
}
/**
 * @locale: zh-CN
 * # `SnapshotInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::SnapshotInfo`
 * - **功能**：该结构体用于存储书籍信息，包括书籍ID、书籍标题、书籍图片、书籍分类、书籍简介、书籍状态、书籍标签、书籍作者、书籍价格、书籍收费、书籍收费类型、书籍收费金额、书籍收费时间、书籍收费状态
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 * - `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## records
 * - **类型**：`alloc::vec::Vec<crate::tasks::dto::SnapshotBook>`
 * - **描述**: 书籍列表。
 *
 * @locale: en-US
 * # `SnapshotInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::SnapshotInfo`
 * - **Function**: This structure is used to store book information, including book ID, book title, book image, book category, book introduction, book status, book tags, book author, book price, book charge, book charge type, book charge amount, book charge time, book charge status
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## records
 * - **Type**: `alloc::vec::Vec<crate::tasks::dto::SnapshotBook>`
 * - **Description**: Book list.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SnapshotInfo {
    pub(crate) records: Vec<SnapshotBook>,
}
/**
 * @locale: zh-CN
 * # `ChapterInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::ChapterInfo`
 * - **功能**：该结构体用于存储书籍信息，包括书籍ID、书籍标题、书籍图片、书籍分类、书籍简介、书籍状态、书籍标签、书籍作者、书籍价格、书籍收费、书籍收费类型、书籍收费金额、书籍收费时间、书籍收费状态
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 * - `Deserialize`：用于反序列化结构体。
 * - `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**: ID。
 * ## title
 * - **类型**：`alloc::string::String`
 * - **描述**: 标题。
 * ## pic
 * - **类型**：`alloc::string::String`
 * - **描述**: 图片。
 * ## sort
 * - **类型**：`std::u32`
 * - **描述**: 排序。
 * ## price
 * - **类型**：`std::u32`
 * - **描述**: 价格。
 * ## isSyn
 * - **类型**：`std::u8`
 * - **描述**: 是否同步。
 * ## createTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 创建时间。
 * ## feel
 * - **类型**：`std::u8`
 * - **描述**: 阅读量。
 * ## payMode
 * - **类型**：`std::u8`
 * - **描述**: 收费模式。
 * ## formatTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 格式化时间。
 *
 * @locale: en-US
 * # `ChapterInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::ChapterInfo`
 * - **Function**: This structure is used to store book information, including book ID, book title, book image, book category, book introduction, book status, book tags, book author, book price, book charge, book charge type, book charge amount, book charge time, book charge status
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: ID.
 * ## title
 * - **Type**: `alloc::string::String`
 * - **Description**: Title.
 * ## pic
 * - **Type**: `alloc::string::String`
 * - **Description**: Picture.
 * ## sort
 * - **Type**: `std::u32`
 * - **Description**: Sort.
 * ## price
 * - **Type**: `std::u32`
 * - **Description**: Price.
 * ## isSyn
 * - **Type**: `std::u8`
 * - **Description**: Whether to synchronize.
 * ## createTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Create time.
 * ## feel
 * - **Type**: `std::u8`
 * - **Description**: Readings.
 * ## payMode
 * - **Type**: `std::u8`
 * - **Description**: Payment mode.
 * ## formatTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Formatted time.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ChapterInfo {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) pic: String,
    pub(crate) sort: u32,
    #[serde(default = "default_price")]
    pub(crate) price: u32,
    isSyn: u8,
    createTime: String,
    feel: u8,
    payMode: u8,
    formatTime: String,
}
/**
 * @locale: zh-CN
 * # `default_price` 函数
 * - **功能**：默认价格。
 * - **返回值**：`u32`
 *
 * @locale: en-US
 * # `default_price` function
 * - **Function**: Default price.
 * - **Return Value**: `u32`
 *
 */
fn default_price() -> u32 {
    0
}
/**
 * @locale: zh-CN
 * # `BookInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::BookInfo`
 * - **功能**：该结构体用于存储书籍信息，包括书籍ID、书籍标题、书籍图片、书籍分类、书籍简介、书籍状态、书籍标签、书籍作者、书籍价格、书籍收费、书籍收费类型、书籍收费金额、书籍收费时间、书籍收费状态
 *
 * # 特质
 * - `Debug`：用于调试结构体。
 *- `Deserialize`：用于反序列化结构体。
 * - `Serialize`：用于序列化结构体。
 *
 * # 字段
 * ## id
 * - **类型**：`alloc::string::String`
 * - **描述**: ID。
 * ## title
 * - **类型**：`alloc::string::String`
 * - **描述**: 标题。
 * ## pic
 * - **类型**：`alloc::string::String`
 * - **描述**: 图片。
 * ## bigPic
 * - **类型**：`alloc::string::String`
 * - **描述**: 大图。
 * ## author
 * - **类型**：`alloc::string::String`
 * - **描述**: 作者。
 * ## note
 * - **类型**：`alloc::string::String`
 * - **描述**: 说明。
 * ## payMode
 * - **类型**：`std::u8`
 * - **描述**: 收费模式。
 * ## feelCount
 * - **类型**：`std::u8`
 * - **描述**: 阅读量。
 * ## payCoin
 * - **类型**：`std::u8`
 * - **描述**: 收费金额。
 * ## praiseCount
 * - **类型**：`std::u64`
 * - **描述**: 点赞量。
 * ## clickCount
 * - **类型**：`std::u64`
 * - **描述**: 点击量。
 * ## favCount
 * - **类型**：`std::u64`
 * - **描述**: 收藏量。
 * ## sales
 * - **类型**：`std::u8`
 * - **描述**: 销量。
 * ## payTotal
 * - **类型**：`std::u8`
 * - **描述**: 总计金额。
 * ## overType
 * - **类型**：`std::u8`
 * - **描述**: 完结状态。
 * ## categoryId
 * - **类型**：`alloc::string::String`
 * - **描述**: 分类ID。
 * ## isSyn
 * - **类型**：`std::u8`
 * - **描述**: 是否同步。
 * ## sort
 * - **类型**：`std::u32`
 * - **描述**: 排序。
 * ## status
 * - **类型**：`std::u8`
 * - **描述**: 状态。
 * ## tags
 * - **类型**：`alloc::string::String`
 * - **描述**: 标签。
 * ## indexCol
 * - **类型**：`alloc::string::String`
 * - **描述**: 索引列。
 * ## createTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 创建时间。
 * ## updateTime
 * - **类型**：`alloc::string::String`
 * - **描述**: 更新时间。
 * ## ext
 * - **类型**：`alloc::vec::Vec<crate::tasks::dto::ChapterInfo>`
 * - **描述**: 扩展信息。
 *
 * @locale: en-US
 * # `BookInfo` structure
 * - **Full Qualified Name**: `crate::tasks::dto::BookInfo`
 * - **Function**: This structure is used to store book information, including book ID, book title, book image, book category, book introduction, book status, book tags, book author, book price, book charge, book charge type, book charge amount, book charge time, book charge status
 *
 * # Trait
 * - `Debug`: Used for debugging the structure.
 * - `Deserialize`: Used for deserializing the structure.
 * - `Serialize`: Used for serializing the structure.
 *
 * # Fields
 * ## id
 * - **Type**: `alloc::string::String`
 * - **Description**: ID.
 * ## title
 * - **Type**: `alloc::string::String`
 * - **Description**: Title.
 * ## pic
 * - **Type**: `alloc::string::String`
 * - **Description**: Picture.
 * ## bigPic
 * - **Type**: `alloc::string::String`
 * - **Description**: Big picture.
 * ## author
 * - **Type**: `alloc::string::String`
 * - **Description**: Author.
 * ## payMode
 * - **Type**: `std::u8`
 * - **Description**: Payment mode.
 * ## feelCount
 * - **Type**: `std::u8`
 * - **Description**: Readings.
 * ## payCoin
 * - **Type**: `std::u8`
 * - **Description**: Payment amount.
 * ## praiseCount
 * - **Type**: `std::u64`
 * - **Description**: Likes.
 * ## clickCount
 * - **Type**: `std::u64`
 * - **Description**: Clicks.
 * ## favCount
 * - **Type**: `std::u64`
 * - **Description**: Favorites.
 * ## sales
 * - **Type**: `std::u8`
 * - **Description**: Sales.
 * ## payTotal
 * - **Type**: `std::u8`
 * - **Description**: Total amount.
 * ## overType
 * - **Type**: `std::u8`
 * - **Description**: Completion status.
 * ## categoryId
 * - **Type**: `alloc::string::String`
 * - **Description**: Category ID.
 * ## isSyn
 * - **Type**: `std::u8`
 * - **Description**: Whether to synchronize.
 * ## sort
 * - **Type**: `std::u32`
 * - **Description**: Sort.
 * ## status
 * - **Type**: `std::u8`
 * - **Description**: Status.
 * ## tags
 * - **Type**: `alloc::string::String`
 * - **Description**: Tags.
 * ## indexCol
 * - **Type**: `alloc::string::String`
 * - **Description**: Index column.
 * ## createTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Creation time.
 * ## updateTime
 * - **Type**: `alloc::string::String`
 * - **Description**: Update time.
 * ## ext
 * - **Type**: `alloc::vec::Vec<crate::tasks::dto::ChapterInfo>`
 * - **Description**: Extended information.
 *
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookInfo {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) pic: String,
    pub(crate) bigPic: String,
    pub(crate) author: String,
    pub(crate) note: String,
    payMode: u8,
    feelCount: u8,
    payCoin: u8,
    pub(crate) praiseCount: u64,
    pub(crate) clickCount: u64,
    pub(crate) favCount: u64,
    sales: u8,
    payTotal: u8,
    overType: u8,
    pub(crate) categoryId: String,
    isSyn: u8,
    pub(crate) sort: u32,
    status: u8,
    pub(crate) tags: String,
    indexCol: String,
    createTime: String,
    updateTime: String,
    pub(crate) ext: Vec<ChapterInfo>,
}
/**
 * @locale zh-CN
 * # `ItemInfo` 结构体
 * - **全限定名称**：`crate::tasks::dto::ItemInfo`
 * - **功能**：此结构体用于存储项信息，主要包含内容列表，适用于需要保存一系列字符串数据的场景。
 *
 * # 特质
 * - `Debug`：为结构体提供调试打印格式。
 * - `Deserialize`：允许从JSON等格式反序列化实例。
 * - `Serialize`：允许序列化实例为JSON等格式。
 *
 * # 字段
 * ## content
 * - **类型**：`alloc::vec::Vec<alloc::string::String>`
 * - **描述**: 存储字符串内容的列表，每个元素代表一项具体内容。
 *
 * @locale en-US
 * # `ItemInfo` Structure
 * - **Full Qualified Name**: `crate::tasks::dto::ItemInfo`
 * - **Function**: This structure is used to store item information, primarily containing a list of contents, suitable for scenarios where a series of string data needs to be saved.
 *
 * # Trait
 * - `Debug`: Provides a debug print format for the structure.
 * - `Deserialize`: Enables deserialization of instances from formats like JSON.
 * - `Serialize`: Enables serialization of instances into formats like JSON.
 *
 * # Fields
 * ## content
 * - **Type**: `alloc::vec::Vec<alloc::string::String>`
 * - **Description**: A list storing string contents, where each element represents a specific piece of content.
 */
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ItemInfo {
    pub(crate) content: Vec<String>,
}