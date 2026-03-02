use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingCategory {
    #[serde(rename = "TrackingCategoryID")]
    pub tracking_category_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<TrackingOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingOption {
    #[serde(rename = "TrackingOptionID")]
    pub tracking_option_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingCategoriesWrapper {
    #[serde(rename = "TrackingCategories")]
    pub tracking_categories: Vec<TrackingCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingOptionsWrapper {
    #[serde(rename = "Options")]
    pub options: Vec<TrackingOption>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tracking_category() {
        let json = r#"{
            "TrackingCategoryID": "tc-123",
            "Name": "Region",
            "Status": "ACTIVE",
            "Options": [
                {"TrackingOptionID": "opt-1", "Name": "North", "Status": "ACTIVE"},
                {"TrackingOptionID": "opt-2", "Name": "South", "Status": "ACTIVE"}
            ]
        }"#;
        let tc: TrackingCategory = serde_json::from_str(json).unwrap();
        assert_eq!(tc.tracking_category_id.as_deref(), Some("tc-123"));
        assert_eq!(tc.name.as_deref(), Some("Region"));
        assert_eq!(tc.options.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_tracking_categories_wrapper() {
        let json = r#"{
            "TrackingCategories": [
                {
                    "TrackingCategoryID": "tc-1",
                    "Name": "Department",
                    "Status": "ACTIVE"
                }
            ]
        }"#;
        let wrapper: TrackingCategoriesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.tracking_categories.len(), 1);
    }

    #[test]
    fn deserialize_tracking_option() {
        let json = r#"{
            "TrackingOptionID": "opt-1",
            "Name": "North",
            "Status": "ACTIVE"
        }"#;
        let opt: TrackingOption = serde_json::from_str(json).unwrap();
        assert_eq!(opt.tracking_option_id.as_deref(), Some("opt-1"));
        assert_eq!(opt.name.as_deref(), Some("North"));
    }
}
