//! Application-level catalog joining backend metadata with normalized battery state.

mod classification;

pub use classification::{ClassificationInput, DeviceClassification, classify_device};

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use airbattery_core::{FreshnessPolicy, ObservationProcessor, ProcessingError};
use device_protocols::{DeviceProvider, RawObservation};
use shared_models::{BluetoothAudioDevice, Capability, ComponentType, DeviceFamily, Transport};
use time::OffsetDateTime;

/// Operating-system metadata retained separately from protocol state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    /// Identifier used only to correlate platform observations.
    pub backend_id: String,
    /// Privacy-safe identifier exposed outside the service boundary.
    pub privacy_id: String,
    /// User-visible device name.
    pub display_name: String,
    /// Operating-system name, when different.
    pub system_name: Option<String>,
    /// Manufacturer when reliably identified.
    pub manufacturer: Option<String>,
    /// Model when reliably identified.
    pub model: Option<String>,
    /// Platform icon hint, such as `input-mouse` or `audio-headset`.
    pub icon: Option<String>,
    /// Bluetooth Class of Device.
    pub class: Option<u32>,
    /// Bluetooth SIG appearance value.
    pub appearance: Option<u16>,
    /// Lowercase service UUIDs used only for classification and provider routing.
    pub service_uuids: Vec<String>,
    /// Broad presentation family.
    pub device_family: DeviceFamily,
    /// Bluetooth transport reported by the platform.
    pub transport: Transport,
    /// Most recent platform observation time.
    pub last_seen_at: Option<OffsetDateTime>,
}

/// Stateful normalization engine shared by desktop and shell integrations.
#[derive(Debug)]
pub struct ApplicationEngine {
    processor: ObservationProcessor,
    descriptors: BTreeMap<String, DeviceDescriptor>,
}

impl ApplicationEngine {
    /// Creates an engine with the default freshness policy.
    #[must_use]
    pub fn new(providers: Vec<Arc<dyn DeviceProvider>>) -> Self {
        Self::with_policy(FreshnessPolicy::default(), providers)
    }

    /// Creates an engine with an explicit freshness policy.
    #[must_use]
    pub fn with_policy(policy: FreshnessPolicy, providers: Vec<Arc<dyn DeviceProvider>>) -> Self {
        Self {
            processor: ObservationProcessor::new(policy, providers),
            descriptors: BTreeMap::new(),
        }
    }

    /// Inserts or updates platform metadata for a device.
    pub fn upsert_descriptor(&mut self, descriptor: DeviceDescriptor) {
        self.descriptors
            .insert(descriptor.backend_id.clone(), descriptor);
    }

    /// Reconciles descriptors and provider state with one authoritative platform catalog.
    pub fn retain_descriptors(&mut self, backend_ids: &BTreeSet<String>) {
        self.descriptors
            .retain(|backend_id, _descriptor| backend_ids.contains(backend_id));
        self.processor.retain_devices(backend_ids);
    }

    /// Routes one raw observation and returns a public device when metadata is known.
    ///
    /// # Errors
    ///
    /// Returns a `ProcessingError` when the observation cannot be processed
    /// or converted into a normalized device snapshot.
    pub async fn process(
        &mut self,
        observation: &RawObservation,
    ) -> Result<Option<BluetoothAudioDevice>, ProcessingError> {
        let snapshot = self.processor.process(observation).await?;
        Ok(self
            .descriptors
            .get(&snapshot.device_id)
            .map(|descriptor| assemble_device(descriptor, snapshot)))
    }

    /// Returns all public devices with freshness evaluated at `now`.
    #[must_use]
    pub fn devices(&self, now: OffsetDateTime) -> Vec<BluetoothAudioDevice> {
        self.processor
            .snapshots(now)
            .into_iter()
            .filter_map(|snapshot| {
                self.descriptors
                    .get(&snapshot.device_id)
                    .map(|descriptor| assemble_device(descriptor, snapshot))
            })
            .collect()
    }
}

fn assemble_device(
    descriptor: &DeviceDescriptor,
    snapshot: airbattery_core::DeviceSnapshot,
) -> BluetoothAudioDevice {
    let mut capabilities = BTreeSet::new();
    for component in &snapshot.components {
        match component.component_type {
            ComponentType::Left | ComponentType::Right => {
                capabilities.insert(Capability::EarbudBattery);
            }
            ComponentType::Case => {
                capabilities.insert(Capability::CaseBattery);
            }
            ComponentType::Headset | ComponentType::Aggregate | ComponentType::Unknown => {
                capabilities.insert(Capability::AggregateBattery);
            }
        }
        if !matches!(
            component.charging_state,
            shared_models::ChargingState::Unknown
        ) {
            capabilities.insert(Capability::ChargingState);
        }
    }

    if matches!(descriptor.device_family, DeviceFamily::AirPods) {
        capabilities.extend([
            Capability::EarbudBattery,
            Capability::CaseBattery,
            Capability::ChargingState,
        ]);
    }

    let last_updated_at = snapshot
        .components
        .iter()
        .filter(|component| component.percentage.is_some())
        .map(|component| component.updated_at)
        .max();

    let classification = classify_device(ClassificationInput {
        name: &descriptor.display_name,
        manufacturer: descriptor.manufacturer.as_deref(),
        model: descriptor.model.as_deref(),
        icon: descriptor.icon.as_deref(),
        class: descriptor.class,
        appearance: descriptor.appearance,
        service_uuids: &descriptor.service_uuids,
        family_hint: descriptor.device_family,
    });

    BluetoothAudioDevice {
        id: descriptor.privacy_id.clone(),
        display_name: descriptor.display_name.clone(),
        system_name: descriptor.system_name.clone(),
        manufacturer: descriptor.manufacturer.clone(),
        model: descriptor.model.clone(),
        device_family: classification.family,
        visual: classification.visual,
        transport: descriptor.transport,
        connection_state: snapshot.connection_state,
        last_seen_at: descriptor.last_seen_at,
        last_updated_at,
        capabilities,
        components: snapshot.components,
    }
}
