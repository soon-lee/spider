use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::{Rng, thread_rng};
use soft_aes::aes::aes_enc_ecb;

pub(crate) fn aes_encrypt(key: &str, data: &str) -> String {
    let bytes = aes_enc_ecb(data.as_bytes(), key.as_bytes(), Some("PKCS7")).unwrap();
    STANDARD.encode(&bytes)
}
pub(crate) fn aes_decrypt(key: &str, data: &str) -> String {
    let bytes = STANDARD.decode(data).unwrap();
    let bytes = soft_aes::aes::aes_dec_ecb(&*bytes, key.as_bytes(), Some("PKCS7")).unwrap();
    String::from_utf8(bytes).unwrap()
}
pub(crate) fn random_str(template:&str) -> String {
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
pub(crate) fn fill_path(path: &str) -> String {
    let mut result = path.clone();
    if !path.starts_with("/") {
        result.insert(0, '/');
    }
    result.insert_str(0, "/api");
    result
}
pub(crate) fn path_hash(path: &str,template:&str, timestamp: u128) -> String {
    let text = format!(
        "{}-{}-{}-0-{}",
        path.clone(),
        timestamp,
        random_str(template),
        template
    );
    format!("{:x}", md5::compute(text))
}
pub(crate) fn auth_path(path: &str,template:&str) -> String {
    let concat = match path.contains("?") {
        true => "&",
        false => "?",
    };
    let timestamp_10 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap()
        / 1000;
    let random_str = random_str(template);
    let hash = path_hash(path, template,timestamp_10);
    format!(
        "{}{}cpt_auth={}-{}-0-{}",
        path.clone(),
        concat,
        timestamp_10,
        random_str,
        hash
    )
}