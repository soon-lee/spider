use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use getset::Getters;
use serde_json::{Map, Number, Value};
use sqlx::mysql::{MySqlConnectOptions, MySqlRow};
use sqlx::{Column, Row, TypeInfo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataField {
    key: String,
    name: String,
    comment: Option<String>,
    primary: bool,
    nullable: bool,
    default: Option<String>,
}
impl DataField {
    pub fn new(
        key: String,
        name: String,
        comment: Option<String>,
        primary: bool,
        nullable: bool,
        default: Option<String>,
    ) -> Self {
        DataField {
            key,
            name,
            comment,
            primary,
            nullable,
            default,
        }
    }
    pub fn sql(&self) -> String {
        let mut items = Vec::<String>::new();
        items.push(format!("`{}`", self.name));
        if self.primary {
            items.push(String::from("PRIMARY KEY"));
        } else if !self.nullable {
            items.push(String::from("NOT NULL"));
        }
        if let Some(default) = &self.default {
            items.push(format!("DEFAULT {}", default));
        }
        if let Some(comment) = &self.comment {
            items.push(format!("COMMENT \"{}\"", comment));
        }
        items.join(" ")
    }
    pub fn alter(&self, operation: &String, new_name: &Option<String>) -> String {
        match operation.as_str() {
            "ADD" => format!("ADD {}", self.sql()),
            "MODIFY" => format!("MODIFY {}", self.sql()),
            "CHANGE" => format!(
                "CHANGE {}",
                self.sql().replace(
                    self.name.as_str(),
                    format!(
                        "{} {}",
                        self.name,
                        new_name.clone().unwrap_or(self.name.clone())
                    )
                    .as_str()
                )
            ),
            "DROP" => format!("DROP {}", self.name),
            _ => String::from(""),
        }
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataIndex {
    key: String,
    name: String,
    fields: Vec<String>,
    unique: bool,
}
impl DataIndex {
    pub fn new(key: String, name: String, fields: Vec<String>, unique: bool) -> Self {
        DataIndex {
            key,
            name,
            fields,
            unique,
        }
    }
    pub fn sql(&self) -> String {
        let mut items = Vec::<String>::new();
        if self.unique {
            items.push(String::from("UNIQUE"));
        }
        items.push(String::from("INDEX"));
        items.push(format!("`{}`", self.name));
        items.push(format!(
            "({})",
            self.fields
                .iter()
                .map(|field| format!("`{}`", field))
                .collect::<Vec<_>>()
                .join(",")
        ));
        items.join(" ")
    }
    pub fn alter(&self, operation: &String) -> String {
        match operation.as_str() {
            "ADD" => format!("ADD {}", self.sql()),
            "DROP" => format!("DROP {}", self.name),
            _ => String::from(""),
        }
    }
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataUser {
    name: String,
    password: String,
}
impl DataUser {
    pub fn new(name: String, password: String) -> Self {
        DataUser { name, password }
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataTable {
    key: String,
    name: String,
    comment: Option<String>,
    fields: Vec<DataField>,
    indexes: Option<Vec<DataIndex>>,
}
impl DataTable {
    pub fn new(
        key: String,
        name: String,
        comment: Option<String>,
        fields: Vec<DataField>,
        indexes: Option<Vec<DataIndex>>,
    ) -> Self {
        DataTable {
            key,
            name,
            comment,
            fields,
            indexes,
        }
    }
    pub fn sql_create_table(&self) -> String {
        let mut fields = self
            .fields
            .iter()
            .map(|field| field.sql())
            .collect::<Vec<_>>();
        if let Some(indexes) = &self.indexes {
            for index in indexes {
                fields.push(index.sql());
            }
        }
        format!(
            "DROP TABLE IF EXISTS `{}`;CREATE TABLE `{}` ({})",
            self.name,
            self.name,
            fields.join(",")
        )
    }
    pub fn sql_alter_table(&self, items: &Vec<(String, DataField, Option<String>)>) -> String {
        items
            .iter()
            .map(|(operation, field, new_name)| {
                format!(
                    "ALTER TABLE {} {} {}",
                    self.name,
                    operation,
                    field.alter(operation, new_name)
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    }
    pub fn sql_drop_table(&self) -> String {
        format!("DROP TABLE IF EXISTS `{}`", self.name)
    }
    pub fn sql_create_index(&self, indexes: &Vec<DataIndex>) -> String {
        indexes
            .iter()
            .map(|index| {
                format!(
                    "CREATE INDEX `{}` ON `{}` ({});",
                    index.name,
                    self.name,
                    index.fields.join(",")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn sql_drop_index(&self, indexes: &Vec<String>) -> String {
        indexes
            .iter()
            .map(|index| format!("DROP INDEX `{}` ON `{}`;", index, self.name))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn sql_insert_row(&self) -> String {
        format!(
            "INSERT INTO `{}` ({}) VALUE ({});",
            self.name,
            self.fields()
                .iter()
                .map(|field| format!("`{}`", field.name()))
                .collect::<Vec<_>>()
                .join(","),
            self.fields()
                .iter()
                .map(|_| String::from("?"))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    pub fn sql_update_row(&self) -> String {
        format!(
            "UPDATE `{}` SET {} WHERE id = ?;",
            self.name,
            self.fields()
                .iter()
                .map(|field| format!("`{}` = ?", field.name()))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    pub fn sql_delete_row(&self) -> String {
        format!("DELETE FROM `{}` WHERE id = ?;", self.name)
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataSource {
    key: String,
    name: String,
    kind: String,
    host: String,
    port: u16,
    database: String,
    tables: Vec<DataTable>,
    ddl_user: DataUser,
    dml_user: DataUser,
    dql_user: DataUser,
}
impl DataSource {
    pub fn new(
        key: String,
        name: String,
        kind: String,
        host: String,
        port: u16,
        database: String,
        tables: Vec<DataTable>,
        ddl_user: DataUser,
        dml_user: DataUser,
        dql_user: DataUser,
    ) -> Self {
        DataSource {
            key,
            name,
            kind,
            host,
            port,
            database,
            tables,
            ddl_user,
            dml_user,
            dql_user,
        }
    }
    fn pool_connect_options(&self, username: &String, password: &String) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.database)
            .username(username)
            .password(password)
    }
    pub fn ddl_pool_connect_options(&self) -> MySqlConnectOptions {
        self.pool_connect_options(&self.ddl_user.name, &self.ddl_user.password)
    }
    pub fn dml_pool_connect_options(&self) -> MySqlConnectOptions {
        self.pool_connect_options(&self.dml_user.name, &self.dml_user.password)
    }
    pub fn dql_pool_connect_options(&self) -> MySqlConnectOptions {
        self.pool_connect_options(&self.dql_user.name, &self.dql_user.password)
    }
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataRow {
    value: Value,
}
impl DataRow {
    pub fn from(row: &MySqlRow) -> Result<Self, String> {
        let mut map = Map::<String, Value>::new();
        for (index, column) in row.columns().iter().enumerate() {
            let value = match column.type_info().name() {
                "TINYINT" => Value::from(Number::from(row.get::<i8, _>(index))),
                "SMALLINT" => Value::from(Number::from(row.get::<i16, _>(index))),
                "MEDIUMINT" | "INT" | "INTEGER" => {
                    Value::from(Number::from(row.get::<i32, _>(index)))
                }
                "BIGINT" => Value::from(Number::from(row.get::<i64, _>(index))),
                "TINYINT UNSIGNED" => Value::from(Number::from(row.get::<u8, _>(index))),
                "SMALLINT UNSIGNED" => Value::from(Number::from(row.get::<u16, _>(index))),
                "MEDIUMINT UNSIGNED" | "INT UNSIGNED" | "INTEGER UNSIGNED" => {
                    Value::from(Number::from(row.get::<u32, _>(index)))
                }
                "BIGINT UNSIGNED" => Value::from(Number::from(row.get::<u64, _>(index))),
                "FLOAT" | "DOUBLE" | "REAL" | "NUMERIC" => {
                    let value =
                        Number::from_f64(row.get::<f64, _>(index)).ok_or("Invalid float value")?;
                    Value::Number(value)
                }
                "CHAR" | "VARCHAR" | "TINYTEXT" | "TEXT" | "MEDIUMTEXT" | "LONGTEXT" | "ENUM"
                | "SET" => Value::String(row.get::<String, _>(index)),
                "JSON" => Value::String(row.get::<Value, _>(index).to_string()),
                "BINARY" | "VARBINARY" | "TINYBLOB" | "BLOB" | "MEDIUMBLOB" | "LONGBLOB" => {
                    Value::String(STANDARD.encode(row.get::<Vec<u8>, _>(index)))
                }
                "BOOLEAN" => Value::Bool(row.get::<bool, _>(index)),
                "DATE" => Value::String(row.get::<NaiveDate, _>(index).to_string()),
                "TIME" => Value::String(row.get::<NaiveTime, _>(index).to_string()),
                "DATETIME" => Value::String(row.get::<NaiveDateTime, _>(index).to_string()),
                "TIMESTAMP" => Value::String(row.get::<DateTime<Utc>, _>(index).to_string()),
                "YEAR" => Value::String(row.get::<u32, _>(index).to_string()),
                _ => Value::Null,
            };
            map.insert(column.name().to_string(), value);
        }
        Ok(Self {
            value: Value::Object(map),
        })
    }
}
