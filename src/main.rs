use sqlx::mysql::MySqlConnectOptions;
use sqlx::{Column, Row, TypeInfo};

mod core;
mod dbm;

#[tokio::main]
async fn main() {
    // let spider = core::spider::Spider::load("src/spider.json".to_string()).unwrap();
    // println!("{:?}", spider);
    // spider.run().await.expect("err");
    // let mut ms_source = core::dbm::Datasource::new(
    //     "".to_string(),
    //     "阿里云RDS-Mysql".to_string(),
    //     "mysql".to_string(),
    //     "diamater.mysql.rds.aliyuncs.com".to_string(),
    //     3306,
    //     "spider".to_string(),
    //     "S112358r".to_string(),
    //     "db_spider".to_string(),
    //     None,
    // );
    // // let ms_connection = ms_source.connect_mysql().await.unwrap();
    // let ms_connection = ms_source.connect().await.unwrap();
    // ms_connection.create_table().await.unwrap();
    let option = MySqlConnectOptions::new()
        .host("diamater.mysql.rds.aliyuncs.com")
        .port(3306)
        .database("db_spider")
        .username("spider_ddl")
        .password("S112358r@d");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect_with(option)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE `test1`(`id1` INT);")
        .execute(&pool)
        .await
        .unwrap();
}
