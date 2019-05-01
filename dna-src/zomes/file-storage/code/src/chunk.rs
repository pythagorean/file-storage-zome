use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{
    dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Chunk {}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "chunk",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Chunk>| {
            Ok(())
        },

        links: [
        ]
    )
}
