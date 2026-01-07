//use input2.xsd  http://other.example.com;
#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = { "tns" = "http://example.com" })]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "a")]
    pub a: f64,
}

impl Validate for FooType {}


// pub type Foo = FooType;
