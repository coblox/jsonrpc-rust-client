#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "1.0")]
    V1,
    // At the moment, only version 1 is supported.
    //    #[serde(rename = "2.0")]
    //    V2,
}
