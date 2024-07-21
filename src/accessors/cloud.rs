use std::io::Cursor;
use std::sync::Arc;

use qiniu_sdk::download::DownloadManager;
use qiniu_sdk::upload::{AutoUploader, AutoUploaderObjectParams, UploadManager};
use serde_json::Value;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::utils::crypt::{decrypt_bytes, encrypt_bytes};

#[derive(Debug)]
pub(crate) struct CloudAccessor {
    aes_key: Arc<String>,
    upload_manager: Arc<UploadManager>,
    download_manager: Arc<DownloadManager>,
}
impl CloudAccessor {
    pub(crate) async fn new(aes_key: Arc<String>, upload_manager: Arc<UploadManager>, download_manager: Arc<DownloadManager>) -> Self {
        CloudAccessor {
            aes_key,
            upload_manager,
            download_manager,
        }
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
