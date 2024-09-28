use async_stream::try_stream;
use futures::TryStream;
use models::{Feature, Features};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub f: String,
    #[serde(rename = "cacheHint")]
    pub cache_hint: bool,
    #[serde(rename = "resultOffset")]
    pub result_offset: u32,
    #[serde(rename = "where")]
    pub r#where: String,
    #[serde(rename = "orderByFields")]
    pub order_by_fields: String,
    #[serde(rename = "outFields")]
    pub out_fields: String,
    #[serde(rename = "returnGeometry")]
    pub return_geometry: bool,
}

#[derive(Debug, Clone)]
pub struct OccurrencesClient {
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum OccurrencesError {
    #[error("unknown")]
    Unknown,
}

// TODO: add pagination
// TODO: exceededTransferLimit
impl OccurrencesClient {
    fn occorrences_request(&self) -> Result<RequestBuilder, OccurrencesError> {
        self.reqwest_client
            .get(&self.base_url)
            .query(&QueryParams {
                f: "json".into(),
                cache_hint: false,
                result_offset: 0,
                r#where: "1=1".into(),
                order_by_fields: "Data DESC".into(),
                out_fields: "*".into(),
                return_geometry: false,
            })
            .try_clone()
            .ok_or(OccurrencesError::Unknown)
    }
    pub fn occorrences(&self) -> impl TryStream<Ok = Feature, Error = OccurrencesError> {
        let request = self.occorrences_request();
        try_stream! {
            let fires = request?
                .send()
                .await.map_err(|_| OccurrencesError::Unknown)?
                .json::<Features>()
                .await.map_err(|_| OccurrencesError::Unknown)?;

            for i in fires.features.into_iter() {
                yield i;
            }
        }
    }
}
