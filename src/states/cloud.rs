use std::env::var;
use std::io::Cursor;
use std::time::Duration;

use futures::io::AsyncReadExt;
use qiniu_sdk::apis::credential::Credential;
use qiniu_sdk::upload::{AutoUploader, AutoUploaderObjectParams, UploadManager, UploadTokenSigner};
use serde_json::Value;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::utils::crypt::encrypt_bytes;

#[derive(Debug)]
pub(crate) struct QiniuClient {
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) bucket_name: String,
    pub(crate) aes_key: String,
    pub(crate) upload_manager: UploadManager,
    pub(crate) uploader: AutoUploader,
}
impl QiniuClient {
    pub(crate) fn construct() -> Result<Self,String> {
        let access_key = var("QINIU_ACCESS_KEY").map_err(|_| "缺失环境变量：QINIU_ACCESS_KEY！")?;
        let secret_key = var("QINIU_SECRET_KEY").map_err(|_| "缺失环境变量：QINIU_SECRET_KEY！")?;
        let bucket_name = var("QINIU_BUCKET_NAME").map_err(|_| "缺失环境变量：QINIU_BUCKET_NAME！")?;
        let aes_key = var("QINIU_AES_KEY").map_err(|_| "缺失环境变量：QINIU_AES_KEY！")?;
        let credential = Credential::new(&access_key, &secret_key);
        let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
            credential,
            &bucket_name,
            Duration::from_secs(3600),
        ))
        .build();
        let uploader: AutoUploader = upload_manager.auto_uploader();
        Ok(Self {
            access_key,
            secret_key,
            bucket_name,
            aes_key,
            upload_manager,
            uploader,
        })
    }
    pub(crate) async fn post_bytes(&self, bytes:&[u8], object_name: &str) -> Result<Value, String> {
        let bytes = encrypt_bytes(bytes, &self.aes_key, object_name)?;
        let cursor = Cursor::new(bytes);
        let mut reader = cursor.compat();
        let mut buff = [0;1024];
        reader.read_exact(&mut buff).await.map_err(|err| format!("读取失败：{}",err))?;
        let params = AutoUploaderObjectParams::builder()
            .object_name(object_name)
            .file_name(object_name)
            .build();
        let response = self.uploader.async_upload_reader(reader, params).await.map_err(|err| format!("上传失败：{}",err))?;
        println!("上传成功：{}", response);
        Ok(response)
    }
    // pub(crate) async fn get_bytes(&self, object_name: &str) -> Result<Vec<u8>, String> {
    //     let mut bytes = Vec::new();
    //     let mut response = self.upload_manager.d.get_object_reader(&self.bucket_name, object_name).await.map_err(|err| format!("获取失败：{}",err))?;
    //     response.read_to_end(&mut bytes).await.map_err(|err| format!("读取失败：{}",err))?;
    // }
}