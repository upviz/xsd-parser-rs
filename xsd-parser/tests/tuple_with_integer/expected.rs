#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType (pub xsd_types::types::Integer);

impl Validate for FooType {}
// pub type Foo = FooType;
