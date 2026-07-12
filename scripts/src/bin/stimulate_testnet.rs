//! Generate one varied synthetic Abstract activity scenario on a target network.

use abstract_client::{AbstractClient, Application};
use abstract_std::objects::module::{ModuleInfo, ModuleStatus, ModuleVersion};
use axone_gov::{
    msg::{AxoneGovExecuteMsg, AxoneGovInstantiateMsg, ExecuteMsg as AxoneGovExecuteEndpointMsg},
    AxoneGovInterface, APP_VERSION as AXONE_GOV_VERSION, AXONE_GOV_ID,
};
use axone_networks::{
    abstract_deployment::seed_abstract_addresses, parse_network as parse_axone_network,
};
use axone_scripts::synthetic_account::{
    self, DEFAULT_DESCRIPTION, DEFAULT_LINK, DEFAULT_NAME_PREFIX,
};
use axone_vc::{
    msg::{
        AuthorityResponse, AxoneVcExecuteMsg, AxoneVcInstantiateMsg, AxoneVcQueryMsg,
        ExecuteMsg as AxoneVcExecuteEndpointMsg, QueryMsg as AxoneVcQueryEndpointMsg,
    },
    AxoneVcInterface, APP_VERSION as AXONE_VC_VERSION, AXONE_VC_ID,
};
use clap::Parser;
use cosmwasm_std::Binary;
use cw_orch::{
    anyhow,
    daemon::{networks::ChainInfo, Daemon},
    prelude::*,
    tokio::runtime::Runtime,
};
use log::info;
use rand::{rngs::StdRng, Rng, SeedableRng};

const CONSTITUTION: &str = include_str!("../../examples/constitution.pl");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Profile {
    AccountOnly,
    Governance,
    Credential,
    GovernanceAndCredential,
}

impl Profile {
    fn needs_governance(self) -> bool {
        matches!(self, Self::Governance | Self::GovernanceAndCredential)
    }
    fn needs_credential(self) -> bool {
        matches!(self, Self::Credential | Self::GovernanceAndCredential)
    }
}

fn select_profile(rng: &mut impl Rng) -> Profile {
    let profiles = [
        Profile::AccountOnly,
        Profile::Governance,
        Profile::Credential,
        Profile::GovernanceAndCredential,
    ];
    profiles[rng.gen_range(0..profiles.len())]
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(short, long, default_value = "testnet")]
    network_id: String,
    #[arg(long, default_value = DEFAULT_NAME_PREFIX)]
    name_prefix: String,
    #[arg(long)]
    run_marker: Option<String>,
    #[arg(long, default_value = DEFAULT_DESCRIPTION)]
    description: String,
    #[arg(long, default_value = DEFAULT_LINK)]
    link: String,
    /// Deterministic seed for manual reproduction of a profile selection.
    #[arg(long)]
    seed: Option<u64>,
}

fn assert_published(
    client: &AbstractClient<Daemon>,
    id: &str,
    version: &str,
) -> anyhow::Result<()> {
    let info = ModuleInfo::from_id(id, ModuleVersion::Version(version.to_string()))?;
    if client.module_status(info)? != Some(ModuleStatus::Registered) {
        anyhow::bail!(
            "required module {id}@{version} is not published and registered on Abstract testnet"
        );
    }
    Ok(())
}

fn gov_execute(message: AxoneGovExecuteMsg) -> AxoneGovExecuteEndpointMsg {
    AxoneGovExecuteEndpointMsg::Module(message)
}

fn vc_execute(message: AxoneVcExecuteMsg) -> AxoneVcExecuteEndpointMsg {
    AxoneVcExecuteEndpointMsg::Module(message)
}

fn vc_query(message: AxoneVcQueryMsg) -> AxoneVcQueryEndpointMsg {
    AxoneVcQueryEndpointMsg::Module(message)
}

fn issue_credential_payload(authority: &str, marker: &str) -> Binary {
    let identifier = format!("urn:axone:testnet:credential:{marker}");
    let subject = format!("urn:axone:testnet:subject:{marker}");
    Binary::from(format!(
        "<{identifier}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .\n\
<{identifier}> <https://www.w3.org/2018/credentials#issuer> <{authority}> .\n\
<{identifier}> <https://www.w3.org/2018/credentials#issuanceDate> \"2026-01-01T00:00:00Z\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n\
<{identifier}> <https://www.w3.org/2018/credentials#credentialSubject> <{subject}> .\n\
<{identifier}> <https://www.w3.org/2018/credentials#type> <https://axone.xyz/testnet/SyntheticCredential> .\n"
    ).into_bytes())
}

fn stimulate(network: ChainInfo, args: &Arguments) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;
    seed_abstract_addresses(&chain, &network, &rt)?;
    let client = AbstractClient::new(chain)?;
    assert_published(&client, AXONE_GOV_ID, AXONE_GOV_VERSION)?;
    assert_published(&client, AXONE_VC_ID, AXONE_VC_VERSION)?;

    let marker = args
        .run_marker
        .clone()
        .unwrap_or_else(synthetic_account::default_marker);
    let mut rng = args
        .seed
        .map_or_else(StdRng::from_entropy, StdRng::seed_from_u64);
    let profile = select_profile(&mut rng);
    info!("Selected synthetic profile: {profile:?}");

    let account = synthetic_account::create(
        &client,
        &args.name_prefix,
        &marker,
        &args.description,
        &args.link,
    )?;
    info!(
        "Created Abstract Account: id={}, address={}",
        account.id()?,
        account.address()?
    );

    if profile.needs_governance() {
        let app: Application<Daemon, AxoneGovInterface<Daemon>> = account
            .install_app::<AxoneGovInterface<_>>(
                &AxoneGovInstantiateMsg {
                    constitution: Binary::from(CONSTITUTION.as_bytes()),
                },
                &[],
            )?;
        let cases = [
            "case{action:transfer}",
            "case{action:withdraw}",
            "case{action:mint}",
        ];
        let case = cases[rng.gen_range(0..cases.len())].to_string();
        app.execute(
            &gov_execute(AxoneGovExecuteMsg::RecordDecision {
                case,
                motivated: Some(false),
            }),
            &[],
        )?;
        info!("Recorded synthetic governance decision");
    }

    if profile.needs_credential() {
        let app: Application<Daemon, AxoneVcInterface<Daemon>> =
            account.install_app::<AxoneVcInterface<_>>(&AxoneVcInstantiateMsg {}, &[])?;
        let authority: AuthorityResponse = app.query(&vc_query(AxoneVcQueryMsg::Authority {}))?;
        app.execute(
            &vc_execute(AxoneVcExecuteMsg::IssueCredential {
                credential: issue_credential_payload(&authority.did, &marker),
                format: None,
            }),
            &[],
        )?;
        info!("Issued synthetic verifiable credential");
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Arguments::parse();
    let network = parse_axone_network(&args.network_id)
        .or_else(|_| networks::parse_network(&args.network_id))
        .map_err(anyhow::Error::msg)?;
    stimulate(network, &args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_selection_is_reproducible_and_covers_every_profile() {
        let mut profiles = std::collections::BTreeSet::new();
        for seed in 0..128 {
            let mut rng = StdRng::seed_from_u64(seed);
            profiles.insert(select_profile(&mut rng) as u8);
        }
        assert_eq!(profiles.len(), 4);

        let mut first = StdRng::seed_from_u64(42);
        let mut second = StdRng::seed_from_u64(42);
        assert_eq!(select_profile(&mut first), select_profile(&mut second));
    }

    #[test]
    fn credential_payload_uses_the_authority_and_marker() {
        let payload = String::from_utf8(
            issue_credential_payload("did:pkh:cosmos:axone:cosmos1authority", "run-1").to_vec(),
        )
        .unwrap();
        assert!(payload.contains("<did:pkh:cosmos:axone:cosmos1authority>"));
        assert!(payload.contains("urn:axone:testnet:credential:run-1"));
        assert!(payload.contains("VerifiableCredential"));
    }
}
