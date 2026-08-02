//! `AirPods` providers for passive advertisements and active accessory packets.

use async_trait::async_trait;
use device_protocols::{
    DeviceProvider, ProviderError, ProviderReport, RawObservation, VendorProtocol,
};
use shared_models::{BatteryComponent, ComponentType, DataConfidence, DataSource};

use crate::{
    APPLE_MANUFACTURER_ID, AirPodsComponent, PROXIMITY_PAIRING_TYPE, parse_accessory_battery,
    parse_proximity_pairing,
};

const PASSIVE_PAYLOAD_LENGTH: usize = 27;

/// Provider for Apple Continuity proximity-pairing advertisements.
#[derive(Debug, Default, Clone, Copy)]
pub struct AirPodsProvider;

/// Provider for exact `AirPods` battery notifications over active accessory transport.
#[derive(Debug, Default, Clone, Copy)]
pub struct AirPodsAccessoryProvider;

fn normalized_component(
    component_type: ComponentType,
    component: AirPodsComponent,
    observed_at: time::OffsetDateTime,
    source: DataSource,
    confidence: DataConfidence,
) -> BatteryComponent {
    BatteryComponent {
        component_type,
        percentage: component.percentage,
        charging_state: component.charging_state,
        updated_at: observed_at,
        source,
        confidence,
        stale: false,
    }
}

#[async_trait]
impl DeviceProvider for AirPodsProvider {
    fn id(&self) -> &'static str {
        "airpods-proximity-pairing"
    }

    fn supports(&self, observation: &RawObservation) -> bool {
        matches!(
            observation,
            RawObservation::ManufacturerData {
                company_id: APPLE_MANUFACTURER_ID,
                payload,
                ..
            } if payload.first().copied() == Some(PROXIMITY_PAIRING_TYPE)
                && payload.len() >= PASSIVE_PAYLOAD_LENGTH
        )
    }

    async fn parse(
        &self,
        observation: &RawObservation,
    ) -> Result<Option<ProviderReport>, ProviderError> {
        let RawObservation::ManufacturerData {
            device_id,
            company_id,
            payload,
            observed_at,
        } = observation
        else {
            return Ok(None);
        };
        if *company_id != APPLE_MANUFACTURER_ID
            || payload.first().copied() != Some(PROXIMITY_PAIRING_TYPE)
            || payload.len() < PASSIVE_PAYLOAD_LENGTH
        {
            return Ok(None);
        }
        let Some(parsed) = parse_proximity_pairing(payload)
            .map_err(|error| ProviderError::InvalidObservation(error.to_string()))?
        else {
            return Ok(None);
        };

        Ok(Some(ProviderReport::new(
            self.id(),
            device_id,
            100,
            *observed_at,
            vec![
                normalized_component(
                    ComponentType::Left,
                    parsed.left,
                    *observed_at,
                    DataSource::AirPodsAdvertisement,
                    DataConfidence::Medium,
                ),
                normalized_component(
                    ComponentType::Right,
                    parsed.right,
                    *observed_at,
                    DataSource::AirPodsAdvertisement,
                    DataConfidence::Medium,
                ),
                normalized_component(
                    ComponentType::Case,
                    parsed.case,
                    *observed_at,
                    DataSource::AirPodsAdvertisement,
                    DataConfidence::Medium,
                ),
            ],
        )))
    }
}

#[async_trait]
impl DeviceProvider for AirPodsAccessoryProvider {
    fn id(&self) -> &'static str {
        "airpods-accessory-battery"
    }

    fn supports(&self, observation: &RawObservation) -> bool {
        matches!(
            observation,
            RawObservation::VendorPacket {
                protocol: VendorProtocol::AppleAccessory,
                ..
            }
        )
    }

    async fn parse(
        &self,
        observation: &RawObservation,
    ) -> Result<Option<ProviderReport>, ProviderError> {
        let RawObservation::VendorPacket {
            device_id,
            protocol: VendorProtocol::AppleAccessory,
            payload,
            observed_at,
        } = observation
        else {
            return Ok(None);
        };
        let Some(parsed) = parse_accessory_battery(payload)
            .map_err(|error| ProviderError::InvalidObservation(error.to_string()))?
        else {
            return Ok(None);
        };

        let mut components = Vec::with_capacity(3);
        for (component_type, component) in [
            (ComponentType::Left, parsed.left),
            (ComponentType::Right, parsed.right),
            (ComponentType::Case, parsed.case),
        ] {
            if let Some(component) = component {
                components.push(normalized_component(
                    component_type,
                    component,
                    *observed_at,
                    DataSource::VendorProtocol,
                    DataConfidence::Verified,
                ));
            }
        }

        Ok(Some(ProviderReport::new(
            self.id(),
            device_id,
            300,
            *observed_at,
            components,
        )))
    }
}
