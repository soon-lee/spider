/**
 * @locale zh-CN
 * # 获取当前时间戳（单位：秒）的字符串表示。
 * - **全限定名称**： `crate::tasks::action::timestamp_str`
 * - **功能**：利用系统时间API获取当前时间，并将其转换为自UNIX纪元以来的秒数。
 *
 * # 返回
 * - **类型**：`alloc::string::String`
 * - **描述**：返回当前时间戳的字符串形式。
 *
 * # 示例
 *
 * ```rust
 * use crate::tasks::action::timestamp_str;
 * let timestamp = timestamp_str();
 * println!("当前时间戳为: {}", timestamp);
 * ```
 *
 * @locale en-US
 * # Retrieves the current timestamp in seconds as a string representation.
 * - **Fully Qualified Name**: `crate::tasks::action::timestamp_str`
 * - **Functionality**: Fetches the current time using system time API and converts it to seconds since the UNIX epoch.
 *
 * # Returns
 * - **Type**: `alloc::string::String`
 * - **Value**: A string representing the current timestamp.
 *
 * # Example
 *
 * ```rust
 * use crate::tasks::action::timestamp_str;
 * let timestamp = timestamp_str();
 * println!("The current timestamp is: {}", timestamp);
 * ```
 */
pub(crate) fn timestamp_str() -> String {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap()
        / 1000)
        .to_string()
}