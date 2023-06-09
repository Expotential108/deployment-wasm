use std::error::Error;

use cosm_tome::{chain::response::ChainResponse, modules::cosmwasm::error::CosmwasmError};
use inquire::InquireError;
use interactive_parse::error::SchemaError;
#[cfg(feature = "ledger")]
use ledger_utility::error::LedgerUtilityError;
use thiserror::Error;

pub type DeployResult<T> = core::result::Result<T, DeployError>;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("{0}")]
    Error(String),

    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    Keyring(#[from] keyring::Error),

    #[cfg(feature = "ledger")]
    #[error("{0}")]
    LedgerUtilityError(#[from] LedgerUtilityError),

    #[error("invalid admin address")]
    AdminAddress,

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Cosmwasm(#[from] CosmwasmError),

    #[error("{0}")]
    InteractiveParse(#[from] SchemaError),

    #[error("{0}")]
    Std(#[from] Box<dyn Error>),

    #[error("{0}")]
    Inquire(#[from] InquireError),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),

    #[error("{0}")]
    Clap(#[from] clap::error::Error),

    #[error("invalid mnemonic")]
    Mnemonic,

    #[error("invalid derivation path")]
    DerivationPath,

    #[error("Account id error")]
    AccountId { id: String },

    #[error("Cosmos Sdk Error {:?}", res)]
    CosmosSdk { res: ChainResponse },

    #[error("Unsupported shell, must use bash or zsh")]
    UnsupportedShell,

    #[error("Chain already exists")]
    ChainAlreadyExists,

    #[error("Contract already exists")]
    ContractAlreadyExists,

    #[error("Contract not found")]
    ContractNotFound,

    #[error("Env already exists")]
    EnvAlreadyExists,

    #[error("Invalid directory")]
    InvalidDir,

    #[error("Contract does not have an address")]
    NoAddr,

    #[error("Error parsing chain")]
    ChainId { chain_id: String },

    #[error("Error parsing denom")]
    Denom { name: String },

    #[error("Empty response")]
    EmptyResponse,

    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("Key not found")]
    KeyNotFound { key_name: String },

    #[error("Code id not found")]
    CodeIdNotFound,

    #[error("Env not found")]
    EnvNotFound,

    #[error("Contract address not found for {name}, perhaps you need to instantiate first?")]
    AddrNotFound { name: String },

    #[error(
        "{} Config file not found, perhaps you need to run \"deploy init\"?",
        "Deploy Error"
    )]
    ConfigNotFound {},

    #[error("Invalid derivation path.")]
    DerviationPath,

    #[error(
        "Both gRPC endpoint and RPC endpoint cannot be null.\
        Update you ChainInfo to add at least one endpoint"
    )]
    MissingClient,

    #[error(
        "The current version of wasm-deploy requires the gRPC endpoint.\
        Update you ChainInfo to include the endpoint address"
    )]
    MissingGRpc,
}
