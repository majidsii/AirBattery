//! Classification tests for universal Bluetooth presentation metadata.

use airbattery_service::{ClassificationInput, classify_device};
use shared_models::{DeviceFamily, VisualConfidence};

fn classify(name: &str) -> airbattery_service::DeviceClassification {
    classify_device(ClassificationInput {
        name,
        manufacturer: None,
        model: None,
        icon: None,
        class: None,
        appearance: None,
        service_uuids: &[],
        family_hint: DeviceFamily::Unknown,
    })
}

#[test]
fn classifies_known_audio_models_and_category_fallbacks() {
    let airpods = classify_device(ClassificationInput {
        name: "SHABIN",
        manufacturer: Some("Apple"),
        model: Some("AirPods Pro (1st generation)"),
        icon: None,
        class: None,
        appearance: None,
        service_uuids: &[],
        family_hint: DeviceFamily::AirPods,
    });
    assert_eq!(airpods.family, DeviceFamily::AirPods);
    assert_eq!(airpods.visual.key, "airpods-pro-1");
    assert_eq!(airpods.visual.confidence, VisualConfidence::Exact);

    let galaxy = classify("Galaxy Buds2 Pro");
    assert_eq!(galaxy.family, DeviceFamily::Earbuds);
    assert_eq!(galaxy.visual.key, "galaxy-buds2");

    let soundcore = classify("soundcore Liberty 5");
    assert_eq!(soundcore.family, DeviceFamily::Earbuds);
    assert_eq!(soundcore.visual.key, "soundcore-liberty");

    let sony = classify("WH-1000XM5");
    assert_eq!(sony.family, DeviceFamily::Headset);
    let jbl = classify("JBL Charge 5");
    assert_eq!(jbl.family, DeviceFamily::Speaker);
}

#[test]
fn classifies_input_devices_and_unknown_fallback() {
    assert_eq!(classify("MX Master 3S").family, DeviceFamily::Mouse);
    assert_eq!(classify("Keychron K2").family, DeviceFamily::Keyboard);
    assert_eq!(
        classify("Xbox Wireless Controller").family,
        DeviceFamily::GameController
    );
    assert_eq!(classify("Apple Pencil").family, DeviceFamily::Stylus);

    let unknown = classify("Accessory 42");
    assert_eq!(unknown.family, DeviceFamily::Unknown);
    assert_eq!(unknown.visual.key, "bluetooth");
    assert_eq!(unknown.visual.confidence, VisualConfidence::Fallback);
}

#[test]
fn bluez_icon_and_appearance_are_used_when_names_are_ambiguous() {
    let mouse = classify_device(ClassificationInput {
        name: "BT Device",
        manufacturer: None,
        model: None,
        icon: Some("input-mouse"),
        class: None,
        appearance: Some(0x03c2),
        service_uuids: &["00001812-0000-1000-8000-00805f9b34fb".to_string()],
        family_hint: DeviceFamily::Unknown,
    });
    assert_eq!(mouse.family, DeviceFamily::Mouse);
    assert_eq!(mouse.visual.key, "mouse-generic");
    assert_eq!(mouse.visual.confidence, VisualConfidence::Category);
}

#[test]
fn classifies_model_aware_earbud_artwork_across_major_brands() {
    let fixtures = [
        ("AirPods 4", "airpods-4"),
        ("QCY T13 ANC 2", "qcy-t13-anc-2"),
        ("QCY Crossky C30", "qcy-crossky-c30"),
        ("QCY MeloBuds N70", "qcy-melobuds-n70"),
        ("QCY AilyBuds Pro+", "qcy-ailybuds-pro-plus"),
        ("QCY H3 Pro", "qcy-h3-pro"),
        ("QCY ArcBuds Lite", "qcy-arcbuds-lite"),
        ("QCY Heroad VT200", "qcy-heroad"),
        ("QCY SP7 Speaker", "speaker-generic"),
        ("QCY Watch GS2", "bluetooth"),
        ("Xiaomi Buds 6", "xiaomi-buds-6"),
        ("Redmi Buds 6", "redmi-buds-6"),
        ("soundcore Space A40", "soundcore-space-a40"),
        ("Galaxy Buds3 Pro", "galaxy-buds3"),
        ("Sony LinkBuds Open WF-L910", "sony-linkbuds-open"),
        ("Sony WF-1000XM4", "sony-wf-1000xm4"),
        ("soundcore Sport X20", "soundcore-sport-x20"),
        ("JBL Free", "jbl-free"),
        ("JBL Tour Pro 3", "jbl-tour-pro"),
        ("Pixel Buds Pro 2", "pixel-buds-pro"),
        ("Nothing Ear (a)", "nothing-ear-a"),
        ("Nothing Ear (1)", "nothing-ear-1"),
        ("HUAWEI FreeBuds 6", "huawei-freebuds-6"),
        ("Beats Studio Buds +", "beats-studio-buds-plus"),
        ("OnePlus Buds Pro 3", "oneplus-buds-pro"),
        ("HUAWEI FreeClip", "huawei-freeclip"),
        ("Beats Fit Pro", "beats-fit-pro"),
    ];

    for (name, expected_key) in fixtures {
        assert_eq!(classify(name).visual.key, expected_key, "{name}");
    }
}
