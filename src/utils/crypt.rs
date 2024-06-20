use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

fn random_str() -> String {
    let template = "xxxxxxxxxxxx4xxxyxxxxxxxxxxxxxxx";
    template.chars().map(|c| {
        let mut rander = rand::thread_rng();
        let n = rander.gen_range(0..16);
        match c {
            'x' => format!("{:x}", n),
            'y' => format!("{:x}", (n & 0x3 | 0x8)),
            _ => c.to_string()
        }
    }).collect::<Vec<_>>().join("")
}

fn fill_path(path: String) -> String {
    let mut result = path.clone();
    if !path.starts_with("/") {
        result.insert(0, '/');
    }
    result.insert_str(0, "/api");
    result
}

fn path_hash(path: String, timestamp_10: u128) ->String {
    let text = format!("{}-{}-{}-0-mq9hyJjPq4Au5gQfGsM", path.clone(), timestamp_10, random_str());
    format!("{:x}", md5::compute(text))
}

pub(crate) fn auth_path(path: String) -> String {
    let concat = match path.contains("?") {
        true => "&",
        false => "?"
    };
    let timestamp_10 = SystemTime::now().duration_since(UNIX_EPOCH).map(
        |duration| duration.as_millis()
    ).unwrap() / 1000;
    let random_str = random_str();
    let hash = path_hash(path.clone(), timestamp_10);
    format!("{}{}cpt_auth={}-{}-0-{}", path.clone(), concat, timestamp_10, random_str, hash)
}
