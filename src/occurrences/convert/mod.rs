use chrono::{DateTime, Utc};
use prost_types::Timestamp;

use crate::json_api;

use crate::occurrences::foundation::v1::Coordinates;

use super::occurrence::v1::{GroupedStatus, Occurrence, OccurrenceStatus};
use super::spatial_planning::v1::{Crepc, Csrepc};

mod code_map;

impl From<json_api::models::Coordinates> for Coordinates {
    fn from(value: json_api::models::Coordinates) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}

impl From<json_api::models::Crepc> for Crepc {
    fn from(value: json_api::models::Crepc) -> Self {
        match value {
            json_api::models::Crepc::C1 => Self::C1,
            json_api::models::Crepc::C2 => Self::C2,
            json_api::models::Crepc::C3 => Self::C3,
            json_api::models::Crepc::C4 => Self::C4,
            json_api::models::Crepc::C5 => Self::C5,
        }
    }
}

impl From<json_api::models::Csrepc> for Csrepc {
    fn from(value: json_api::models::Csrepc) -> Self {
        match value {
            json_api::models::Csrepc::C1 => Self::C1,
            json_api::models::Csrepc::C2 => Self::C2,
            json_api::models::Csrepc::C3 => Self::C3,
            json_api::models::Csrepc::C4 => Self::C4,
            json_api::models::Csrepc::C5 => Self::C5,
            json_api::models::Csrepc::C6 => Self::C6,
            json_api::models::Csrepc::C7 => Self::C7,
            json_api::models::Csrepc::C8 => Self::C8,
            json_api::models::Csrepc::C9 => Self::C9,
            json_api::models::Csrepc::C10 => Self::C10,
            json_api::models::Csrepc::C11 => Self::C11,
            json_api::models::Csrepc::C12 => Self::C12,
            json_api::models::Csrepc::C13 => Self::C13,
            json_api::models::Csrepc::C14 => Self::C14,
            json_api::models::Csrepc::C15 => Self::C15,
            json_api::models::Csrepc::C16 => Self::C16,
            json_api::models::Csrepc::C17 => Self::C17,
            json_api::models::Csrepc::C18 => Self::C18,
            json_api::models::Csrepc::C19 => Self::C19,
            json_api::models::Csrepc::C20 => Self::C20,
            json_api::models::Csrepc::C21 => Self::C21,
            json_api::models::Csrepc::C22 => Self::C22,
            json_api::models::Csrepc::C23 => Self::C23,
            json_api::models::Csrepc::C24 => Self::C24,
        }
    }
}

impl From<json_api::models::GroupedStatus> for GroupedStatus {
    fn from(value: json_api::models::GroupedStatus) -> Self {
        match value {
            json_api::models::GroupedStatus::Active => Self::Active,
            json_api::models::GroupedStatus::Concluding => Self::Concluding,
            json_api::models::GroupedStatus::Dispatching => Self::Dispatching,
            json_api::models::GroupedStatus::Resolving => Self::Resolving,
        }
    }
}

impl From<json_api::models::OccurrenceStatus> for OccurrenceStatus {
    fn from(value: json_api::models::OccurrenceStatus) -> Self {
        match value {
            json_api::models::OccurrenceStatus::Active => Self::Active,
            json_api::models::OccurrenceStatus::InitialDispatch => Self::FirstDispatch,
            json_api::models::OccurrenceStatus::Dispatch => Self::Dispatch,
            json_api::models::OccurrenceStatus::Concluding => Self::Concluding,
            json_api::models::OccurrenceStatus::Resolving => Self::Resolving,
            json_api::models::OccurrenceStatus::Monitoring => Self::Monitoring,
            json_api::models::OccurrenceStatus::SiteArrival => Self::SiteArrival,
        }
    }
}

impl From<json_api::models::Occurrence> for Occurrence {
    fn from(value: json_api::models::Occurrence) -> Self {
        Self {
            kind: Some(value.kind.into()),
            location: Some(value.location.into()),
            start_date: Some(Timestamp::from_chrono(value.start_date)),
            data_generated_at: Some(Timestamp::from_chrono(value.data_generated_at)),
            crepc: Crepc::from(value.crepc).into(),
            csrepc: Csrepc::from(value.csrepc).into(),
            occurence_status: OccurrenceStatus::from(value.occurence_status).into(),
            grouped_status: GroupedStatus::from(value.grouped_status).into(),
            number_of_air_means: value.number_of_air_means,
            number_of_water_means: value.number_of_water_means,
            number_of_land_means: value.number_of_land_means,
            number_of_operatives: value.number_of_operatives,
            // `value.anepc_number` shouldn't be an f64 but the API is weird
            anepc_number: value.anepc_number.trunc() as u64,
        }
    }
}

pub trait TimestampExt {
    fn from_chrono(date_time: DateTime<Utc>) -> Self;
}

impl TimestampExt for Timestamp {
    fn from_chrono(date_time: DateTime<Utc>) -> Self {
        let seconds = date_time.timestamp();
        let nanoseconds = date_time.timestamp_subsec_nanos();

        // TODO: I hate this. nanoseconds should be u32. Google well known types are stupid.
        // TODO: but I can't justify having this return an error. Nanoseconds should always fit in an i32, however,
        // TODO: I should split this into a function and test it.
        let nanos = nanoseconds
            .try_into()
            .expect("invalid nanoseconds, doesn't fit in i32");

        Self { seconds, nanos }
    }
}
