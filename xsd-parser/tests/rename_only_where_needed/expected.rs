#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = { "tns" = "http://example.com" })]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "min")]
    pub min: i32,

    #[yaserde(prefix = "tns", rename = "max")]
    pub max: i32,
}

impl Validate for FooType {}


// pub type Foo = FooType;
