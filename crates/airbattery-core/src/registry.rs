//! In-memory normalized device registry.

use std::collections::{BTreeMap, BTreeSet};

use device_protocols::{ProviderReport, resolve_reports};
use shared_models::{BatteryComponent, ComponentType, ConnectionState};
use time::OffsetDateTime;

use crate::{FreshnessPolicy, expire_components};

#[derive(Debug, Default)]
struct ProviderState {
    priority: u16,
    observed_at: Option<OffsetDateTime>,
    components: BTreeMap<ComponentType, BatteryComponent>,
}

impl ProviderState {
    fn apply(&mut self, report: &ProviderReport) -> bool {
        let mut changed = self.priority != report.priority;
        self.priority = report.priority;
        self.observed_at = Some(self.observed_at.map_or(report.observed_at, |current| {
            current.max(report.observed_at)
        }));

        for incoming in &report.components {
            let should_replace = match self.components.get(&incoming.component_type) {
                Some(existing) if incoming.updated_at < existing.updated_at => false,
                Some(existing)
                    if existing.percentage.is_some() && incoming.percentage.is_none() =>
                {
                    // A temporarily absent value must not erase the last known value.
                    // Its original timestamp remains unchanged so normal TTL expiry applies.
                    false
                }
                Some(existing) => existing != incoming,
                None => true,
            };

            if should_replace {
                self.components
                    .insert(incoming.component_type, incoming.clone());
                changed = true;
            }
        }
        changed
    }

    fn as_report(&self, provider_id: &str, device_id: &str) -> Option<ProviderReport> {
        Some(ProviderReport::new(
            provider_id,
            device_id,
            self.priority,
            self.observed_at?,
            self.components.values().cloned().collect(),
        ))
    }
}

#[derive(Debug, Default)]
struct DeviceState {
    providers: BTreeMap<String, ProviderState>,
    connection_state: ConnectionState,
}

/// Read-only user-visible registry snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSnapshot {
    /// Backend device identifier.
    pub device_id: String,
    /// Current connection state.
    pub connection_state: ConnectionState,
    /// Resolved and freshness-adjusted components.
    pub components: Vec<BatteryComponent>,
}

/// Registry that keeps the latest known component per provider and device.
#[derive(Debug)]
pub struct DeviceRegistry {
    policy: FreshnessPolicy,
    devices: BTreeMap<String, DeviceState>,
}

impl DeviceRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new(policy: FreshnessPolicy) -> Self {
        Self {
            policy,
            devices: BTreeMap::new(),
        }
    }

    /// Applies a provider report component by component.
    ///
    /// An unavailable component does not erase a previously known value. The
    /// known value keeps its original timestamp and therefore becomes stale at
    /// the normal policy boundary.
    pub fn apply_report(&mut self, report: &ProviderReport) -> bool {
        self.devices
            .entry(report.device_id.clone())
            .or_default()
            .providers
            .entry(report.provider_id.clone())
            .or_default()
            .apply(report)
    }

    /// Updates connection state independently from battery reports.
    pub fn set_connection_state(&mut self, device_id: &str, state: ConnectionState) {
        self.devices
            .entry(device_id.to_owned())
            .or_default()
            .connection_state = state;
    }

    /// Removes devices that are no longer present in the authoritative platform catalog.
    pub fn retain_devices(&mut self, device_ids: &BTreeSet<String>) {
        self.devices
            .retain(|device_id, _state| device_ids.contains(device_id));
    }

    /// Builds a current snapshot for a known device.
    #[must_use]
    pub fn snapshot(&self, device_id: &str, now: OffsetDateTime) -> Option<DeviceSnapshot> {
        let state = self.devices.get(device_id)?;
        let reports: Vec<_> = state
            .providers
            .iter()
            .filter_map(|(provider_id, provider)| provider.as_report(provider_id, device_id))
            .collect();
        let resolved = resolve_reports(&reports);
        Some(DeviceSnapshot {
            device_id: device_id.to_owned(),
            connection_state: state.connection_state,
            components: expire_components(&resolved, now, state.connection_state, self.policy),
        })
    }

    /// Returns snapshots for all known devices.
    #[must_use]
    pub fn snapshots(&self, now: OffsetDateTime) -> Vec<DeviceSnapshot> {
        self.devices
            .keys()
            .filter_map(|device_id| self.snapshot(device_id, now))
            .collect()
    }
}
