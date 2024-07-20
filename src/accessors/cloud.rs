use std::env::var;
use std::io::Cursor;
use std::time::Duration;

use qiniu_sdk::credential::Credential;
use qiniu_sdk::download::{DownloadManager, EndpointsUrlGenerator, UrlsSigner};
use qiniu_sdk::download::apis::http_client::BucketDomainsQueryer;
use qiniu_sdk::upload::{AutoUploader, AutoUploaderObjectParams, UploadManager, UploadTokenSigner};
use serde_json::Value;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::utils::crypt::{decrypt_bytes, encrypt_bytes};

#[derive(Debug)]
pub(crate) struct QiniuClient {
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) bucket_name: String,
    pub(crate) aes_key: String,
    pub(crate) upload_manager: UploadManager,
    pub(crate) download_manager: DownloadManager,
}
impl QiniuClient {
    pub(crate) fn construct() -> Result<Self, String> {
        let access_key = var("QINIU_ACCESS_KEY").map_err(|_| "缺失环境变量：QINIU_ACCESS_KEY！")?;
        let secret_key = var("QINIU_SECRET_KEY").map_err(|_| "缺失环境变量：QINIU_SECRET_KEY！")?;
        let bucket_name =
            var("QINIU_BUCKET_NAME").map_err(|_| "缺失环境变量：QINIU_BUCKET_NAME！")?;
        let aes_key = var("QINIU_AES_KEY").map_err(|_| "缺失环境变量：QINIU_AES_KEY！")?;
        let upload_credential = Credential::new(&access_key, &secret_key);
        let token_signer = UploadTokenSigner::new_credential_provider(
            upload_credential,
            &bucket_name,
            Duration::from_secs(3600),
        );
        let upload_manager = UploadManager::builder(token_signer).build();
        let query_credential = Credential::new(&access_key, &secret_key);
        let domain_query = BucketDomainsQueryer::new().query(query_credential, &bucket_name);
        let url_generator = EndpointsUrlGenerator::builder(domain_query)
            .use_https(false)
            .build();
        let download_credential = Credential::new(&access_key, &secret_key);
        let url_signer = UrlsSigner::new(download_credential, url_generator);
        let download_manager = DownloadManager::builder(url_signer).build();
        Ok(Self {
            access_key,
            secret_key,
            bucket_name,
            aes_key,
            upload_manager,
            download_manager,
        })
    }
    pub(crate) async fn post_bytes(
        &self,
        bytes: &[u8],
        object_name: &str,
    ) -> Result<Value, String> {
        let magic = &bytes[0..2];
        let object_name = object_name.to_string();
        let object_name = match magic {
            &[0xff, 0xd8] => object_name.replace(".data", "jpg"),
            &[0x89, 0x50] => object_name.replace(".data", "png"),
            &[0x47, 0x49] => object_name.replace(".data", "gif"),
            &[0x42, 0x4d] => object_name.replace(".data", "bmp"),
            _ => object_name,
        };
        let bytes = encrypt_bytes(bytes, &self.aes_key, &object_name.as_str())?;
        let bytes = bytes.to_vec();
        let params = AutoUploaderObjectParams::builder()
            .object_name(&object_name)
            .file_name(&object_name)
            .build();
        let uploader: AutoUploader = self.upload_manager.auto_uploader();
        let response = uploader
            .async_upload_reader(Cursor::new(bytes).compat(), params)
            .await
            .map_err(|err| format!("上传失败：{}", err))?;
        Ok(response)
    }
    pub(crate) async fn get_bytes(&self, object_name: &str) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        let stream = self
            .download_manager
            .async_download(&object_name)
            .await
            .map_err(|err| format!("请求失败：{}", err))?;
        stream
            .to_async_writer(&mut bytes)
            .await
            .map_err(|err| format!("下载失败：{}", err))?;
        let bytes = decrypt_bytes(&bytes, &self.aes_key, object_name)?;
        Ok(bytes)
    }
}
