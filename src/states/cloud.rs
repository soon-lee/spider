pub(crate) async fn post_file(config: &String) {
    // let access_key = "access key";
    // let secret_key = "secret key";
    // let bucket_name = "bucket name";
    // let object_name = "object name";
    // let credential = Credential::new(access_key, secret_key);
    // let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
    //     credential,
    //     bucket_name,
    //     Duration::from_secs(3600),
    // ))
    // .build();
    // let uploader: AutoUploader = upload_manager.auto_uploader();
    // let params = AutoUploaderObjectParams::builder()
    //     .object_name(object_name)
    //     .file_name(object_name)
    //     .build();
    // uploader.upload_path("/home/qiniu/test.png", params)?;
}
struct QiniuClient {
    client: Client,
    bucket_manager: BucketManager,
}

impl QiniuClient {
    fn new(access_key: &str, secret_key: &str) -> Self {
        let auth = AccessKeySecretKey::new(access_key, secret_key);
        let client = Client::new(auth);
        let bucket_manager = BucketManager::new(client.clone());
        QiniuClient {
            client,
            bucket_manager,
        }
    }

    fn generate_upload_token(&self, bucket_name: &str, key: Option<&str>) -> UploadToken {
        let put_policy = PutPolicy::new(bucket_name, key, 3600);
        self.client.generate_upload_token(put_policy)
    }
}

// 上传接口
trait Uploader {
    fn upload(&self, file_path: &PathBuf) -> Result<String, String>;
}

// 单文件上传实现
struct SingleFileUploader {
    client: QiniuClient,
}

impl Uploader for SingleFileUploader {
    fn upload(&self, file_path: &PathBuf) -> Result<String, String> {
        // 使用七牛云SDK进行单文件上传
        // ...
    }
}

// 多文件上传实现
struct MultiFileUploader {
    client: QiniuClient,
}

impl Uploader for MultiFileUploader {
    fn upload(&self, file_path: &PathBuf) -> Result<String, String> {
        // 使用七牛云SDK进行多文件上传
        // ...
    }
}

// 大文件上传实现
struct LargeFileUploader {
    client: QiniuClient,
}

impl Uploader for LargeFileUploader {
    fn upload(&self, file_path: &PathBuf) -> Result<String, String> {
        // 使用七牛云SDK进行大文件上传
        // ...
    }
}

// 统一上传功能
fn upload_file(file_path: &PathBuf, uploader: &dyn Uploader) -> Result<String, String> {
    uploader.upload(file_path)
}