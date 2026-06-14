pub mod contract;
pub mod domain;
pub mod error;
mod handlers;
pub mod msg;
mod services;
pub mod state;
mod translation;

pub use contract::interface::AxoneVcInterface;
pub use error::AxoneVcError;

/// The version of your app.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const AXONE_NAMESPACE: &str = "axone";
pub const AXONE_VC_NAME: &str = "axone-vc";
pub const AXONE_VC_ID: &str = const_format::concatcp!(AXONE_NAMESPACE, ":", AXONE_VC_NAME);
pub const RESPONSE_KEY_AUTHORITY: &str = "authority";
pub const RESPONSE_KEY_CREDENTIAL_ID: &str = "credential_id";

// oxrdf pulls rand/getrandom transitively, but CosmWasm contracts must not depend
// on host randomness. This custom backend keeps wasm32-unknown-unknown builds
// linkable and fails explicitly if random bytes are ever requested at runtime.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_getrandom {
    use core::num::NonZeroU32;

    use getrandom::{register_custom_getrandom, Error};

    pub fn unavailable(_: &mut [u8]) -> Result<(), Error> {
        let code = NonZeroU32::new(Error::CUSTOM_START)
            .expect("getrandom custom error code should be non-zero");
        Err(Error::from(code))
    }

    register_custom_getrandom!(unavailable);
}
