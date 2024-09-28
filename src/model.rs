
#[derive(Clone, Debug, Serialize, Deserialize)]
#[ormx(table = "feature_attributes", id = objectid, insertable)]
pub struct FeatureAttributes {
    #[serde(rename = "OBJECTID")]
    #[ormx(column = "OBJECTID")]
    objectid: i64,
    #[serde(rename = "CodNatureza")]
    #[ormx(column = "OBJECTID")]
    objectid: i64,
}


#[derive(Debug, Serialize, Deserialize)]
struct Fires {
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Feature {
    attributes: FeatureAttributes,
    #[serde(rename = "exceededTransferLimit")]
    exceeded_transfer_limit: Option<bool>,
}
