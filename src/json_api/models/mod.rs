use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use occurrence_kind::OccurenceKind;
use serde::{Deserialize, Serialize};

pub mod occurrence_kind;

#[derive(Debug, Serialize, Deserialize)]
pub enum Crepc {
    #[serde(rename = "Norte")]
    C1,
    #[serde(rename = "Centro")]
    C2,
    #[serde(rename = "Lisboa e Vale do Tejo")]
    C3,
    #[serde(rename = "Algarve")]
    C4,
    #[serde(rename = "Alentejo")]
    C5,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Csrepc {
    #[serde(rename = "Alto Minho")]
    C1,
    #[serde(rename = "Alto Tâmega e Barroso")]
    C2,
    #[serde(rename = "Area M. Porto")]
    C3,
    #[serde(rename = "Ave")]
    C4,
    #[serde(rename = "Cávado")]
    C5,
    #[serde(rename = "Douro")]
    C6,
    #[serde(rename = "Tâmega e Sousa")]
    C7,
    #[serde(rename = "Terras de Trás-os-Montes")]
    C8,
    #[serde(rename = "Beira Baixa")]
    C9,
    #[serde(rename = "Beiras e Serra da Estrela")]
    C10,
    #[serde(rename = "Região de Aveiro")]
    C11,
    #[serde(rename = "Região de Leiria")]
    C12,
    #[serde(rename = "Região de Coimbra")]
    C13,
    #[serde(rename = "Viseu Dão Lafões")]
    C14,
    #[serde(rename = "Grande Lisboa")]
    C15,
    #[serde(rename = "Península de Setúbal")]
    C16,
    #[serde(rename = "Lezíria do Tejo")]
    C17,
    #[serde(rename = "Médio Tejo")]
    C18,
    #[serde(rename = "Oeste")]
    C19,
    #[serde(rename = "Alentejo Central")]
    C20,
    #[serde(rename = "Alentejo Litoral")]
    C21,
    #[serde(rename = "Alto Alentejo")]
    C22,
    #[serde(rename = "Baixo Alentejo")]
    C23,
    #[serde(rename = "Algarve")]
    C24,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub enum GroupedStatus {
    #[serde(rename = "Em Curso")]
    Active,
    #[serde(rename = "Em Conclusão")]
    Concluding,
    #[serde(rename = "Em Despacho")]
    Dispatching,
    #[serde(rename = "Em Resolução")]
    Resolving,
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub enum OccurrenceStatus {
    #[serde(rename = "Em Curso")]
    Active,
    #[serde(rename = "Despacho de 1.º Alerta")]
    InitialDispatch,
    #[serde(rename = "Despacho")]
    Dispatch,
    #[serde(rename = "Conclusão")]
    Concluding,
    #[serde(rename = "Em Resolução")]
    Resolving,
    #[serde(rename = "Vigilância")]
    Monitoring,
    #[serde(rename = "Chegada ao TO")]
    SiteArrival,
}

#[derive(Debug, Deserialize)]
pub struct Occurrence {
    #[serde(rename = "OBJECTID")]
    pub objectid: i32,
    #[serde(rename = "CodNatureza")]
    pub kind: OccurenceKind,
    #[serde(rename = "Data", with = "ts_milliseconds")]
    pub start_date: DateTime<Utc>,
    #[serde(rename = "DataDosDados", with = "ts_milliseconds")]
    pub data_generated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub location: Coordinates,
    #[serde(rename = "EstadoOcorrencia")]
    pub occurence_status: OccurrenceStatus,
    #[serde(rename = "EstadoAgrupado")]
    pub grouped_status: GroupedStatus,
    #[serde(rename = "CREPC")]
    pub crepc: Crepc,
    #[serde(rename = "CSREPC")]
    pub csrepc: Csrepc,
    #[serde(rename = "NumeroMeiosAereosEnvolvidos")]
    pub number_of_air_means: u32,
    #[serde(rename = "NumeroMeiosAquaticos")]
    pub number_of_water_means: u32,
    #[serde(rename = "NumeroMeiosTerrestresEnvolvidos")]
    pub number_of_land_means: u32,
    #[serde(rename = "Operacionais")]
    pub number_of_operatives: u32,
    // this shouldn't be an f64 but the API is weird
    #[serde(rename = "Numero")]
    pub anepc_number: f64,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub attributes: Occurrence,
    pub exceeded_transfer_limit: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    pub features: Vec<Feature>,
}
