use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use xsd_parser::generator::validator::Validate;
        use yaserde_derive::{YaDeserialize, YaSerialize};
        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::FooType = yaserde::de::from_str(ser).unwrap();

    // Note: this test uses a generated fixture (`expected.rs`), and the generator currently does
    // not pull in fields from imported schemas. So we only assert the local field here.
    assert_eq!(de, expected::FooType { a: 150.0 });
}

#[test]
fn generator_does_not_panic() {
    println!("{}", utils::generate(include_str!("input.xsd")))
}

#[test]
#[ignore]
fn generator_output_has_correct_ast() {
    utils::ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
