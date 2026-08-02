//! Generic aggregate and Bluetooth SIG Battery Service normalization.

use async_trait::async_trait;
use device_protocols::{DeviceProvider, ProviderError, ProviderReport, RawObservation};
use shared_models::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, DataConfidence, DataSource,
    InvalidBatteryPercentage,
};
use time::OffsetDateTime;

/// Validation error for generic battery input.
pub type GenericBatteryError = InvalidBatteryPercentage;

fn report(
    provider_id: &'static str,
    device_id: &str,
    percentage: u8,
    observed_at: OffsetDateTime,
    component_type: ComponentType,
    source: DataSource,
    confidence: DataConfidence,
) -> Result<ProviderReport, GenericBatteryError> {
    let percentage = BatteryPercentage::new(percentage)?;
    Ok(ProviderReport::new(
        provider_id,
        device_id,
        30,
        observed_at,
        vec![BatteryComponent {
            component_type,
            percentage: Some(percentage),
            charging_state: ChargingState::Unknown,
            updated_at: observed_at,
            source,
            confidence,
            stale: false,
        }],
    ))
}

/// Normalizes an operating-system aggregate battery value.
///
/// # Errors
///
/// Returns [`GenericBatteryError`] when `percentage` is outside the valid
/// battery range of 0 through 100.
pub fn normalize_platform_battery(
    device_id: &str,
    percentage: u8,
    observed_at: OffsetDateTime,
) -> Result<ProviderReport, GenericBatteryError> {
    normalize_platform_battery_from(
        device_id,
        percentage,
        observed_at,
        DataSource::AggregatePlatform,
    )
}

/// Normalizes a platform battery value while preserving its exact source.
///
/// # Errors
///
/// Returns [`GenericBatteryError`] when `percentage` is outside the valid
/// battery range of 0 through 100.
pub fn normalize_platform_battery_from(
    device_id: &str,
    percentage: u8,
    observed_at: OffsetDateTime,
    source: DataSource,
) -> Result<ProviderReport, GenericBatteryError> {
    report(
        "platform-aggregate",
        device_id,
        percentage,
        observed_at,
        ComponentType::Aggregate,
        source,
        DataConfidence::Medium,
    )
}

/// Normalizes characteristic `0x2A19` from the standard Battery Service.
///
/// # Errors
///
/// Returns [`GenericBatteryError`] when `percentage` is outside the valid
/// battery range of 0 through 100.
pub fn normalize_standard_battery(
    device_id: &str,
    percentage: u8,
    observed_at: OffsetDateTime,
) -> Result<ProviderReport, GenericBatteryError> {
    report(
        "standard-battery-service",
        device_id,
        percentage,
        observed_at,
        ComponentType::Headset,
        DataSource::StandardBleBattery,
        DataConfidence::Verified,
    )
}

/// Provider for platform aggregate and standard Battery Service observations.
#[derive(Debug, Default, Clone, Copy)]
pub struct GenericBatteryProvider;

#[async_trait]
impl DeviceProvider for GenericBatteryProvider {
    fn id(&self) -> &'static str {
        "generic-battery"
    }

    fn supports(&self, observation: &RawObservation) -> bool {
        matches!(
            observation,
            RawObservation::PlatformBattery { .. } | RawObservation::StandardBattery { .. }
        )
    }

    async fn parse(
        &self,
        observation: &RawObservation,
    ) -> Result<Option<ProviderReport>, ProviderError> {
        let report = match observation {
            RawObservation::PlatformBattery {
                device_id,
                percentage,
                source,
                observed_at,
            } => normalize_platform_battery_from(device_id, *percentage, *observed_at, *source),
            RawObservation::StandardBattery {
                device_id,
                percentage,
                observed_at,
            } => normalize_standard_battery(device_id, *percentage, *observed_at),
            _ => return Ok(None),
        }
        .map_err(|error| ProviderError::InvalidObservation(error.to_string()))?;
        Ok(Some(report))
    }
}
