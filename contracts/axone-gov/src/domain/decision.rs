use crate::domain::constitution::ConstitutionStatus;
use cosmwasm_std::Addr;
use getset::{CopyGetters, Getters};

#[derive(Clone, CopyGetters, Debug, Getters, PartialEq)]
pub struct Decision {
    #[getset(get_copy = "pub")]
    constitution_revision: u64,
    #[getset(get_copy = "pub")]
    constitution_hash: [u8; 32],
    #[getset(get = "pub")]
    case: String,
    #[getset(get = "pub")]
    verdict: String,
    #[getset(get = "pub")]
    motivation: Option<String>,
    #[getset(get = "pub")]
    author: Addr,
    #[getset(get_copy = "pub")]
    height: u64,
    #[getset(get_copy = "pub")]
    time_seconds: u64,
}

impl Decision {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        constitution_status: &ConstitutionStatus,
        case: String,
        verdict: String,
        motivation: Option<String>,
        author: Addr,
        height: u64,
        time_seconds: u64,
    ) -> Self {
        Self {
            constitution_revision: constitution_status.constitution_revision(),
            constitution_hash: *constitution_status.constitution_hash(),
            case,
            verdict,
            motivation,
            author,
            height,
            time_seconds,
        }
    }
}
