//! Deterministic component-wise provider resolution.

use std::{cmp::Ordering, collections::BTreeMap};

use shared_models::{BatteryComponent, ComponentType};

use crate::ProviderReport;

#[derive(Debug, Clone, Copy)]
struct Candidate<'a> {
    component: &'a BatteryComponent,
    report_priority: u16,
    provider_id: &'a str,
}

impl Candidate<'_> {
    fn compare(self, other: Self) -> Ordering {
        self.component
            .percentage
            .is_some()
            .cmp(&other.component.percentage.is_some())
            .then_with(|| (!self.component.stale).cmp(&(!other.component.stale)))
            .then_with(|| self.component.confidence.cmp(&other.component.confidence))
            .then_with(|| {
                self.component
                    .source
                    .priority()
                    .cmp(&other.component.source.priority())
            })
            .then_with(|| self.component.updated_at.cmp(&other.component.updated_at))
            .then_with(|| self.report_priority.cmp(&other.report_priority))
            .then_with(|| other.provider_id.cmp(self.provider_id))
    }
}

/// Resolves provider reports independently for each component type.
///
/// Reports for other device identifiers are ignored after the first report's
/// device id establishes the resolution group.
#[must_use]
pub fn resolve_reports(reports: &[ProviderReport]) -> Vec<BatteryComponent> {
    let Some(first) = reports.first() else {
        return Vec::new();
    };

    let mut selected: BTreeMap<ComponentType, Candidate<'_>> = BTreeMap::new();

    for report in reports
        .iter()
        .filter(|report| report.device_id == first.device_id)
    {
        for component in &report.components {
            let candidate = Candidate {
                component,
                report_priority: report.priority,
                provider_id: &report.provider_id,
            };
            match selected.get(&component.component_type).copied() {
                Some(current) if candidate.compare(current).is_le() => {}
                _ => {
                    selected.insert(component.component_type, candidate);
                }
            }
        }
    }

    selected
        .into_values()
        .map(|candidate| candidate.component.clone())
        .collect()
}
