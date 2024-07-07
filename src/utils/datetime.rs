use std::time::{SystemTime, UNIX_EPOCH};

/**
 * @locale zh-CN
 * # 获取当前时间戳（单位：秒）的字符串表示。
 * - **全限定名称**： `crate::tasks::action::timestamp_str`
 * - **功能**：此函数利用系统时间API获取当前时间，并直接转换为自UNIX纪元以来的秒数，最后以字符串形式返回。
 * - **改进**：现已优化为直接获取秒数，提升效率。
 *
 * # 返回
 * - **类型**：`Result<alloc::string::String, String>`
 * - **说明**：成功时返回当前时间戳的秒数值转换的字符串；若操作失败，则返回描述错误的字符串。
 *
 * # 示例
 * ```rust
 * use crate::tasks::action::timestamp_str;
 * match timestamp_str() {
 *     Ok(timestamp) => println!("当前时间戳为: {}", timestamp),
 *     Err(err) => eprintln!("获取时间戳时遇到错误: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # Retrieves the current timestamp in seconds as a string representation.
 * - **Fully Qualified Name**: `crate::tasks::action::timestamp_str`
 * - **Functionality**: This function fetches the current time using the system time API, directly converting it to seconds since the UNIX epoch, and returns it as a string.
 * - **Enhancement**: Optimized to fetch seconds directly, improving efficiency.
 *
 * # Returns
 * - **Type**: `Result<alloc::string::String, String>`
 * - **Description**: On success, returns a string of the current timestamp in seconds; on failure, returns a descriptive error message.
 *
 * # Example
 * ```rust
 * use crate::tasks::action::timestamp_str;
 * match timestamp_str() {
 *     Ok(timestamp) => println!("The current timestamp is: {}", timestamp),
 *     Err(err) => eprintln!("An error occurred while getting the timestamp: {}", err),
 * }
 * ```
 */
pub(crate) fn timestamp_str() -> Result<String, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .map_err(|err| format!("获取时间戳失败: {}", err))
}