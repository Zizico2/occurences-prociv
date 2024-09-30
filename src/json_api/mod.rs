use core::str;

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
        let client = self
            .reqwest_client
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
            .ok_or(OccurrencesError::Unknown)?;

        Ok(client)
    }
    async fn occorrences_iter(
        request: Result<RequestBuilder, OccurrencesError>,
    ) -> Result<Features, OccurrencesError> {
        let occurrences = request?
            .send()
            .await
            .inspect_err(|err| tracing::error!("send: {err}"))
            .map_err(|_| OccurrencesError::Unknown)?
            .bytes()
            .await
            .inspect_err(|err| tracing::error!("couldn't get body as bytes: {err}"))
            .map_err(|_| OccurrencesError::Unknown)?;

        let features = match serde_json::from_slice(&occurrences) {
            Ok(features) => features,
            Err(err) => {
                let occurrences = str::from_utf8(&occurrences)
                    .inspect_err(|err| tracing::error!("body is invalid utf8: {err}"))
                    .map_err(|_| OccurrencesError::Unknown)?;
                tracing::error!("{err}: {occurrences}");
                return Err(OccurrencesError::Unknown);
            }
        };
        Ok(features)
    }
    pub fn occorrences(&self) -> impl TryStream<Ok = Feature, Error = OccurrencesError> {
        let request = self.occorrences_request();
        try_stream! {
            let occorrences = Self::occorrences_iter(request).await?;

            for i in occorrences.features.into_iter() {
                yield i;
            }
        }
    }
}
