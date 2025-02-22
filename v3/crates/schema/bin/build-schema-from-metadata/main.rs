/// This is a small command-line utility that prints out the GraphQL schemas that result from an
/// engine-metadata value.
///
/// For examples of metadata files, see the 'tests/' folder in this crate.
use std::{
    collections::BTreeSet,
    io::{stdin, Read},
    process::exit,
};

use hasura_authn_core::Role;
use schema::{GDSNamespaceGetterAgnostic, GDSRoleNamespaceGetter};

pub fn main() {
    let mut metadata_string = String::new();
    let mut h = stdin();

    h.read_to_string(&mut metadata_string).unwrap();

    if metadata_string.is_empty() {
        println!("Usage: Provide a metadata json file via stdin and get the corresponding sdl schema and introspection query results on stdout.");
        exit(-1);
    }

    let metadata =
        open_dds::traits::OpenDd::deserialize(serde_json::from_str(&metadata_string).unwrap())
            .unwrap();
    let gds = schema::GDS::new_with_default_flags(metadata).unwrap();
    let sch = gds.build_schema().unwrap();

    let dedup_roles: BTreeSet<Role> = gds.metadata.roles.into_iter().collect();

    for role in dedup_roles {
        println!("-------------------------------:");
        println!("Now comes the SDL schema for {role}:");
        println!("-------------------------------:");
        println!();
        println!("{}", sch.generate_sdl(&GDSRoleNamespaceGetter, &role));
        println!();
        println!();
        println!();
    }

    println!("---------------------------------------:");
    println!("Now comes the role-agnostic SDL schema :");
    println!("---------------------------------------:");
    println!();
    println!(
        "{}",
        sch.generate_sdl(
            &GDSNamespaceGetterAgnostic,
            &Role("this value is irrelevant".to_string())
        )
    );
    println!();
    println!();
    println!();
}
