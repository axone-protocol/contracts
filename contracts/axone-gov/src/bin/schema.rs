use axone_gov::contract::AxoneGov;
use cosmwasm_schema::remove_schemas;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().expect("Failed to get current directory");
    out_dir.push("schema");
    create_dir_all(&out_dir).expect("Failed to create schema directory");
    remove_schemas(&out_dir).expect("Failed to remove existing schemas");

    AxoneGov::export_schema(&out_dir);
}
