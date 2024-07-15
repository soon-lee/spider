use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::{Rng, thread_rng};
use soft_aes::aes::{aes_dec_cbc, aes_dec_ecb, aes_enc_cbc, aes_enc_ecb};

use crate::utils::datetime::timestamp_str;

/**
 * @locale zh-CN
 * # `aes_encrypt` 函数
 * - **全限定名称**：`crate::tasks::utils::aes_encrypt`
 * - **功能**：使用AES-ECB模式和PKCS7填充对给定的`data`进行加密，并将加密结果转换为Base64编码的字符串。
 *
 * # 参数
 * ## `key`
 * - **类型**：&std::str
 * - **描述**：加密密钥。
 * ## `data`
 * - **类型**：&std::str
 * - **描述**：需要加密的数据。
 *
 * # 返回
 * - **类型**：`core::result::Result<alloc::string::String, alloc::string::String>`
 * - **说明**：成功时返回Base64编码的加密数据；失败则返回错误信息，说明加密过程中遇到的问题。
 *
 * # 注意
 * - 确保`key`长度满足AES算法要求（如128位、192位或256位）。
 *
 * # 示例
 * ```rust
 * let key = "my_secret_key";
 * let data = "my_secret_data";
 * match aes_encrypt(key, data) {
 *     Ok(encrypted_data) => println!("加密后文本: {}", encrypted_data),
 *     Err(err) => println!("异常: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `aes_encrypt` Function
 * - **Full Qualified Name**: `crate::tasks::utils::aes_encrypt`
 * - **Function**: Encrypts the given `data` using AES-ECB mode with PKCS7 padding, and converts the encrypted result into a Base64-encoded string.
 *
 * # Parameters
 * ## `key`
 * - **Type**: &std::str
 * - **Description**: The encryption key.
 * ## `data`
 * - **Type**: &std::str
 * - **Description**: The data to be encrypted.
 *
 * # Returns
 * - **Type**: `core::result::Result<alloc::string::String, alloc::string::String>`
 * - **Description**: On success, returns the encrypted data encoded in Base64; on failure, returns an error message describing the issue encountered during encryption.
 *
 * # Note
 * - Ensure the `key` length meets the AES algorithm requirements (e.g., 128-bit, 192-bit, or 256-bit).
 *
 * # Examples
 * ```rust
 * let key = "my_secret_key";
 * let data = "my_secret_data";
 * match aes_encrypt(key, data) {
 *     Ok(encrypted_data) => println!("Encrypted data: {}", encrypted_data),
 *     Err(err) => println!("Error: {}", err),
 * }
 */
pub(crate) fn aes_encrypt(key: &str, data: &str) -> Result<String, String> {
    let bytes = aes_enc_ecb(data.as_bytes(), key.as_bytes(), Some("PKCS7"))
        .map_err(|err| format!("AES加密失败：{}", err))?;
    Ok(STANDARD.encode(&bytes))
}
/**
 * @locale zh-CN
 * # `aes_decrypt` 函数
 * - **全限定名称**：`crate::utils::aes_decrypt`
 * - **功能**：对给定的Base64编码的加密数据进行AES-ECB解密，并采用PKCS7填充方式。解密后，将字节数据转换为UTF-8编码的字符串。
 *
 * # 参数
 * ## `key`
 * - **类型**：&std::str
 * - **描述**：用于解密的密钥。
 * ## `data`
 * - **类型**：&std::str
 * - **描述**：Base64编码的加密数据。
 *
 * # 返回
 * - **类型**：`core::result::Result<alloc::string::String, alloc::string::String>`
 * - **说明**：成功时返回解密后的字符串；失败则返回错误信息，说明解密或转换过程中遇到的问题。
 *
 * # 注意
 * - 确保使用与加密时相同的密钥和填充模式。
 * - 输入数据必须是有效的Base64编码且能成功解码为AES加密的字节流。
 *
 * # 示例
 * ```rust
 * let key = "my_secret_key";
 * let encrypted_data = "your_base64_encoded_ciphertext";
 * match aes_decrypt(key, encrypted_data) {
 *     Ok(decrypted_text) => println!("解密后的文本: {}", decrypted_text),
 *     Err(err) => println!("异常: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `aes_decrypt` Function
 * - **Full Qualified Name**: `crate::utils::aes_decrypt`
 * - **Function**: Decrypts the given Base64-encoded encrypted data using AES-ECB mode with PKCS7 padding. After decryption, the byte data is converted into a UTF-8 encoded string.
 *
 * # Parameters
 * ## `key`
 * - **Type**: &std::str
 * - **Description**: The key used for decryption.
 * ## `data`
 * - **Type**: &std::str
 * - **Description**: The Base64-encoded encrypted data.
 *
 * # Returns
 * - **Type**: `core::result::Result<alloc::string::String, alloc::string::String>`
 * - **Description**: On success, returns the decrypted string; on failure, returns an error message describing the issue encountered during decryption or conversion.
 *
 * # Note
 * - Ensure the same key and padding mode as used during encryption.
 * - The input data must be a valid Base64 encoding that can be successfully decoded into an AES-encrypted byte stream.
 *
 * # Examples
 * ```rust
 * let key = "my_secret_key";
 * let encrypted_data = "your_base64_encoded_ciphertext";
 * match aes_decrypt(key, encrypted_data) {
 *     Ok(decrypted_text) => println!("Decrypted text: {}", decrypted_text),
 *     Err(err) => println!("Error: {}", err),
 * }
 * ```
 */
pub(crate) fn aes_decrypt(key: &str, data: &str) -> Result<String, String> {
    let bytes = STANDARD
        .decode(data)
        .map_err(|err| format!("Base64解码失败：{}", err))?;
    let bytes = aes_dec_ecb(&*bytes, key.as_bytes(), Some("PKCS7"))
        .map_err(|err| format!("AES解密失败：{}", err))?;
    let text = String::from_utf8(bytes).map_err(|err| format!("UTF-8解码失败：{}", err))?;
    Ok(text)
}
/**
 * @locale zh-CN
 * # `random_str` 函数
 * - **全限定名称**：`crate::utils::random_str`
 * - **功能**：根据提供的模板字符串，生成一个新的字符串。模板中的每个 `'x'` 字符将被替换为一个随机的十六进制数，每个 `'y'` 字符将被替换为一个随机的十六进制数（限制在8到F之间），其余字符保持不变。
 *
 * # 参数
 * ## `template`
 * - **类型**：&std::str
 * - **描述**：用作生成随机字符串基础的模板字符串，其中 `'x'` 和 `'y'` 作为特殊占位符。
 *
 * # 返回
 * - **类型**：alloc::string::String
 * - **说明**：根据模板生成的随机字符串。
 *
 * # 注意
 * - 本函数利用线程本地的随机数生成器，确保并发安全。
 * - 模板中 `'x'` 和 `'y'` 的数量将直接影响生成字符串的长度和内容。
 *
 * # 示例
 * ```rust
 * let template = "Exa_yple_xxx_yyy";
 * let random_text = random_str(template);
 * println!("生成的随机字符串: {}", random_text);
 * ```
 *
 * @locale en-US
 * # `random_str` Function
 * - **Full Qualified Name**: `crate::tasks::utils::random_str`
 * - **Function**: Generates a new string based on the provided template. Each `'x'` in the template is replaced with a random hexadecimal digit, and each `'y'` is replaced with a random hexadecimal digit (restricted to the range 8 to F). Other characters are left unchanged.
 *
 * # Parameters
 * ## `template`
 * - **Type**: &std::str
 * - **Description**: The template string used as the basis for generating the random string, with `'x'` and `'y'` serving as placeholders.
 *
 * # Returns
 * - **Type**: alloc::string::String
 * - **Description**: The random string generated according to the template.
 *
 * # Note
 * - This function utilizes a thread-local random number generator to ensure thread safety.
 * - The number of `'x'` and `'y'` in the template directly influences the length and content of the generated string.
 *
 * # Examples
 * ```rust
 * let template = "Exa_yple_xxx_yyy";
 * let random_text = random_str(template);
 * println!("Generated random string: {}", random_text);
 * ```
 */
pub(crate) fn random_str(template: &str) -> String {
    template
        .chars()
        .map(|c| {
            let mut rander = thread_rng();
            let n = rander.gen_range(0..16);
            match c {
                'x' => format!("{:x}", n),
                'y' => format!("{:x}", (n & 0x3 | 0x8)),
                _ => c.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
/**
 * @locale zh-CN
 * # `fill_path` 函数
 * - **全限定名称**：`crate::utils::fill_path`
 * - **功能**：补充路径字符串，确保其以`/api`开头并以`/`分隔。如果输入路径不以`/`开始，则在路径前添加；之后，在所有路径之前插入`/api`前缀。
 *
 * # 参数
 * ## `path`
 * - **类型**：alloc::string::String
 * - **描述**：需要补充的原始路径字符串。
 *
 * # 返回
 * - **类型**：alloc::string::String
 * - **说明**：补充后的路径，格式化为以`/api`开头，并确保第一个字符为`/`。
 *
 * # 示例
 * ```rust
 * let original_path = "test/path";
 * let filled_path = fill_path(original_path.to_string());
 * println!("补充后的路径: {}", filled_path);
 * ```
 *
 * @locale en-US
 * # `fill_path` Function
 * - **Full Qualified Name**: `crate::utils::fill_path`
 * - **Function**: Augments the path string to ensure it starts with `/api` and is properly separated by `/`. If the input path does not start with `/`, one is prepended; afterward, the `/api` prefix is inserted before the rest of the path.
 *
 * # Parameters
 * ## `path`
 * - **Type**: alloc::string::String
 * - **Description**: The original path string that needs augmentation.
 *
 * # Returns
 * - **Type**: alloc::string::String
 * - **Description**: The augmented path, formatted to start with `/api` and ensuring the first character is `/`.
 *
 * # Examples
 * ```rust
 * let original_path = "test/path";
 * let filled_path = fill_path(original_path.to_string());
 * println!("Augmented path: {}", filled_path);
 * ```
 */
pub(crate) fn fill_path(path: String) -> String {
    let mut result = path.clone();
    if !path.starts_with("/") {
        result.insert(0, '/');
    }
    result.insert_str(0, "/api");
    result
}
/**
 * @locale zh-CN
 * # `path_hash` 函数
 * - **全限定名称**：`crate::utils::path_hash`
 * - **功能**：为给定的路径生成一个哈希字符串，该哈希结合了路径、时间戳、基于模板的随机字符串以及默认值。首先，构造一个格式化的字符串，然后计算其MD5哈希值作为输出。
 *
 * # 参数
 * ## `path`
 * - **类型**：&std::str
 * - **描述**：需要参与哈希计算的路径部分。
 * ## `template`
 * - **类型**：&std::str
 * - **描述**：用于生成随机字符串的模板，传递给`random_str`函数。
 * ## `default`
 * - **类型**：&std::str
 * - **描述**：默认值字符串，作为哈希计算的一部分。
 * ## `timestamp`
 * - **类型**：&std::str
 * - **描述**：时间戳字符串，确保哈希的独特性。
 *
 * # 返回
 * - **类型**：alloc::string::String
 * - **说明**：基于输入信息计算得到的MD5哈希值的十六进制表示。
 *
 * # 注意
 * - 本函数使用MD5算法生成哈希，适用于简单标识用途。对于安全性要求较高的场景，请考虑更安全的哈希算法。
 *
 * # 示例
 * ```rust
 * let path = "example/path";
 * let template = "xy";
 * let default_val = "default";
 * let time_stamp = "20230401";
 * let hash_result = path_hash(path, template, default_val, time_stamp);
 * println!("路径哈希: {}", hash_result);
 * ```
 *
 * @locale en-US
 * # `path_hash` Function
 * - **Full Qualified Name**: `crate::utils::path_hash`
 * - **Function**: Generates a hash string for a given path, which combines the path, timestamp, a random string based on a template, and a default value. A formatted string is constructed first, and then its MD5 hash value is computed as the output.
 *
 * # Parameters
 * ## `path`
 * - **Type**: &std::str
 * - **Description**: The path component that participates in the hash calculation.
 * ## `template`
 * - **Type**: &std::str
 * - **Description**: The template used to generate a random string, passed to the `random_str` function.
 * ## `default`
 * - **Type**: &std::str
 * - **Description**: The default value string, included as part of the hash calculation.
 * ## `timestamp`
 * - **Type**: &std::str
 * - **Description**: The timestamp string, ensuring the uniqueness of the hash.
 *
 * # Returns
 * - **Type**: alloc::string::String
 * - **Description**: The hexadecimal representation of the MD5 hash value computed from the input information.
 *
 * # Note
 * - This function uses the MD5 algorithm for hashing, suitable for simple identification purposes. For scenarios requiring higher security, consider more secure hash algorithms.
 *
 * # Examples
 * ```rust
 * let path = "example/path";
 * let template = "xy";
 * let default_val = "default";
 * let time_stamp = "20230401";
 * let hash_result = path_hash(path, template, default_val, time_stamp);
 * println!("Path Hash: {}", hash_result);
 * ```
 */
pub(crate) fn path_hash(path: &str, template: &str, default: &str, timestamp: &str) -> String {
    let text = format!(
        "{}-{}-{}-0-{}",
        path,
        timestamp,
        random_str(template),
        default
    );
    format!("{:x}", md5::compute(text))
}
/**
 * @locale zh-CN
 * # `auth_path` 函数
 * - **全限定名称**：`crate::utils::auth_path`
 * - **功能**：为指定路径生成授权字符串，增强路径以包含时间戳、随机字符串以及基于这些值计算的哈希，以便进行鉴权。根据原路径是否包含查询参数，选择使用`&`或`?`连接新增的鉴权参数。
 *
 * # 参数
 * ## `path`
 * - **类型**：&str
 * - **描述**：需要增加鉴权信息的基础路径。
 * ## `template`
 * - **类型**：&str
 * - **描述**：用于生成随机字符串的模板，传递给`random_str`函数。
 * ## `default`
 * - **类型**：&str
 * - **描述**：默认值字符串，用于构建哈希的一部分。
 *
 * # 返回
 * - **类型**：core::result::Result<alloc::string::String, alloc::string::String>
 * - **说明**：成功时返回带有鉴权参数的完整路径；失败则返回错误信息，通常与时间戳生成失败相关。
 *
 * # 注意
 * - 该函数首先确定路径中是否已有查询参数，以决定如何附加新的鉴权参数。
 * - 使用`timestamp_str`函数获取当前时间戳，可能抛出错误。
 * - 生成的鉴权字符串包括时间戳、随机字符串以及它们的MD5哈希值，增强路径的安全性。
 *
 * # 示例
 * ```rust
 * let path = "/example/path";
 * let template = "xy";
 * let default_val = "optional_default";
 * match auth_path(path, template, default_val) {
 *     Ok(authenticated_path) => println!("授权路径: {}", authenticated_path),
 *     Err(err) => println!("异常: {}", err),
 * }
 * ```
 *
 * @locale en-US
 * # `auth_path` Function
 * - **Full Qualified Name**: `crate::utils::auth_path`
 * - **Function**: Generates an authorization string for a specified path, enhancing the path with a timestamp, a random string, and a hash computed from these values for authentication purposes. Chooses between `&` and `?` to concatenate the new authentication parameters based on whether the original path contains query parameters.
 *
 * # Parameters
 * ## `path`
 * - **Type**: &str
 * - **Description**: The base path that requires additional authentication information.
 * ## `template`
 * - **Type**: &str
 * - **Description**: The template used to generate a random string, passed to the `random_str` function.
 * ## `default`
 * - **Type**: &str
 * - **Description**: The default value string, used as part of the hash construction.
 *
 * # Returns
 * - **Type**: core::result::Result<alloc::string::String, alloc::string::String>
 * - **Description**: On success, returns the complete path with authentication parameters; on failure, returns an error message, typically related to timestamp generation failure.
 *
 * # Note
 * - The function first determines if the path already has query parameters to decide how to append the new authentication parameters.
 * - Utilizes the `timestamp_str` function to obtain the current timestamp, which may fail.
 * - The authentication string includes a timestamp, a random string, and their MD5 hash, enhancing the path's security.
 *
 * # Examples
 * ```rust
 * let path = "/example/path";
 * let template = "xy";
 * let default_val = "optional_default";
 * match auth_path(path, template, default_val) {
 *     Ok(authenticated_path) => println!("Authorized Path: {}", authenticated_path),
 *     Err(err) => println!("Error: {}", err),
 * }
 * ```
 */
pub(crate) fn auth_path(path: &str, template: &str, default: &str) -> Result<String, String> {
    let concat = match path.contains("?") {
        true => "&",
        false => "?",
    };
    let timestamp_10 = timestamp_str()?;
    let random_str = random_str(template);
    let hash = path_hash(path, template, default, &*timestamp_10);
    let path = format!(
        "{}{}cpt_auth={}-{}-0-{}",
        path.clone(),
        concat,
        timestamp_10,
        random_str,
        hash
    );
    Ok(path)
}

pub(crate) fn encrypt_bytes(input: &[u8], key: &str, iv: &str) -> Result<Vec<u8>, String> {
    let iv = md5::compute(iv).0;
    aes_enc_cbc(input, key.as_bytes(), &iv, Some("PKCS7")).map_err(|err| format!("AES加密失败：{}", err))
}
pub(crate) fn decrypt_bytes(input: &[u8], key: &str, iv: &str) -> Result<Vec<u8>, String> {
    let iv = md5::compute(iv).0;
    aes_dec_cbc(input, key.as_bytes(), &iv, Some("PKCS7")).map_err(|err| format!("AES解密失败：{}", err))
}