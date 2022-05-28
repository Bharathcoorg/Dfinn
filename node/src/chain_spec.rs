use frame_support::PalletId;
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use itertools::Itertools;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use dfinn_primitives::Block;
pub use dfinn_primitives::{AccountId, Balance, Signature};
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{AccountIdConversion, IdentifyAccount, Verify},
	Perbill,
};

pub use node_dfinn_runtime::GenesisConfig;
use node_dfinn_runtime::{
	constants::currency::FINN, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, CouncilConfig, IndicesConfig, OrmlVestingConfig, FINNMigrationConfig,
	SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig,
};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const MAINNET_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

pub(crate) fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn udon_testnet_config_genesis() -> GenesisConfig {
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5H8cZrqdVt8fBaFTouYvdxQUmdpPbUmQPw4XoMCg47w1k2oW
			hex!["0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"].into(),
			// 5Dt8Esj9b5bXj8PmJUse67KzLoLwfRYYUqTRh5T6gR92neuK
			hex!["0x5066d8f7858d1bac080c4028ed856de71b6e0c97e8b6261a20e9df69d78fd662"]
				.unchecked_into(),
		),
		(
			// 5H8cZrqdVt8fBaFTouYvdxQUmdpPbUmQPw4XoMCg47w1k2oW
			hex!["0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"].into(),
			// 5Dt8Esj9b5bXj8PmJUse67KzLoLwfRYYUqTRh5T6gR92neuK
			hex!["0x5066d8f7858d1bac080c4028ed856de71b6e0c97e8b6261a20e9df69d78fd662"]
				.unchecked_into(),
		),
	];

	let root_key: AccountId = hex![
		// 5H8cZrqdVt8fBaFTouYvdxQUmdpPbUmQPw4XoMCg47w1k2oW
		"0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"
	]
	.into();

	testnet_genesis(initial_authorities, vec![], root_key)
}

/// Staging testnet config.
pub fn udon_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Dfinn Test Net",
		"dfinn_udon_testnet",
		ChainType::Live,
		udon_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

fn soba_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
	)
}

/// Local testnet config ()
pub fn soba_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"soba_testnet",
		ChainType::Local,
		soba_testnet_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

fn mainnet_genesis_constuctor() -> GenesisConfig {
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5H8cZrqdVt8fBaFTouYvdxQUmdpPbUmQPw4XoMCg47w1k2oW
			hex!["0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"].into(),
			// 5Dt8Esj9b5bXj8PmJUse67KzLoLwfRYYUqTRh5T6gR92neuK
			hex!["0x5066d8f7858d1bac080c4028ed856de71b6e0c97e8b6261a20e9df69d78fd662"]
				.unchecked_into(),
		),
		(
			// 5H8cZrqdVt8fBaFTouYvdxQUmdpPbUmQPw4XoMCg47w1k2oW
			hex!["0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"].into(),
			// 5Dt8Esj9b5bXj8PmJUse67KzLoLwfRYYUqTRh5T6gR92neuK
			hex!["0x5066d8f7858d1bac080c4028ed856de71b6e0c97e8b6261a20e9df69d78fd662"]
				.unchecked_into(),
		),
	];
	let root_key = hex!["0xe0285781189953fdbfacbd2fe3eed78ecec0481b0b3c3bfd9ca5c5627bef5386"].into();
	testnet_genesis(initial_authorities, vec![], root_key)
}

pub fn mainnet_testnet_config() -> ChainSpec {
	let bootnodes = vec![];
	const DFINN_PROTOCOL_ID: &str = "finn";
	ChainSpec::from_genesis(
		"Bangalore Test Network",
		"bangalore_test_network",
		ChainType::Live,
		mainnet_genesis_constuctor,
		bootnodes,
		Some(
			TelemetryEndpoints::new(vec![(MAINNET_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DFINN_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	)
}

fn adjust_treasury_balance_for_initial_validators(
	initial_validators: usize,
	endowment: u128,
) -> u128 {
	// The extra one is for root_key
	(initial_validators + 1) as u128 * endowment
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	_initial_nominators: Vec<AccountId>,
	root_key: AccountId,
) -> GenesisConfig {
	const ENDOWMENT: u128 = 100 * FINN;
	const STASH: u128 = 2 * FINN;
	// Total Supply in ERC20
	const ERC20_FINN_SUPPLY: u128 =100_000_000 FINN;
	// Total funds in treasury also includes 2_000_000 FINN for parachain auctions
	let mut treasury_funds: u128 = 25_000_000 FINN;
	treasury_funds = treasury_funds -
		adjust_treasury_balance_for_initial_validators(initial_authorities.len(), ENDOWMENT);

	// Treasury Account Id
	pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"py/trsry");
	let treasury_account: AccountId = TREASURY_PALLET_ID.into_account();

	let mut inital_validators_endowment =
		initial_authorities.iter().map(|k| (k.0.clone(), ENDOWMENT)).collect_vec();
	let mut endowed_accounts = vec![
		//      Root Key
		(root_key.clone(), ENDOWMENT),
		//     Treasury Funds
		(treasury_account, treasury_funds),
	];
	// Get rest of the stake holders
	let mut claims = get_stakeholder_tokens();

	let mut total_claims: u128 = 0;
	for (_, balance) in &claims {
		total_claims = total_claims + balance;
	}

	assert_eq!(total_claims, 20_000_000 * FINN, "Total claims is configured correctly");

	endowed_accounts.append(claims.as_mut());
	// Endow to validators
	endowed_accounts.append(&mut inital_validators_endowment);

	let mut total_supply: u128 = 0;
	for (_, balance) in &endowed_accounts {
		total_supply = total_supply + balance.clone()
	}

	assert_eq!(
		total_supply + ERC20_FINN_SUPPLY,
		100_000_000 * FINN,
		"Total Supply Not equal to 100 Million"
	);
	let vesting = get_vesting_terms();

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig { balances: endowed_accounts },

		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)) // stash, controller, balance, status
				.collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		elections: Default::default(),
		council: CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		democracy: Default::default(),
		sudo: SudoConfig { key: Some(root_key.clone()) },
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(node_dfinn_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: Default::default(),
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		orml_vesting: OrmlVestingConfig { vesting },
		finn_migration: FINNMigrationConfig { max_tokens: ERC20_FINN_SUPPLY, operational: false },
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use sp_runtime::BuildStorage;

	use super::*;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			soba_testnet_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	fn test_create_development_chain_spec() {
		assert!(!development_config().build_storage().is_err());
	}

	#[test]
	fn test_create_soba_testnet_chain_spec() {
		assert!(!soba_testnet_config().build_storage().is_err());
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		assert!(!udon_testnet_config().build_storage().is_err());
	}
}
