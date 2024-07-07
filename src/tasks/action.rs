use std::collections::HashMap;

use reqwest::Error;
use serde_json::{json, Value};

use crate::states::local::Config;
use crate::states::mysql::{Book, Category, Chapter, Task, User};
use crate::tasks::dto::{BookInfo, CategoryInfo, ItemInfo, SnapshotInfo, TaskInfo, UserInfo};
use crate::utils::crypt::{aes_decrypt, aes_encrypt, auth_path};
use crate::utils::datetime::timestamp_str;

/**
 * @locale zh-CN
 * # `post_client` 异步函数
 * - **全限定名称**：`crate::tasks::action::post_client`
 * - **功能**：向指定URL发送异步POST请求，携带经过AES加密的数据。请求前，使用`auth_path`函数对路径进行鉴权处理，确保请求的安全性。响应数据同样进行AES解密，返回解密后的结果。
 *
 * # 参数
 * ## `path`
 * - **类型**：&std::str
 * - **描述**：目标API的路径部分。
 * ## `data`
 * - **类型**：&std::str
 * - **描述**：待加密并发送的数据。
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）、`template_str`（随机字符串模板）、`default_str`（默认字符串）和`aes_key`（AES密钥）。
 *
 * # 返回
 * - **类型**：core::result::Result<alloc::string::String, alloc::string::String>
 * - **说明**：成功时返回解密后的响应数据；失败则返回错误信息，包括但不限于加密/解密错误、HTTP请求失败或响应数据解析错误。
 *
 * # 注意
 * - 本函数依赖于`reqwest`库来执行HTTP请求。
 * - 所有必要的选项必须在`options`参数中提供，否则将导致错误。
 * - 使用`aes_encrypt`和`aes_decrypt`函数对数据进行加密和解密。
 * - 请求头中包含`Appid`和`Content-Type`，其中`Content-Type`设置为`application/json`。
 *
 * # 示例
 * ```rust
 * let path = "/api/data";
 * let data = "{"key":"value"}";
 * let options = HashMap::from([
 *     ("origin".to_string(), "https://example.com".to_string()),
 *     ("app_id".to_string(), "123456".to_string())
 * ]);
 * match post_client(path, data, &options).await {
 *     Ok(response) => println!("响应数据: {}", response),
 *     Err(err) => println!("异常: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `post_client` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::post_client`
 * - **Function**: Sends an asynchronous POST request to a specified URL carrying AES-encrypted data. Before sending the request, the path is authenticated using the `auth_path` function to ensure request security. The response data is also AES-decrypted, returning the decrypted result.
 *
 * # Parameters
 * ## `path`
 * - **Type**: &std::str
 * - **Description**: The path component of the target API.
 * ## `data`
 * - **Type**: &std::str
 * - **Description**: Data to be encrypted and sent.
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), `template_str` (random string template), `default_str` (default string), and `aes_key` (AES key).
 *
 * # Returns
 * - **Type**: core::result::Result<alloc::string::String, alloc::string::String>
 * - **Description**: On success, returns the decrypted response data; on failure, returns an error message, including but not limited to encryption/decryption errors, HTTP request failures, or response data parsing errors.
 *
 * # Note
 * - This function relies on the `reqwest` library to perform HTTP requests.
 * - All necessary options must be provided in the `options` parameter, otherwise, errors will occur.
 * - Data is encrypted and decrypted using the `aes_encrypt` and `aes_decrypt` functions.
 * - The request headers include `Appid` and `Content-Type`, where `Content-Type` is set to `application/json`.
 *
 * # Examples
 * ```rust
 * let path = "/api/data";
 * let data = "{"key":"value"}";
 * let options = HashMap::from([
 *     ("origin".to_string(), "https://example.com".to_string()),
 *     ("app_id".to_string(), "123456".to_string())
 * ]);
 * match post_client(path, data, &options).await {
 *     Ok(response) => println!("Response data: {}", response),
 *     Err(err) => println!("Error: {}", err),
 * }
 * ```
 */
pub(crate) async fn post_client(
    path: &str,
    data: &str,
    options: &HashMap<String, String>,
) -> Result<String, String> {
    let origin = options.get("origin").ok_or("缺少origin参数")?;
    let app_id = options.get("app_id").ok_or("缺少app_id参数")?;
    let template = options.get("template_str").ok_or("缺少template_str参数")?;
    let default_str = options.get("default_str").ok_or("缺少default_str参数")?;
    let aes_key = options.get("aes_key").ok_or("缺少aes_key参数")?;

    let encrypted_data = aes_encrypt(aes_key, data)?;

    let client = reqwest::Client::new();
    let authed_path = auth_path(path, template, default_str)?;
    let url = format!("{}{}", origin, authed_path);
    let response = client
        .post(&url)
        .header("Appid", app_id)
        .header("Content-Type", "application/json")
        .body(format!("{{\"data\":\"{}\"}}", encrypted_data))
        .send()
        .await
        .map_err(|err| format!("请求失败: {}", err))?;

    let json = response
        .json::<Value>()
        .await
        .map_err(|err| format!("json解析失败: {}", err))?;
    let success = json["success"]
        .as_bool()
        .ok_or("返回数据中没有success字段")?;
    if success {
        let decrypted_data = json["result"]
            .as_str()
            .ok_or("返回数据中没有data字段")
            .unwrap();
        let text = aes_decrypt(aes_key, decrypted_data)?;
        Ok(text)
    } else {
        Err(format!("请求{}时发生异常: 请求失败{}", url, json))
    }
}
/**
 * @locale zh-CN
 * # `register_user` 异步函数
 * - **全限定名称**：`crate::tasks::action::register_user`
 * - **功能**：使用提供的选项注册用户，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括设备类型和时间戳，响应数据被解析为`UserInfo`结构体，并从中提取用户ID，最终创建并返回一个`User`实例。
 *
 * # 参数
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`dev_type`（设备类型）。`origin`、`app_id`、`template_str`、`default_str`和`aes_key`等选项应存在于`options`中，用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<crate::models::User, alloc::string::String>
 * - **说明**：成功时返回新创建的`User`实例；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误或用户信息转换错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 * - 用户ID从`UserInfo`结构体中解析为`u64`类型。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match register_user(&options).await {
 *     Ok(user) => println!("注册用户: {:?}", user),
 *     Err(err) => println!("注册失败: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `register_user` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::register_user`
 * - **Function**: Registers a user using provided options by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes device type and timestamp. Response data is parsed into a `UserInfo` struct from which the user ID is extracted, ultimately creating and returning a `User` instance.
 *
 * # Parameters
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `dev_type` (device type). Options such as `origin`, `app_id`, `template_str`, `default_str`, and `aes_key` should be present in `options` for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<crate::models::User, alloc::string::String>
 * - **Description**: On success, returns a newly created `User` instance; on failure, returns an error message, including but not limited to HTTP request failures, JSON parsing errors, or user info conversion errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 * - The user ID is parsed from the `UserInfo` struct into a `u64` type.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match register_user(&options).await {
 *     Ok(user) => println!("Registered user: {:?}", user),
 *     Err(err) => println!("Registration failed: {}", err),
 * }
 * ```
 */
pub(crate) async fn register_user(options: &HashMap<String, String>) -> Result<User, String> {
    let dev_type = options.get("dev_type").ok_or("缺少origin参数")?;

    let data = json!({
        "devType": dev_type,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/user/regUser", &data, options).await?;

    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("UserInfo序列化失败: {}", err))?;

    let user_id = user_info
        .id
        .parse::<u64>()
        .map_err(|err| format!("UserInfo序列化失败: {}", err))?;

    let user = User::new(user_id, user_info.account, user_info.pwd, 0);
    Ok(user)
}
/**
 * @locale zh-CN
 * # `user_info` 异步函数
 * - **全限定名称**：`crate::tasks::action::user_info`
 * - **功能**：根据用户ID获取用户信息，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括用户ID和时间戳，响应数据被解析为`UserInfo`结构体，然后创建并返回一个`User`实例。
 *
 * # 参数
 * ## `user_id`
 * - **类型**：std::u64
 * - **描述**：要查询的用户ID。
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）、`template_str`（随机字符串模板）、`default_str`（默认字符串）和`aes_key`（AES密钥）。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<crate::models::User, alloc::string::String>
 * - **说明**：成功时返回包含用户信息的`User`实例；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误或用户ID转换错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 * - 用户ID从`UserInfo`结构体中解析为`u64`类型。
 *
 * # 示例
 * ```rust
 * let user_id = 1234567890;
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match user_info(user_id, &options).await {
 *     Ok(user) => println!("用户信息: {:?}", user),
 *     Err(err) => println!("获取用户信息失败: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `user_info` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::user_info`
 * - **Function**: Retrieves user information based on the user ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the user ID and timestamp. Response data is parsed into a `UserInfo` struct, then a `User` instance is created and returned.
 *
 * # Parameters
 * ## `user_id`
 * - **Type**: std::u64
 * - **Description**: The ID of the user to query.
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), `template_str` (random string template), `default_str` (default string), and `aes_key` (AES key). These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<crate::models::User, alloc::string::String>
 * - **Description**: On success, returns a `User` instance containing user information; on failure, returns an error message, including but not limited to HTTP request failures, JSON parsing errors, or user ID conversion errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 * - The user ID is parsed from the `UserInfo` struct into a `u64` type.
 *
 * # Examples
 * ```rust
 * let user_id = 1234567890;
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match user_info(user_id, &options).await {
 *     Ok(user) => println!("User info: {:?}", user),
 *     Err(err) => println!("User info retrieval failed: {}", err),
 * }
 * ```
 */
pub(crate) async fn user_info(
    user_id: u64,
    options: &HashMap<String, String>,
) -> Result<User, String> {
    let data = json!({
        "userId": user_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/user/getUserInfo", &data, options).await?;

    let user_info: UserInfo = serde_json::from_str(&response_text)
        .map_err(|e| format!("从Json解析UserInfo失败: {}", e))?;

    let parsed_user_id = user_info
        .id
        .parse::<u64>()
        .map_err(|_| "字符串解析用户主键失败".to_string())?;

    let user = User::new(
        parsed_user_id,
        user_info.account,
        user_info.pwd,
        user_info.balance,
    );
    Ok(user)
}
/**
 * @locale zh-CN
 * # `task_list` 异步函数
 * - **全限定名称**：`crate::tasks::action::task_list`
 * - **功能**：根据用户ID获取任务列表，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括用户ID和时间戳，响应数据被解析为`TaskInfo`结构体的列表，然后创建并返回一个`Task`实例的列表。
 *
 * # 参数
 * ## `user_id`
 * - **类型**：u64
 * - **描述**：要查询的用户ID。
 * ## `options`
 * - **类型**：&std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<Vec<crate::models::Task>, alloc::string::String>
 * - **说明**：成功时返回包含任务信息的`Task`实例列表；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 *
 * # 示例
 * ```rust
 * let user_id = 1234567890;
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match task_list(user_id, &options).await {
 *     Ok(tasks) => println!("任务列表: {:?}", tasks),
 *     Err(err) => println!("获取任务列表失败: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `task_list` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::task_list`
 * - **Function**: Retrieves a list of tasks based on the user ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the user ID and timestamp. Response data is parsed into a list of `TaskInfo` structs, then a list of `Task` instances is created and returned.
 *
 * # Parameters
 * ## `user_id`
 * - **Type**: u64
 * - **Description**: The user ID to query.
 * ## `options`
 * - **Type**: &std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<Vec<crate::models::Task>, alloc::string::String>
 * - **Description**: On success, returns a list of `Task` instances containing task information; on failure, returns an error message, including but not limited to HTTP request failures or JSON parsing errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 *
 * # Examples
 * ```rust
 * let user_id = 1234567890;
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match task_list(user_id, &options).await {
 *     Ok(tasks) => println!("Task list: {:?}", tasks),
 *     Err(err) => println!("Task list retrieval failed: {}", err),
 * }
 * ```
 */
pub(crate) async fn task_list(
    user_id: u64,
    options: &HashMap<String, String>,
) -> Result<Vec<Task>, String> {
    let data = json!({
        "userId": user_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/user/getTaskList", &data, options).await?;

    let task_info_list: Vec<TaskInfo> = serde_json::from_str(&response_text)
        .map_err(|e| format!("从Json解析TaskInfo列表失败: {}", e))?;

    let task_list = task_info_list
        .into_iter()
        .map(|task_info| Task::new(task_info.taskNo, task_info.giveCoin, task_info.taskName))
        .collect::<Vec<_>>();

    Ok(task_list)
}
/**
 * @locale zh-CN
 * # `category_list` 异步函数
 * - **全限定名称**：`crate::tasks::action::category_list`
 * - **功能**：获取分类列表，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括分类代码和时间戳，响应数据被解析为`CategoryInfo`结构体的列表，然后创建并返回一个`Category`实例的列表。
 *
 * # 参数
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<Vec<crate::models::Category>, alloc::string::String>
 * - **说明**：成功时返回包含分类信息的`Category`实例列表；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误或数据转换错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 * - 分类ID和排序值从`CategoryInfo`结构体中解析为`u64`和`u32`类型。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match category_list(&options).await {
 *     Ok(categories) => println!("分类列表: {:?}", categories),
 *     Err(err) => println!("获取分类列表失败: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `category_list` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::category_list`
 * - **Function**: Retrieves a list of categories by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the category code and timestamp. Response data is parsed into a list of `CategoryInfo` structs, then a list of `Category` instances is created and returned.
 *
 * # Parameters
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<Vec<crate::models::Category>, alloc::string::String>
 * - **Description**: On success, returns a list of `Category` instances containing category information; on failure, returns an error message, including but not limited to HTTP request failures, JSON parsing errors, or data conversion errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 * - The category ID and sort value are parsed from the `CategoryInfo` struct into `u64` and `u32` types.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match category_list(&options).await {
 *     Ok(categories) => println!("Category list: {:?}", categories),
 *     Err(err) => println!("Category list retrieval failed: {}", err),
 * }
 * ```
 */
pub(crate) async fn category_list(
    options: &HashMap<String, String>,
) -> Result<Vec<Category>, String> {
    let data = json!({
        "c": "yml",
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/h5/getCategory", &data, options).await?;

    let category_info_list: Vec<CategoryInfo> = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析CategoryInfo列表失败: {}", err))?;

    let category_list = category_info_list
        .into_iter()
        .map(|category_info| {
            let id = category_info
                .id
                .parse::<u64>().unwrap();
            let sort = category_info
                .sort
                .parse::<u32>().unwrap();
            Category::new(id, category_info.title, sort)
        })
        .collect::<Vec<_>>();

    Ok(category_list)
}
/**
 * @locale zh-CN
 * # `snapshot_list` 异步函数
 * - **全限定名称**：`crate::tasks::action::snapshot_list`
 * - **功能**：根据分类ID、页码和每页限制获取漫画快照列表，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括分类ID、页码、每页限制和时间戳，响应数据被解析为`SnapshotInfo`结构体，然后创建并返回一个`Book`实例的列表。
 *
 * # 参数
 * ## `category_id`
 * - **类型**：std::u64
 * - **描述**：要查询的分类ID。
 * ## `page`
 * - **类型**：std::u32
 * - **描述**：要获取的页码。
 * ## `limit`
 * - **类型**：std::u32
 * - **描述**：每页的条目数量限制。
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<Vec<crate::models::Book>, alloc::string::String>
 * - **说明**：成功时返回包含漫画信息的`Book`实例列表；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 * - 分类ID从`SnapshotInfo`结构体中解析为`u64`类型。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match snapshot_list(1, 1, 10, &options).await {
 *     Ok(books) => println!("漫画快照列表: {:?}", books),
 *     Err(err) => println!("获取漫画快照列表失败: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `snapshot_list` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::snapshot_list`
 * - **Function**: Retrieves a list of comic snapshots based on the category ID, page number, and limit by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the category ID, page number, limit, and timestamp. Response data is parsed into a `SnapshotInfo` struct, then a list of `Book` instances is created and returned.
 *
 * # Parameters
 * ## `category_id`
 * - **Type**: std::u64
 * - **Description**: The category ID to query.
 * ## `page`
 * - **Type**: std::u32
 * - **Description**: The page number to retrieve.
 * ## `limit`
 * - **Type**: std::u32
 * - **Description**: The limit of entries per page.
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<Vec<crate::models::Book>, alloc::string::String>
 * - **Description**: On success, returns a list of `Book` instances containing comic information; on failure, returns an error message, including but not limited to HTTP request failures or JSON parsing errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 * - The category ID is parsed from the `SnapshotInfo` struct into a `u64` type.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match snapshot_list(1, 1, 10, &options).await {
 *     Ok(books) => println!("Book list: {:?}", books),
 *     Err(err) => println!("Book list retrieval failed: {}", err),
 * }
 * ```
 */
pub(crate) async fn snapshot_list(
    category_id: u64,
    page: u32,
    limit: u32,
    options: &HashMap<String, String>,
) -> Result<Vec<Book>, String> {
    let data = json!({
        "page": page,
        "limit": limit,
        "categoryId": category_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/h5/getComicByCategoryId", &data, options).await?;

    let snapshot_info: SnapshotInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
    let book_list = snapshot_info
        .records
        .iter()
        .map(|snapshot_info| {
            Book::new(
                snapshot_info.id.clone(),
                snapshot_info.title.clone(),
                snapshot_info.author.clone(),
                snapshot_info.note.clone(),
                snapshot_info.pic.clone(),
                snapshot_info.bigPic.clone(),
                0,
                snapshot_info.clickCount,
                0,
                snapshot_info.overType_dictText.clone(),
                snapshot_info.categoryId.parse::<u64>().unwrap(),
                0,
                snapshot_info.tags.clone(),
                vec![]
            )
        })
        .collect::<Vec<_>>();
    Ok(book_list)
}
/**
 * @locale zh-CN
 * # `comic_info` 异步函数
 * - **全限定名称**：`crate::tasks::action::comic_info`
 * - **功能**：根据漫画ID和每页限制获取漫画详细信息，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括漫画ID、每页限制和时间戳，响应数据被解析为`BookInfo`结构体，然后创建并返回一个`Book`实例。
 *
 * # 参数
 * ## `comic_id`
 * - **类型**：u64
 * - **描述**：要查询的漫画ID。
 * ## `limit`
 * - **类型**：u32
 * - **描述**：每页的条目数量限制。
 * ## `options`
 * - **类型**：&std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<crate::models::Book, alloc::string::String>
 * - **说明**：成功时返回包含漫画详细信息的`Book`实例；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 * - 漫画ID和分类ID从`BookInfo`结构体中解析为`u64`类型。
 * - 章节信息从`BookInfo`的`ext`字段中提取，并转换为`Chapter`实例列表。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match comic_info(1, 1, &options).await {
 *     Ok(book) => println!("漫画详细信息: {:?}", book),
 *     Err(_) => println!("获取漫画详细信息失败"),
 * }
 * ```
 *
 * @locale en-US
 * # `comic_info` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::comic_info`
 * - **Function**: Retrieves detailed comic information based on the comic ID and limit by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the comic ID, limit, and timestamp. Response data is parsed into a `BookInfo` struct, then a `Book` instance is created and returned.
 *
 * # Parameters
 * ## `comic_id`
 * - **Type**: u64
 * - **Description**: The comic ID to query.
 * ## `limit`
 * - **Type**: u32
 * - **Description**: The limit of entries per page.
 * ## `options`
 * - **Type**: &std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<crate::models::Book, alloc::string::String>
 * - **Description**: On success, returns a `Book` instance containing detailed comic information; on failure, returns an error message, including but not limited to HTTP request failures, JSON parsing errors, typically indicating HTTP request failures or JSON parsing errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 * - The comic ID and category ID are parsed from the `BookInfo` struct into `u64` types.
 * - Chapter information is extracted from the `ext` field of `BookInfo` and converted into a list of `Chapter` instances.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match comic_info(1, 1, &options).await {
 *     Ok(book) => println!("Book info: {:?}", book),
 *     Err(_) => println!("Book info retrieval failed"),
 * }
 * ```
 */
pub(crate) async fn comic_info(
    comic_id: u64,
    limit: u32,
    options: &HashMap<String, String>,
) -> Result<Book, String> {
    let data = json!({
        "comicId": comic_id,
        "limit": limit,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/h5/getComicInfo", &data, options).await?;
    let book_info: BookInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;

    let book = Book::new(
        book_info.id.clone(),
        book_info.title.clone(),
        book_info.author.clone(),
        book_info.note.clone(),
        book_info.pic.clone(),
        book_info.bigPic.clone(),
        book_info.praiseCount,
        book_info.clickCount,
        book_info.favCount,
        "".parse().unwrap(),
        book_info.categoryId.parse::<u64>().unwrap(),
        book_info.sort,
        book_info.tags.clone(),
        book_info
            .ext
            .iter()
            .map(|chapter_info| {
                Chapter::new(
                    chapter_info.id.clone(),
                    chapter_info.title.clone(),
                    chapter_info.pic.clone(),
                    chapter_info.sort,
                    chapter_info.price,
                    vec![],
                )
            })
            .collect::<Vec<_>>(),
    );
    Ok(book)
}
/**
 * @locale zh-CN
 * # `chapter_content` 异步函数
 * - **全限定名称**：`crate::tasks::action::chapter_content`
 * - **功能**：根据章节ID和用户ID获取章节内容，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括章节ID、用户ID和时间戳，响应数据被解析为`ItemInfo`结构体，然后返回章节内容作为字符串列表。
 *
 * # 参数
 * ## `chapter_id`
 * - **类型**：std::u64
 * - **描述**：要查询的章节ID。
 * ## `user_id`
 * - **类型**：std::u64
 * - **描述**：用户ID，用于关联请求的用户信息。
 * ## `options`
 * - **类型**：&std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<Vec<alloc::string::String>, alloc::string::String>
 * - **说明**：成功时返回包含章节内容的字符串列表；失败则返回错误信息，包括但不限于HTTP请求失败、JSON解析错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 使用`serde_json`库进行JSON数据的序列化和反序列化。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match chapter_content(1, 1, &options).await {
 *     Ok(content) => println!("章节内容: {:?}", content),
 *     Err(_) => println!("获取章节内容失败"),
 * }
 * ```
 *
 * @locale en-US
 * # `chapter_content` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::chapter_content`
 * - **Function**: Retrieves chapter content based on the chapter ID and user ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the chapter ID, user ID, and timestamp. Response data is parsed into an `ItemInfo` struct, then the chapter content is returned as a list of strings.
 *
 * # Parameters
 * ## `chapter_id`
 * - **Type**: std::u64
 * - **Description**: The chapter ID to query.
 * ## `user_id`
 * - **Type**: std::u64
 * - **Description**: User ID, used to associate the request with user information.
 * ## `options`
 * - **Type**: &std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<Vec<alloc::string::String>, alloc::string::String>
 * - **Description**: On success, returns a list of strings containing the chapter content; on failure, returns an error message, including but not limited to HTTP request failures or JSON parsing errors.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - The `serde_json` library is used for JSON data serialization and deserialization.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match chapter_content(1, 1, &options).await {
 *     Ok(content) => println!("Chapter content: {:?}", content),
 *     Err(_) => println!("Chapter content retrieval failed"),
 * }
 * ```
 */
pub(crate) async fn chapter_content(
    chapter_id: u64,
    user_id: u64,
    options: &HashMap<String, String>,
) -> Result<Vec<String>, String> {
    let data = json!({
        "chapterId": chapter_id,
        "userId": user_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();
    let response_text = post_client("/api/h5/getChapterContent", &data, options).await?;
    let item_info: ItemInfo = serde_json::from_str(&response_text)
        .map_err(|err| format!("从Json解析SnapshotInfo失败: {}", err))?;
    Ok(item_info.content)
}
/**
 * @locale zh-CN
 * # `pay_chapter` 异步函数
 * - **全限定名称**：`crate::tasks::action::pay_chapter`
 * - **功能**：根据用户ID、漫画ID和章节ID支付章节费用，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括用户ID、漫画ID、章节ID和时间戳。此函数用于处理用户购买漫画章节的支付流程。
 *
 * # 参数
 * ## `user_id`
 * - **类型**：std::u64
 * - **描述**：支付章节费用的用户ID。
 * ## `comic_id`
 * - **类型**：std::u64
 * - **描述**：漫画ID，标识用户购买的漫画。
 * ## `chapter_id`
 * - **类型**：std::u64
 * - **描述**：章节ID，标识用户购买的具体章节。
 * ## `options`
 * - **类型**：&std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<(), alloc::string::String>
 * - **说明**：成功时返回无值结果；失败则返回错误信息，通常表示HTTP请求失败或服务器响应中包含的错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 成功支付章节后，不返回任何数据，仅确认操作是否成功。如果失败，会返回具体的错误信息。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match pay_chapter(1, 1, 1, &options).await {
 *     Ok(_) => println!("章节支付成功"),
 *     Err(_) => println!("章节支付失败"),
 * }
 * ```
 *
 * @locale en-US
 * # `pay_chapter` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::pay_chapter`
 * - **Function**: Processes the payment for a chapter based on the user ID, comic ID, and chapter ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the user ID, comic ID, chapter ID, and timestamp. This function is used to handle the payment process for purchasing comic chapters.
 *
 * # Parameters
 * ## `user_id`
 * - **Type**: std::u64
 * - **Description**: The user ID paying for the chapter.
 * ## `comic_id`
 * - **Type**: std::u64
 * - **Description**: The comic ID, identifying the comic being purchased by the user.
 * ## `chapter_id`
 * - **Type**: std::u64
 * - **Description**: The chapter ID, identifying the specific chapter being purchased by the user.
 * ## `options`
 * - **Type**: &std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<(), alloc::string::String>
 * - **Description**: On success, returns a unit value; on failure, returns an error message, typically indicating an HTTP request failure or an error contained in the server's response.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - After successfully paying for the chapter, no data is returned; only confirmation of whether the operation was successful. If it fails, a specific error message is returned.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match pay_chapter(1, 1, 1, &options).await {
 *     Ok(_) => println!("Chapter payment succeeded"),
 *     Err(_) => println!("Chapter payment failed"),
 * }
 * ```
 */
pub(crate) async fn pay_chapter(
    user_id: u64,
    comic_id: u64,
    chapter_id: u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": user_id,
        "comicId": comic_id,
        "chapterId": chapter_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();
    let response_text = post_client("/api/user/coinPay", &data, options).await?;
    Ok(())
}
/**
 * @locale zh-CN
 * # `daily_sign` 异步函数
 * - **全限定名称**：`crate::tasks::action::daily_sign`
 * - **功能**：根据用户ID执行每日签到操作，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括用户ID和时间戳。此函数用于处理用户的每日签到流程。
 *
 * # 参数
 * ## `user_id`
 * - **类型**：std::u64
 * - **描述**：执行签到的用户ID。
 * ## `options`
 * - **类型**：&std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<(), alloc::string::String>
 * - **说明**：成功时返回无值结果；失败则返回错误信息，通常表示HTTP请求失败或服务器响应中包含的错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 成功执行签到后，不返回任何数据，仅确认操作是否成功。如果失败，会返回具体的错误信息。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match daily_sign(1, &options).await {
 *     Ok(_) => println!("签到成功"),
 *     Err(_) => println!("签到失败"),
 * }
 * ```
 *
 * @locale en-US
 * # `daily_sign` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::daily_sign`
 * - **Function**: Performs a daily sign-in operation based on the user ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the user ID and timestamp. This function is used to handle the daily sign-in process for users.
 *
 * # Parameters
 * ## `user_id`
 * - **Type**: std::u64
 * - **Description**: The user ID performing the sign-in.
 * ## `options`
 * - **Type**: &std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<(), alloc::string::String>
 * - **Description**: On success, returns a unit value; on failure, returns an error message, typically indicating an HTTP request failure or an error contained in the server's response.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - After successfully executing the sign-in, no data is returned; only confirmation of whether the operation was successful. If it fails, a specific error message is returned.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match daily_sign(1, &options).await {
 *     Ok(_) => println!("Daily sign-in succeeded"),
 *     Err(_) => println!("Daily sign-in failed"),
 * }
 * ```
 */
pub(crate) async fn daily_sign(
    user_id: u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": user_id,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/user/checkSign", &data, options).await?;
    Ok(())
}
/**
 * @locale zh-CN
 * # `daily_work` 异步函数
 * - **全限定名称**：`crate::tasks::action::daily_work`
 * - **功能**：根据任务编号和用户ID领取每日任务奖励，通过调用`post_client`函数向服务器发起异步POST请求。请求数据包括用户ID、任务编号和时间戳。此函数用于处理用户的每日任务奖励领取流程。
 *
 * # 参数
 * ## `task_no`
 * - **类型**：std::u8
 * - **描述**：任务编号，标识用户完成的具体任务。
 * ## `user_id`
 * - **类型**：std::u64
 * - **描述**：领取奖励的用户ID。
 * ## `options`
 * - **类型**：&std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **描述**：配置选项，包括但不限于`origin`（请求基地址）、`app_id`（应用ID）等。这些选项用于`post_client`函数的调用。
 *
 * # 返回
 * - **类型**：core::result::Result<(), alloc::string::String>
 * - **说明**：成功时返回无值结果；失败则返回错误信息，通常表示HTTP请求失败或服务器响应中包含的错误。
 *
 * # 注意
 * - 本函数依赖于`post_client`函数，该函数负责发送异步POST请求和处理响应数据。
 * - `options`参数中必须包含所有必要的配置选项，否则将导致错误。
 * - 成功领取奖励后，不返回任何数据，仅确认操作是否成功。如果失败，会返回具体的错误信息。
 *
 * # 示例
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match daily_work(1, 1, &options).await {
 *     Ok(_) => println!("任务奖励领取成功"),
 *     Err(_) => println!("任务奖励领取失败"),
 * }
 * ```
 *
 * @locale en-US
 * # `daily_work` Async Function
 * - **Full Qualified Name**: `crate::tasks::action::daily_work`
 * - **Function**: Claims the daily task reward based on the task number and user ID by invoking the `post_client` function to asynchronously send a POST request to the server. Request data includes the user ID, task number, and timestamp. This function is used to handle the process of claiming daily task rewards for users.
 *
 * # Parameters
 * ## `task_no`
 * - **Type**: std::u8
 * - **Description**: The task number, identifying the specific task completed by the user.
 * ## `user_id`
 * - **Type**: std::u64
 * - **Description**: The user ID claiming the reward.
 * ## `options`
 * - **Type**: &std::collections::hash_map::HashMap<alloc::string::String, alloc::string::String>
 * - **Description**: Configuration options including, but not limited to, `origin` (request base URL), `app_id` (application ID), etc. These options are used for the call to the `post_client` function.
 *
 * # Returns
 * - **Type**: core::result::Result<(), alloc::string::String>
 * - **Description**: On success, returns a unit value; on failure, returns an error message, typically indicating an HTTP request failure or an error contained in the server's response.
 *
 * # Note
 * - This function depends on the `post_client` function, responsible for sending asynchronous POST requests and handling response data.
 * - All necessary configuration options must be included in the `options` parameter, otherwise, errors will occur.
 * - After successfully claiming the reward, no data is returned; only confirmation of whether the operation was successful. If it fails, a specific error message is returned.
 *
 * # Examples
 * ```rust
 * let options = HashMap::from([
 *     ("dev_type".to_string(), "mobile".to_string()),
 *     ("origin".to_string(), "https://example.com".to_string()),
 * ]);
 * match daily_work(1, 1, &options).await {
 *     Ok(_) => println!("Daily work reward claimed successfully"),
 *     Err(_) => println!("Failed to claim daily work reward"),
 * }
 * ```
 */
pub(crate) async fn daily_work(
    task_no: u8,
    user_id: u64,
    options: &HashMap<String, String>,
) -> Result<(), String> {
    let data = json!({
        "userId": user_id,
        "taskNo": task_no,
        "timeStamp": timestamp_str()?
    })
    .to_string();

    let response_text = post_client("/api/user/getTaskReward", &data, options).await?;
    Ok(())
}
