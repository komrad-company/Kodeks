use std::time::{Duration, SystemTime, UNIX_EPOCH};

use kompiler::RuleLevel;
use serde_json::json;

use crate::alert::Alert;

#[test]
fn timestamp_computed_correctly_from_system_time() {
    let time = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let alert = Alert::new_at(
        "rule-001".to_string(),
        "Test alert".to_string(),
        &RuleLevel::High,
        json!({}),
        time,
    );
    assert_eq!(alert.timestamp_unix, 1_700_000_000);
}

#[test]
fn level_stored_as_rule_level_variant() {
    let alert = Alert::new_at(
        "rule-001".to_string(),
        "Test alert".to_string(),
        &RuleLevel::Critical,
        json!({}),
        SystemTime::now(),
    );
    assert_eq!(alert.level, RuleLevel::Critical);
}

#[test]
fn pre_epoch_time_sets_timestamp_to_zero() {
    let before_epoch = UNIX_EPOCH - Duration::from_secs(1);
    let alert = Alert::new_at(
        "rule-001".to_string(),
        "Test alert".to_string(),
        &RuleLevel::Low,
        json!({}),
        before_epoch,
    );
    assert_eq!(alert.timestamp_unix, 0);
}
