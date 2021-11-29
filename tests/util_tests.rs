use binance::util::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};
    use float_cmp::*;

    #[test]
    fn build_request_empty() {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result = build_request(parameters);
        assert!(result.is_empty());
    }

    #[test]
    fn build_request_not_empty() {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("recvWindow".into(), "1234".to_string());
        let result = build_request(parameters);
        assert_eq!(result, format!("recvWindow={}", 1234));
    }

    #[test]
    fn build_signed_request() {
        let now = SystemTime::now();
        let recv_window = 1234;

        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000;

        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result =
            binance::util::build_signed_request_custom(parameters, recv_window, now).unwrap();

        assert_eq!(
            result,
            format!("recvWindow={}&timestamp={}", recv_window, timestamp)
        );
    }

    #[test]
    fn to_i64() {
        let value_max = serde_json::json!(i64::MAX);
        let value_min = serde_json::json!(i64::MIN);
        assert_eq!(binance::util::to_i64(&value_max), i64::MAX);
        assert_eq!(binance::util::to_i64(&value_min), i64::MIN);
    }

    #[test]
    fn to_f64() {
        let value = serde_json::json!("123.3");
        assert!(approx_eq!(
            f64,
            binance::util::to_f64(&value),
            123.3,
            ulps = 2
        ));
    }
}
