use std::time::Duration;

use mongodb::Collection;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use occurences_prociv::json_api::QueryParams;
use tokio::time::{interval, MissedTickBehavior};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let pool = PgPool::connect("postgresql://neondb_owner:Txz31LFiRfsZ@ep-young-pond-a2s4b5je.eu-central-1.aws.neon.tech/neondb?sslmode=require").await?;
    // Replace the placeholder with your Atlas connection string
    let mut client_options =
        ClientOptions::parse("mongodb+srv://admin:admin@cluster0.9ztma.mongodb.net/").await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    let db = client.database("hi");

    let my_coll: Collection<serde_json::Value> = db.collection("features");

    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let reqwest_client = reqwest::Client::new();

    let mut interval_10 = interval(Duration::from_secs(10));
    interval_10.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut interval_300 = interval(Duration::from_secs(300));
    interval_300.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval_10.tick().await;

        tracing::info!("fetching features...");
        let features = reqwest_client
        .get("https://prociv-agserver.geomai.mai.gov.pt/arcgis/rest/services/Ocorrencias_Base/FeatureServer/0/query")
        .query(&QueryParams {
            f: "json".into(),
            cache_hint: false,
            result_offset: 0,
            r#where: "1=1".into(),
            order_by_fields: "Data DESC".into(),
            out_fields: "*".into(),
            return_geometry: false,
        })
        .send()
        .await.inspect_err(|err| tracing::error!("{err}"));

        let features = match features {
            Ok(features) => features,
            Err(_) => continue,
        };

        let features = features
            .json::<serde_json::Value>()
            .await
            .inspect_err(|err| tracing::error!("{err}"));

        let features = match features {
            Ok(features) => features,
            Err(_) => continue,
        };

        let features = if let serde_json::Value::Object(features) = features {
            features
        } else {
            continue;
        };
        let features = if let Some(features) = features.get("features") {
            features.clone()
        } else {
            continue;
        };
        let features = if let serde_json::Value::Array(features) = features {
            features
        } else {
            continue;
        };

        if features.is_empty() {
            continue;
        }

        tracing::info!("inserting features...");
        let features = features.into_iter().filter_map(|feat| {
            if let serde_json::Value::Object(features) = feat {
                features.get("attributes").cloned()
            } else {
                None
            }
        });

        my_coll
            .insert_many(features)
            .await
            .inspect_err(|err| tracing::error!("{err}"))
            .ok();

        interval_300.tick().await;
    }
}
