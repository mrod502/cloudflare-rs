pub mod dns;

pub use dns::*;
use log::{error, info};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use url::Url;

use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    Method, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

#[derive(Serialize, Clone, Copy)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
}
impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::V1 => "v1",
            Self::V2 => "v2",
            Self::V3 => "v3",
            Self::V4 => "v4",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultInfo {
    pub count: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct V4PagePaginationArray<T> {
    pub result: Option<Vec<T>>,
    pub success: bool,
    pub messages: Option<Vec<ResponseInfo>>,
    pub errors: Option<Vec<ResponseInfo>>,
    pub result_info: Option<ResultInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseInfo {
    pub code: usize,
    pub message: String,
}

pub type ApiResult<T: Serialize + DeserializeOwned> = Result<T, ()>;

#[derive(Clone)]
pub struct Cloudflare {
    version: Version,
    base_url: Url,

    email: String,
    auth_key: String,
}

impl Default for Cloudflare {
    fn default() -> Self {
        Self {
            version: Version::V4,
            base_url: Url::parse("https://api.cloudflare.com").unwrap(),
            email: "".to_string(),
            auth_key: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TokenStatusResult {
    pub id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TokenStatus {
    pub result: TokenStatusResult,
}
pub struct CloudflareDns {
    c: Cloudflare,
}

pub type ListDnsRecordsResponse = V4PagePaginationArray<RecordMessage>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub result: Option<T>,
    pub success: bool,
    pub messages: Vec<ResponseInfo>,
    pub errors: Vec<ResponseInfo>,
    pub result_info: Option<ResultInfo>,
}

mod test {
    use std::net::Ipv4Addr;

    use super::*;
    #[tokio::test]
    async fn test_dns() {
        let c = Cloudflare {
            version: Version::V4,
            base_url: Url::parse("").unwrap(),
            email: "".to_string(),
            auth_key: "".to_string(),
        };

        let dns = c.dns();
        //let rec = dns
        //    .list_records(ListRecordsRequest {
        //        zone_id: "539d012da79713e97eedd9968738e2ee".to_string(),
        //        page: 1,
        //    })
        //    .await;
        let rec = dns.get_record("".to_string(), "".to_string()).await;
        let r = rec.unwrap().result.unwrap();
        info!("{:?}", r);
        if let RecordMessage::A(rec) = r {
            let mut rec = rec.clone();
            rec.content = Some(Ipv4Addr::from_str("").unwrap());
            let result = dns
                .overwrite_record("".to_string(), "".to_string(), rec)
                .await;
            error!("result:{:?}", result.unwrap().result.unwrap());
        } else {
            panic!("unexpected message type");
        }
    }
}

pub struct ListRecordsRequest {
    pub zone_id: String,
    pub page: usize,
}

impl CloudflareDns {
    pub async fn list_records(
        &self,
        ListRecordsRequest { zone_id, page }: ListRecordsRequest,
    ) -> ApiResult<ListDnsRecordsResponse> {
        self.c
            .clone()
            .perform_json(
                Method::GET,
                PerformOptions::<()> {
                    path: format!("zones/{}/dns_records", zone_id),
                    params: None,
                    body: None,
                },
            )
            .await
    }

    pub async fn get_record(
        &self,
        zone_id: String,
        record_id: String,
    ) -> ApiResult<ApiResponse<RecordMessage>> {
        self.c
            .clone()
            .perform_json(
                Method::GET,
                PerformOptions::<()> {
                    path: format!("zones/{}/dns_records/{}", zone_id, record_id),
                    body: None,
                    params: None,
                },
            )
            .await
    }
    pub async fn overwrite_record(
        self,
        zone_id: String,
        record_id: String,
        record: impl ToRecordMessage,
    ) -> ApiResult<ApiResponse<RecordMessage>> {
        let record = record.to_record_message().for_update();
        self.c
            .perform_json(
                Method::PUT,
                PerformOptions {
                    body: Some(record),
                    path: format!("zones/{}/dns_records/{}", zone_id, record_id),
                    params: None,
                },
            )
            .await
    }
}

impl Cloudflare {
    pub fn with_email(self, email: String) -> Self {
        let mut s = self.clone();

        s.email = email;
        s
    }

    pub fn with_token(self, tok: String) -> Self {
        let mut s = self.clone();
        s.auth_key = tok;
        s
    }

    fn bearer_token(&self) -> impl ToString {
        format!("Bearer {}", self.auth_key)
    }

    pub async fn verify_token(self) -> ApiResult<TokenStatus> {
        Ok(Default::default())
    }

    pub fn dns(&self) -> CloudflareDns {
        CloudflareDns { c: self.clone() }
    }

    async fn perform(
        self,
        method: Method,
        path: String,
        body: Option<Bytes>,
    ) -> ApiResult<Response<Incoming>> {
        let req_body = match body {
            Some(b) => b,
            None => Bytes::new(),
        };

        let tok = self.bearer_token().to_string().clone();
        let req = match Request::builder()
            .uri(self.url(path)?)
            .method(method)
            .header("Authorization", tok)
            .body(Full::new(req_body))
        {
            Ok(r) => r,
            Err(e) => return Err(()),
        };

        let client: Client<HttpsConnector<HttpConnector>, Full<Bytes>> =
            Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpsConnector::new());

        let res = match client.request(req).await {
            Ok(r) => {
                info!("success:{:?}", r);
                r
            }
            Err(e) => {
                error!("failed to send request:{}", e);
                return Err(());
            }
        };

        Ok(res)
    }

    fn url(self, path: String) -> Result<Uri, ()> {
        println!("getting uri");
        let base = self
            .base_url
            .clone()
            .to_string()
            .trim_end_matches("/")
            .to_string();
        let path = path.trim_start_matches("/");
        let uri = format!("{}/client/{}/{}", base, self.version.to_string(), path);
        println!("URI:{}", uri);
        let res = match Uri::from_str(&uri) {
            Ok(u) => Ok(u),
            Err(e) => Err(()),
        };
        println!("{:?}", res.clone());
        res
    }

    async fn perform_json<R, T>(self, method: Method, opts: PerformOptions<R>) -> ApiResult<T>
    where
        R: Serialize + Clone,
        T: DeserializeOwned,
    {
        let body = match opts.body {
            Some(bod) => Some(Bytes::from(serde_json::to_string(&bod).unwrap_or_default())),
            None => None,
        };

        let result = self.perform(method, opts.path, body).await?;

        let bytes = String::from_utf8(
            result
                .collect()
                .await
                .unwrap_or_default()
                .to_bytes()
                .to_vec(),
        )
        .unwrap_or_default();
        println!("raw_body:{}", bytes);
        let de: T = match serde_json::from_str(&bytes) {
            Ok(v) => v,
            Err(e) => {
                println!("error parsing json:{}", e);
                return Err(());
            }
        };

        Ok(de)
    }
}

#[derive(Clone)]
struct PerformOptions<T>
where
    T: Serialize + Clone,
{
    path: String,
    params: Option<HashMap<String, String>>,
    body: Option<T>,
}
