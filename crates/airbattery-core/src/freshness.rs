//! Freshness and staleness policy.

use shared_models::{BatteryComponent, ComponentType, ConnectionState};
use time::{Duration, OffsetDateTime};

/// Time-to-stale policy for battery components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshnessPolicy {
    /// Case values are short-lived because the case advertises intermittently.
    pub case_ttl: Duration,
    /// Earbud component freshness.
    pub earbud_ttl: Duration,
    /// Aggregate and headset freshness.
    pub aggregate_ttl: Duration,
}

impl Default for FreshnessPolicy {
    fn default() -> Self {
        Self {
            case_ttl: Duration::seconds(90),
            earbud_ttl: Duration::minutes(5),
            aggregate_ttl: Duration::minutes(5),
        }
    }
}

impl FreshnessPolicy {
    fn ttl_for(self, component_type: ComponentType) -> Duration {
        match component_type {
            ComponentType::Case => self.case_ttl,
            ComponentType::Left | ComponentType::Right => self.earbud_ttl,
            ComponentType::Headset | ComponentType::Aggregate | ComponentType::Unknown => {
                self.aggregate_ttl
            }
        }
    }
}

/// Applies staleness rules without deleting the last known value.
#[must_use]
pub fn expire_components(
    components: &[BatteryComponent],
    now: OffsetDateTime,
    connection_state: ConnectionState,
    policy: FreshnessPolicy,
) -> Vec<BatteryComponent> {
    components
        .iter()
        .cloned()
        .map(|mut component| {
            let age = now - component.updated_at;
            let expired = age > policy.ttl_for(component.component_type);
            component.stale = component.stale
                || expired
                || matches!(connection_state, ConnectionState::Disconnected);
            component
        })
        .collect()
}
