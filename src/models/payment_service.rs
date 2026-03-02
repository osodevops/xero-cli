use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentService {
    #[serde(rename = "PaymentServiceID")]
    pub payment_service_id: Option<String>,
    #[serde(rename = "PaymentServiceName")]
    pub payment_service_name: Option<String>,
    #[serde(rename = "PaymentServiceUrl")]
    pub payment_service_url: Option<String>,
    #[serde(rename = "PaymentServiceType")]
    pub payment_service_type: Option<String>,
    #[serde(rename = "PayNowText")]
    pub pay_now_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentServicesWrapper {
    #[serde(rename = "PaymentServices")]
    pub payment_services: Vec<PaymentService>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_payment_service() {
        let json = r#"{"PaymentServiceID": "ps-1", "PaymentServiceName": "PayPal", "PaymentServiceType": "PAYPAL"}"#;
        let ps: PaymentService = serde_json::from_str(json).unwrap();
        assert_eq!(ps.payment_service_name.as_deref(), Some("PayPal"));
    }
    #[test]
    fn deserialize_payment_services_wrapper() {
        let json = r#"{"PaymentServices": [{"PaymentServiceID": "ps-1", "PaymentServiceName": "PayPal"}]}"#;
        let w: PaymentServicesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.payment_services.len(), 1);
    }
}
