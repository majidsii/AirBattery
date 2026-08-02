//! Routes raw backend observations through protocol providers and the registry.

use std::{collections::BTreeSet, sync::Arc};

use device_protocols::{DeviceProvider, ProviderError, RawObservation};
use thiserror::Error;

use crate::{DeviceRegistry, DeviceSnapshot, FreshnessPolicy};

/// Error raised while normalizing one raw backend observation.
#[derive(Debug, Error)]
pub enum ProcessingError {
    /// A provider recognized the observation but could not parse it safely.
    #[error("provider {provider_id} failed: {source}")]
    Provider {
        /// Stable provider identifier.
        provider_id: &'static str,
        /// Provider-specific parsing failure.
        #[source]
        source: ProviderError,
    },
    /// A registry snapshot could not be produced after applying an observation.
    #[error("device {device_id} was not present after observation processing")]
    MissingSnapshot {
        /// Backend device identifier.
        device_id: String,
    },
}

/// Event-driven protocol orchestration for one application process.
pub struct ObservationProcessor {
    providers: Vec<Arc<dyn DeviceProvider>>,
    registry: DeviceRegistry,
}

impl std::fmt::Debug for ObservationProcessor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ObservationProcessor")
            .field("provider_count", &self.providers.len())
            .field("registry", &self.registry)
            .finish()
    }
}

impl ObservationProcessor {
    /// Creates a processor with providers evaluated in deterministic insertion order.
    #[must_use]
    pub fn new(policy: FreshnessPolicy, providers: Vec<Arc<dyn DeviceProvider>>) -> Self {
        Self {
            providers,
            registry: DeviceRegistry::new(policy),
        }
    }

    /// Applies one raw observation and returns the current normalized device snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessingError::Provider`] when a matching provider cannot parse
    /// the observation, or [`ProcessingError::MissingSnapshot`] when processing
    /// does not produce a registry snapshot.
    pub async fn process(
        &mut self,
        observation: &RawObservation,
    ) -> Result<DeviceSnapshot, ProcessingError> {
        let (device_id, observed_at) = observation_identity(observation);

        if let RawObservation::Connection { state, .. } = observation {
            self.registry.set_connection_state(device_id, *state);
        } else {
            for provider in self
                .providers
                .iter()
                .filter(|provider| provider.supports(observation))
            {
                if let Some(report) = provider.parse(observation).await.map_err(|source| {
                    ProcessingError::Provider {
                        provider_id: provider.id(),
                        source,
                    }
                })? {
                    self.registry.apply_report(&report);
                }
            }
        }

        self.registry
            .snapshot(device_id, observed_at)
            .ok_or_else(|| ProcessingError::MissingSnapshot {
                device_id: device_id.to_owned(),
            })
    }

    /// Removes provider state for devices no longer reported by the platform catalog.
    pub fn retain_devices(&mut self, device_ids: &BTreeSet<String>) {
        self.registry.retain_devices(device_ids);
    }

    /// Returns the current read-only snapshots without mutating provider state.
    #[must_use]
    pub fn snapshots(&self, now: time::OffsetDateTime) -> Vec<DeviceSnapshot> {
        self.registry.snapshots(now)
    }
}

fn observation_identity(observation: &RawObservation) -> (&str, time::OffsetDateTime) {
    match observation {
        RawObservation::ManufacturerData {
            device_id,
            observed_at,
            ..
        }
        | RawObservation::VendorPacket {
            device_id,
            observed_at,
            ..
        }
        | RawObservation::ServiceData {
            device_id,
            observed_at,
            ..
        }
        | RawObservation::PlatformBattery {
            device_id,
            observed_at,
            ..
        }
        | RawObservation::StandardBattery {
            device_id,
            observed_at,
            ..
        }
        | RawObservation::Connection {
            device_id,
            observed_at,
            ..
        } => (device_id, *observed_at),
    }
}
