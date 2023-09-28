use aliyun_oss_client::file::Files;
use aliyun_oss_client::{auth::QueryAuth, Client};
// use std::ops::Deref;

#[derive(Clone)]
pub struct OssClient(Client);

// impl Deref for OssClient {
//     type Target = Client;

//     fn deref(&self) -> &Client {
//         &self.0
//     }
// }

impl OssClient {
    pub fn from_env() -> anyhow::Result<Self> {
        let client = Client::from_env()?;
        Ok(Self(client))
    }
    pub async fn put<T: ToString, S: AsRef<str>>(&self, path: T, bytes: &[u8], content_type: S) -> anyhow::Result<()> {
        let response = self
            .0
            .put_content_base(bytes.to_vec(), content_type.as_ref(), path.to_string())
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;

        println!("respose: {:?}", response);
        Ok(())
    }
    pub async fn signature_url(&self, path: String, duration: std::time::Duration) -> anyhow::Result<String> {
        let auth = QueryAuth::from(&self.0);
        Ok(auth.to_url(&path.parse()?, chrono::Utc::now().timestamp() + duration.as_secs() as i64).to_string())
    }
}
