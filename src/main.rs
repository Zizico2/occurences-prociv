use std::str::FromStr;

use clap::Parser;
use futures::stream::BoxStream;
use futures::StreamExt;
use futures::TryStreamExt;
use occurences_prociv::json_api::OccurrencesClient;
use occurences_prociv::occurrences;
use occurences_prociv::occurrences::occurrence::v1::occurrences_service_server::OccurrencesService;
use occurences_prociv::occurrences::occurrence::v1::occurrences_service_server::OccurrencesServiceServer;
use occurences_prociv::occurrences::occurrence::v1::ListOccurrencesRequest;
use occurences_prociv::occurrences::occurrence::v1::ListOccurrencesResponse;
use occurences_prociv::occurrences::occurrence::v1::Occurrence;
use sentry::types::Dsn;
use tonic::transport::Server;
use tonic::Status;
use tracing::instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long, env)]
    prociv_fires_base_url: String,
    #[clap(short, long, env)]
    sentry_dsn: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let Args {
        prociv_fires_base_url,
        sentry_dsn,
    } = Args::parse();
    let _guard = sentry::init(sentry::ClientOptions {
        traces_sample_rate: 1.0,
        dsn: Some(Dsn::from_str(&sentry_dsn)?),
        ..Default::default()
    });
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .init();
    let client = reqwest::Client::new();
    let occurrences_client = OccurrencesClient {
        base_url: prociv_fires_base_url,
        reqwest_client: client,
    };

    let occurrences_service = Occurrences { occurrences_client };

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(occurrences::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // TODO: port should come from ENV var
    let addr = "[::0]:50051".parse()?;
    // let addr = "127.0.0.1:50051".parse()?;

    Server::builder()
        .add_service(reflection)
        .add_service(OccurrencesServiceServer::new(occurrences_service))
        .serve(addr)
        .await?;

    Ok(())
}
#[derive(Debug)]
pub struct Occurrences {
    occurrences_client: OccurrencesClient,
}

#[tonic::async_trait]
impl OccurrencesService for Occurrences {
    type ListOccurrencesStream = BoxStream<'static, Result<ListOccurrencesResponse, Status>>;
    #[instrument]
    async fn list_occurrences(
        &self,
        _: tonic::Request<ListOccurrencesRequest>,
    ) -> Result<tonic::Response<Self::ListOccurrencesStream>, Status> {
        let occurrences = self
            .occurrences_client
            .occorrences()
            .map_err(|err| Status::from_error(err.into()))
            .map_ok(|feat| feat.attributes)
            .map_ok(Occurrence::from)
            .map_ok(|occurrence| ListOccurrencesResponse {
                occurrence: Some(occurrence),
            })
            .inspect_ok(|occ| tracing::info!("{:?}", occ));

        Ok(tonic::Response::new(occurrences.boxed()))
    }
}
