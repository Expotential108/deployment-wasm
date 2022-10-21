
use std::{error::Error, fmt::{Debug, Display}};
use crate::wasm_cli::{wasm_cli_instantiate, wasm_cli_execute, wasm_cli_migrate, wasm_cli_query};

pub trait Contract: Send + Sync + Debug + From<String> + Clone + 'static {
    fn name(&self)                     -> String;
    fn admin(&self)                    -> String;
    fn instantiate_msg(&self)          -> Result<String, Box<dyn Error>>;
    fn base_config_msg(&self)          -> Result<String, Box<dyn Error>>;
    fn execute_msg(&self)              -> Result<String, Box<dyn Error>>;
    fn query_msg(&self)                -> Result<String, Box<dyn Error>>;
    fn setup_msgs(&self)               -> Result<Vec<String>, Box<dyn Error>>;
}

pub fn instantiate(contract: &impl Contract) -> Result<(), Box<dyn Error>> {
    wasm_cli_instantiate(&contract.admin(), &contract.name(), &contract.instantiate_msg()?)
}
pub fn migrate(contract: &impl Contract) -> Result<(), Box<dyn Error>> {
    wasm_cli_migrate(&contract.name(), &contract.instantiate_msg()?)
}
pub fn set_config(contract: &impl Contract) -> Result<(), Box<dyn Error>> {
    wasm_cli_execute(&contract.name(), &contract.base_config_msg()?)
}
pub fn execute_payload(contract: &impl Contract, payload: &String) -> Result<(), Box<dyn Error>> {
    wasm_cli_execute(&contract.name(), payload)
}
pub fn execute_set_up(contract: &impl Contract, msgs: &Vec<String>) -> Result<(), Box<dyn Error>> {
    for msg in msgs {
        wasm_cli_execute(&contract.name(), msg)?;
    }
    Ok(())   
}

pub trait Execute: Display {
    fn execute_msg(&self) -> Result<String, Box<dyn Error>>;
}

pub fn execute(contract: &impl Execute) -> Result<(), Box<dyn Error>> {
    wasm_cli_execute(&contract.to_string(), &contract.execute_msg()?)
}

pub trait Query: Display {
    fn query_msg(&self) -> Result<String, Box<dyn Error>>;
}

pub fn query(contract: &impl Query) -> Result<(), Box<dyn Error>> {
    wasm_cli_query(&contract.to_string(), &contract.query_msg()?)
}

// #[derive(Parser, Debug, Display, Clone)]
// pub enum Msgs<E, Q> 
// where E: Subcommand + Execute,
//       Q: Subcommand + Query   
// {
//     #[command(subcommand)]
//     Execute(E),
//     #[command(subcommand)]
//     Query(Q),
//     Empty
// }

// impl<E, Q> Default for Msgs<E, Q>
// where E: Subcommand,
//       Q: Subcommand
// {
//     fn default() -> Self {
//         Msgs::Empty
//     }
// }