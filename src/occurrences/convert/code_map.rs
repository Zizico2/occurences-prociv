use macros::code_consts;
use occurrence::v1::Kind;

use crate::{json_api::models::occurrence_kind::OccurenceKind, occurrences::*};

code_consts!();

impl From<OccurenceKind> for occurrence::v1::Kind {
    fn from(value: OccurenceKind) -> Self {
        match value {
            OccurenceKind::C1101 => code_consts::C1101,
            OccurenceKind::C1103 => code_consts::C1103,
            OccurenceKind::C1105 => code_consts::C1105,
            OccurenceKind::C1107 => code_consts::C1107,
            OccurenceKind::C1109 => code_consts::C1109,
            OccurenceKind::C1111 => code_consts::C1111,
            OccurenceKind::C1113 => code_consts::C1113,
            OccurenceKind::C1119 => code_consts::C1119,
            OccurenceKind::C1125 => code_consts::C1125,
            OccurenceKind::C2101 => code_consts::C2101,
            OccurenceKind::C2107 => code_consts::C2107,
            OccurenceKind::C2109 => code_consts::C2109,
            OccurenceKind::C2111 => code_consts::C2111,
            OccurenceKind::C2113 => code_consts::C2113,
            OccurenceKind::C2115 => code_consts::C2115,
            OccurenceKind::C2117 => code_consts::C2117,
            OccurenceKind::C2119 => code_consts::C2119,
            OccurenceKind::C2121 => code_consts::C2121,
            OccurenceKind::C2123 => code_consts::C2123,
            OccurenceKind::C2125 => code_consts::C2125,
            OccurenceKind::C2127 => code_consts::C2127,
            OccurenceKind::C2129 => code_consts::C2129,
            OccurenceKind::C2201 => code_consts::C2201,
            OccurenceKind::C2203 => code_consts::C2203,
            OccurenceKind::C2301 => code_consts::C2301,
            OccurenceKind::C2303 => code_consts::C2303,
            OccurenceKind::C2305 => code_consts::C2305,
            OccurenceKind::C2307 => code_consts::C2307,
            OccurenceKind::C2401 => code_consts::C2401,
            OccurenceKind::C2403 => code_consts::C2403,
            OccurenceKind::C2405 => code_consts::C2405,
            OccurenceKind::C2407 => code_consts::C2407,
            OccurenceKind::C2409 => code_consts::C2409,
            OccurenceKind::C2411 => code_consts::C2411,
            OccurenceKind::C2413 => code_consts::C2413,
            OccurenceKind::C2415 => code_consts::C2415,
            OccurenceKind::C2417 => code_consts::C2417,
            OccurenceKind::C2419 => code_consts::C2419,
            OccurenceKind::C2421 => code_consts::C2421,
            OccurenceKind::C2423 => code_consts::C2423,
            OccurenceKind::C2425 => code_consts::C2425,
            OccurenceKind::C2501 => code_consts::C2501,
            OccurenceKind::C2503 => code_consts::C2503,
            OccurenceKind::C2505 => code_consts::C2505,
            OccurenceKind::C2507 => code_consts::C2507,
            OccurenceKind::C2509 => code_consts::C2509,
            OccurenceKind::C2511 => code_consts::C2511,
            OccurenceKind::C3101 => code_consts::C3101,
            OccurenceKind::C3103 => code_consts::C3103,
            OccurenceKind::C3105 => code_consts::C3105,
            OccurenceKind::C3107 => code_consts::C3107,
            OccurenceKind::C3109 => code_consts::C3109,
            OccurenceKind::C3111 => code_consts::C3111,
            OccurenceKind::C3301 => code_consts::C3301,
            OccurenceKind::C3309 => code_consts::C3309,
            OccurenceKind::C3311 => code_consts::C3311,
            OccurenceKind::C3313 => code_consts::C3313,
            OccurenceKind::C3315 => code_consts::C3315,
            OccurenceKind::C3321 => code_consts::C3321,
            OccurenceKind::C3323 => code_consts::C3323,
            OccurenceKind::C3325 => code_consts::C3325,
            OccurenceKind::C3327 => code_consts::C3327,
            OccurenceKind::C3329 => code_consts::C3329,
            OccurenceKind::C3331 => code_consts::C3331,
            OccurenceKind::C3333 => code_consts::C3333,
            OccurenceKind::C4111 => code_consts::C4111,
            OccurenceKind::C4113 => code_consts::C4113,
            OccurenceKind::C4201 => code_consts::C4201,
            OccurenceKind::C4203 => code_consts::C4203,
            OccurenceKind::C4207 => code_consts::C4207,
            OccurenceKind::C4209 => code_consts::C4209,
            OccurenceKind::C4327 => code_consts::C4327,
            OccurenceKind::C4329 => code_consts::C4329,
            OccurenceKind::C4331 => code_consts::C4331,
            OccurenceKind::C4333 => code_consts::C4333,
            OccurenceKind::C4335 => code_consts::C4335,
            OccurenceKind::C4339 => code_consts::C4339,
            OccurenceKind::C9103 => code_consts::C9103,
            // TODO: add error type...
            OccurenceKind::Unspecified => Kind { inner: None },
        }
    }
}
