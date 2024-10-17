use crate::dbm::core::{DataField, DataIndex, DataRow, DataSource, DataTable};
use serde_json::Value;
use sqlx::{query, MySqlPool};
use tokio::sync::Mutex;

pub struct MySqlDDLClient {
    pool: Mutex<MySqlPool>,
}
impl MySqlDDLClient {
    pub async fn create(datasource: DataSource) -> Result<Self, String> {
        let pool = MySqlPool::connect_with(datasource.ddl_pool_connect_options())
            .await
            .map_err(|e| format!("DDL数据连接池创建失败：{}", e))?;
        Ok(Self {
            pool: Mutex::new(pool),
        })
    }
    pub async fn create_table(&self, table: &DataTable) -> Result<(), String> {
        let sql = table.sql_create_table();
        let pool = self.pool.lock().await;
        query(sql.as_str())
            .execute(&*pool)
            .await
            .map_err(|e| format!("创建表{}失败：{}", table.name(), e))?;
        Ok(())
    }
    pub async fn alter_table(
        &self,
        table: &DataTable,
        items: &Vec<(String, DataField, Option<String>)>,
    ) -> Result<(), String> {
        let sql = table.sql_alter_table(items);
        let pool = self.pool.lock().await;
        query(sql.as_str())
            .execute(&*pool)
            .await
            .map_err(|e| format!("修改表{}失败：{}", table.name(), e))?;
        Ok(())
    }
    pub async fn drop_table(&self, table: &DataTable) -> Result<(), String> {
        let sql = table.sql_drop_table();
        let pool = self.pool.lock().await;
        query(sql.as_str())
            .execute(&*pool)
            .await
            .map_err(|e| format!("删除表{}失败：{}", table.name(), e))?;
        Ok(())
    }
    pub async fn create_index(
        &self,
        table: &DataTable,
        indexes: &Vec<DataIndex>,
    ) -> Result<(), String> {
        let sql = table.sql_create_index(indexes);
        let pool = self.pool.lock().await;
        query(sql.as_str())
            .execute(&*pool)
            .await
            .map_err(|e| format!("创建索引失败：{}", e))?;
        Ok(())
    }
    pub async fn drop_index(&self, table: &DataTable, indexes: &Vec<String>) -> Result<(), String> {
        let sql = table.sql_drop_index(indexes);
        let pool = self.pool.lock().await;
        query(sql.as_str())
            .execute(&*pool)
            .await
            .map_err(|e| format!("删除索引失败：{}", e))?;
        Ok(())
    }
}
pub struct MySqlDMLClient {
    pool: Mutex<MySqlPool>,
}
impl MySqlDMLClient {
    pub async fn create(datasource: DataSource) -> Result<Self, String> {
        let pool = MySqlPool::connect_with(datasource.dml_pool_connect_options())
            .await
            .map_err(|e| format!("DML数据连接池创建失败：{}", e))?;
        Ok(Self {
            pool: Mutex::new(pool),
        })
    }
    pub async fn insert(&self, table: &DataTable, row: &DataRow) -> Result<(), String> {
        let pool = self.pool.lock().await;
        query("")
            .bind("")
            .execute(&*pool)
            .await
            .map_err(|e| format!("插入数据失败：{}", e))?;
        Ok(())
    }
}
pub struct MySqlDQLClient {
    pub(crate) pool: MySqlPool,
}
impl MySqlDQLClient {
    pub async fn create(datasource: DataSource) -> Result<Self, String> {
        let pool = MySqlPool::connect_with(datasource.dql_pool_connect_options())
            .await
            .map_err(|e| format!("DQL数据连接池创建失败：{}", e))?;
        Ok(Self { pool })
    }
    pub async fn execute(&self, sql: &str) -> Result<Value, String> {
        let rows = query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("{}", e))?;
        let mut values = vec![];
        for row in rows.iter() {
            let value = DataRow::from(row)?;
            values.push(value.value().clone());
        }
        Ok(Value::Array(values))
    }
}
