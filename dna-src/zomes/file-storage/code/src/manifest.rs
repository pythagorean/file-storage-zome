use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{
    dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Manifest {}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "manifest",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Manifest>| {
            Ok(())
        },

        links: [
        ]
    )
}
