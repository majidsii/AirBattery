//! Deterministic device classification and presentation artwork selection.

use shared_models::{DeviceFamily, DeviceVisual, VisualConfidence};

/// Metadata available to the universal classifier.
#[derive(Debug, Clone, Copy)]
pub struct ClassificationInput<'a> {
    /// Best user-visible name.
    pub name: &'a str,
    /// Reliable manufacturer name when available.
    pub manufacturer: Option<&'a str>,
    /// Reliable model name when available.
    pub model: Option<&'a str>,
    /// `BlueZ` icon hint.
    pub icon: Option<&'a str>,
    /// Bluetooth Class of Device.
    pub class: Option<u32>,
    /// Bluetooth appearance value.
    pub appearance: Option<u16>,
    /// Service UUIDs in lowercase canonical form.
    pub service_uuids: &'a [String],
    /// Strong protocol or platform family hint.
    pub family_hint: DeviceFamily,
}

/// Device category and artwork selected independently from battery values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceClassification {
    /// Broad category used by layout logic.
    pub family: DeviceFamily,
    /// Exact, category, or fallback artwork.
    pub visual: DeviceVisual,
}

fn contains_any(identity: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| identity.contains(needle))
}

fn result(family: DeviceFamily, key: &str, confidence: VisualConfidence) -> DeviceClassification {
    DeviceClassification {
        family,
        visual: DeviceVisual {
            key: key.to_owned(),
            confidence,
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct QcyModelRule {
    needles: &'static [&'static str],
    key: &'static str,
    family: DeviceFamily,
}

const QCY_MODEL_RULES: &[QcyModelRule] = &[
    QcyModelRule {
        needles: &["crossky c50"],
        key: "qcy-crossky-c50",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky c30s"],
        key: "qcy-crossky-c30s",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky c30"],
        key: "qcy-crossky-c30",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky c10"],
        key: "qcy-crossky-c10",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky r70"],
        key: "qcy-crossky-r70",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky link"],
        key: "qcy-crossky-link",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["crossky gtr2"],
        key: "qcy-crossky-gtr2",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds n70"],
        key: "qcy-melobuds-n70",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds n60"],
        key: "qcy-melobuds-n60",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds a30"],
        key: "qcy-melobuds-a30",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds n20"],
        key: "qcy-melobuds-n20",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds n50"],
        key: "qcy-melobuds-n50",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds n65"],
        key: "qcy-melobuds-n65",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds neo"],
        key: "qcy-melobuds-neo",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds anc"],
        key: "qcy-melobuds-anc",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["melobuds pro"],
        key: "qcy-melobuds-pro",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["t13 anc 2"],
        key: "qcy-t13-anc-2",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["t13 pro"],
        key: "qcy-t13-pro",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["t13 anc"],
        key: "qcy-t13-anc",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["t13x"],
        key: "qcy-t13x",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["t17"],
        key: "qcy-t17",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["ailybuds pro+"],
        key: "qcy-ailybuds-pro-plus",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["ailybuds e10"],
        key: "qcy-ailybuds-e10",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["ailybuds clear"],
        key: "qcy-ailybuds-clear",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["qcy air"],
        key: "qcy-air",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["qcy buds anc"],
        key: "qcy-buds-anc",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["qcy buds"],
        key: "qcy-buds",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["arcbuds lite"],
        key: "qcy-arcbuds-lite",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["arcbuds"],
        key: "qcy-arcbuds",
        family: DeviceFamily::Earbuds,
    },
    QcyModelRule {
        needles: &["h3 pro"],
        key: "qcy-h3-pro",
        family: DeviceFamily::Headset,
    },
    QcyModelRule {
        needles: &["h3 lite"],
        key: "qcy-h3-lite",
        family: DeviceFamily::Headset,
    },
    QcyModelRule {
        needles: &["h3s"],
        key: "qcy-h3s",
        family: DeviceFamily::Headset,
    },
    QcyModelRule {
        needles: &["qcy h3"],
        key: "qcy-h3",
        family: DeviceFamily::Headset,
    },
    QcyModelRule {
        needles: &["h2 pro"],
        key: "qcy-h2-pro",
        family: DeviceFamily::Headset,
    },
];

fn qcy_classification(identity: &str) -> (&'static str, DeviceFamily) {
    for rule in QCY_MODEL_RULES {
        if rule.needles.iter().all(|needle| identity.contains(needle)) {
            return (rule.key, rule.family);
        }
    }

    if identity.contains("crossky") {
        ("qcy-crossky", DeviceFamily::Earbuds)
    } else if identity.contains("ailybuds") {
        ("qcy-ailybuds", DeviceFamily::Earbuds)
    } else if identity.contains("melobuds") {
        ("qcy-melobuds", DeviceFamily::Earbuds)
    } else {
        ("qcy-t13", DeviceFamily::Earbuds)
    }
}

/// Classifies one Bluetooth device without inferring battery component values.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn classify_device(input: ClassificationInput<'_>) -> DeviceClassification {
    let identity = [input.manufacturer, input.model, Some(input.name)]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    let icon = input.icon.unwrap_or_default().to_ascii_lowercase();

    if contains_any(&identity, &["airpods pro 3", "airpods pro (3rd"]) {
        return result(
            DeviceFamily::AirPods,
            "airpods-pro-3",
            VisualConfidence::Exact,
        );
    }
    if contains_any(&identity, &["airpods pro (1st", "airpods pro 1"]) {
        return result(
            DeviceFamily::AirPods,
            "airpods-pro-1",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("airpods pro") {
        return result(
            DeviceFamily::AirPods,
            "airpods-pro-2",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("airpods max 2") {
        return result(
            DeviceFamily::Headset,
            "airpods-max-2",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("airpods max") {
        return result(
            DeviceFamily::Headset,
            "airpods-max",
            VisualConfidence::Exact,
        );
    }
    if contains_any(&identity, &["airpods 4", "airpods (4th"]) {
        return result(DeviceFamily::AirPods, "airpods-4", VisualConfidence::Exact);
    }
    if contains_any(&identity, &["airpods 3", "airpods (3rd"]) {
        return result(DeviceFamily::AirPods, "airpods-3", VisualConfidence::Exact);
    }
    if contains_any(&identity, &["airpods 2", "airpods (2nd"]) {
        return result(DeviceFamily::AirPods, "airpods-2", VisualConfidence::Exact);
    }
    if contains_any(&identity, &["airpods 1", "airpods (1st"]) {
        return result(DeviceFamily::AirPods, "airpods-1", VisualConfidence::Exact);
    }
    if identity.contains("airpods") || matches!(input.family_hint, DeviceFamily::AirPods) {
        return result(
            DeviceFamily::AirPods,
            "airpods-generic",
            VisualConfidence::Category,
        );
    }

    if identity.contains("qcy") {
        if contains_any(&identity, &["qcy sp7", "qcy speaker"]) {
            return result(
                DeviceFamily::Speaker,
                "speaker-generic",
                VisualConfidence::Category,
            );
        }
        if contains_any(
            &identity,
            &[
                "qcy watch",
                "watch gs2",
                "watch gt2",
                "urban gs",
                "active gt",
            ],
        ) {
            return result(
                DeviceFamily::GenericBle,
                "bluetooth",
                VisualConfidence::Fallback,
            );
        }
        if contains_any(&identity, &["heroad", "vt200", "v200", "gt2 tri-mode"]) {
            return result(DeviceFamily::Headset, "qcy-heroad", VisualConfidence::Exact);
        }
        let (key, family) = qcy_classification(&identity);
        return result(family, key, VisualConfidence::Exact);
    }

    if identity.contains("xiaomi buds") {
        let key = if identity.contains("buds 6") {
            "xiaomi-buds-6"
        } else {
            "xiaomi-buds-5"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("redmi buds") {
        let key = if identity.contains("buds 8") {
            "redmi-buds-8"
        } else if identity.contains("buds 6") {
            "redmi-buds-6"
        } else if identity.contains("buds 5") {
            "redmi-buds-5"
        } else if identity.contains("buds 4") {
            "redmi-buds-4"
        } else if identity.contains("buds 3") {
            "redmi-buds-3"
        } else {
            "redmi-buds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }

    if identity.contains("soundcore") || identity.contains("liberty") {
        let key = if identity.contains("sport x20") || identity.contains("sport-x20") {
            "soundcore-sport-x20"
        } else if identity.contains("aerofit") {
            "soundcore-aerofit"
        } else if identity.contains("space a40") {
            "soundcore-space-a40"
        } else if identity.contains("sleep") {
            "soundcore-sleep"
        } else if identity.contains("liberty") {
            "soundcore-liberty"
        } else if contains_any(&identity, &["p20", "p25", "p30", "p40", "p41"]) {
            "soundcore-p-series"
        } else {
            "soundcore-earbuds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }

    if identity.contains("galaxy buds") {
        let key = if identity.contains("buds4") {
            "galaxy-buds4"
        } else if identity.contains("buds3") {
            "galaxy-buds3"
        } else if identity.contains("live") {
            "galaxy-buds-live"
        } else if identity.contains("fe") {
            "galaxy-buds-fe"
        } else if identity.contains("buds2") {
            "galaxy-buds2"
        } else {
            "galaxy-buds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }

    if identity.contains("linkbuds open") || identity.contains("wf-l910") {
        return result(
            DeviceFamily::Earbuds,
            "sony-linkbuds-open",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("linkbuds fit") || identity.contains("wf-ls910") {
        return result(
            DeviceFamily::Earbuds,
            "sony-linkbuds-fit",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("wf-1000xm5") {
        return result(
            DeviceFamily::Earbuds,
            "sony-wf-1000xm5",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("wf-1000xm4") {
        return result(
            DeviceFamily::Earbuds,
            "sony-wf-1000xm4",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("sony") && identity.contains("wf-") {
        return result(DeviceFamily::Earbuds, "sony-wf", VisualConfidence::Category);
    }

    if identity.contains("jbl") {
        let key = if identity.contains("jbl free") {
            "jbl-free"
        } else if identity.contains("live buds") || identity.contains("tune buds") {
            if identity.contains("live buds") {
                "jbl-live-buds"
            } else {
                "jbl-tune-buds"
            }
        } else if identity.contains("tour pro") {
            "jbl-tour-pro"
        } else if identity.contains("live beam") {
            "jbl-live-beam"
        } else if identity.contains("tune beam") {
            "jbl-tune-beam"
        } else if identity.contains("charge") || identity.contains("speaker") {
            "speaker-generic"
        } else {
            "jbl-earbuds"
        };
        let family = if key == "speaker-generic" {
            DeviceFamily::Speaker
        } else {
            DeviceFamily::Earbuds
        };
        return result(family, key, VisualConfidence::Exact);
    }

    if identity.contains("pixel buds") {
        let key = if identity.contains("2a") {
            "pixel-buds-2a"
        } else if identity.contains("pro") {
            "pixel-buds-pro"
        } else {
            "pixel-buds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("nothing ear") {
        let key = if identity.contains("open") {
            "nothing-ear-open"
        } else if identity.contains("(a)") || identity.contains("ear a") {
            "nothing-ear-a"
        } else if identity.contains("nothing ear (1)") || identity.contains("nothing ear 1") {
            "nothing-ear-1"
        } else {
            "nothing-ear"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("oneplus") && identity.contains("buds") {
        let key = if identity.contains("nord") {
            "oneplus-nord-buds"
        } else if identity.contains("pro") {
            "oneplus-buds-pro"
        } else {
            "oneplus-buds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("freeclip") {
        return result(
            DeviceFamily::Earbuds,
            "huawei-freeclip",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("freebuds") {
        let key = if identity.contains("freebuds 6") {
            "huawei-freebuds-6"
        } else if identity.contains("pro") {
            "huawei-freebuds-pro"
        } else {
            "huawei-freebuds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("beats fit pro") {
        return result(
            DeviceFamily::Earbuds,
            "beats-fit-pro",
            VisualConfidence::Exact,
        );
    }
    if identity.contains("beats studio buds") {
        let key = if identity.contains("plus") || identity.contains("buds +") {
            "beats-studio-buds-plus"
        } else {
            "beats-studio-buds"
        };
        return result(DeviceFamily::Earbuds, key, VisualConfidence::Exact);
    }
    if identity.contains("beats") && (identity.contains("buds") || identity.contains("fit")) {
        return result(
            DeviceFamily::Earbuds,
            "beats-earbuds",
            VisualConfidence::Category,
        );
    }

    if identity.contains("buds") || identity.contains("earbud") || identity.contains("tws") {
        return result(
            DeviceFamily::Earbuds,
            "earbuds-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("jbl charge")
        || identity.contains("speaker")
        || icon.contains("audio-speakers")
    {
        return result(
            DeviceFamily::Speaker,
            "speaker-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("mx master")
        || identity.contains("mouse")
        || icon.contains("mouse")
        || input.appearance == Some(0x03c2)
    {
        return result(
            DeviceFamily::Mouse,
            "mouse-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("keychron")
        || identity.contains("keyboard")
        || icon.contains("keyboard")
        || input.appearance == Some(0x03c1)
    {
        return result(
            DeviceFamily::Keyboard,
            "keyboard-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("controller")
        || identity.contains("gamepad")
        || identity.contains("joystick")
        || matches!(input.appearance, Some(0x03c4 | 0x03c5))
    {
        return result(
            DeviceFamily::GameController,
            "game-controller-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("pencil")
        || identity.contains("stylus")
        || identity.contains("digital pen")
        || matches!(input.appearance, Some(0x03c6 | 0x03c8))
    {
        return result(
            DeviceFamily::Stylus,
            "stylus-generic",
            VisualConfidence::Category,
        );
    }
    if identity.contains("wh-")
        || identity.contains("headphone")
        || identity.contains("headset")
        || icon.contains("headset")
        || icon.contains("headphones")
    {
        return result(
            DeviceFamily::Headset,
            "headset-generic",
            VisualConfidence::Category,
        );
    }

    match input.family_hint {
        DeviceFamily::Earbuds => result(
            DeviceFamily::Earbuds,
            "earbuds-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Headset => result(
            DeviceFamily::Headset,
            "headset-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Speaker => result(
            DeviceFamily::Speaker,
            "speaker-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Mouse => result(
            DeviceFamily::Mouse,
            "mouse-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Keyboard => result(
            DeviceFamily::Keyboard,
            "keyboard-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::GameController => result(
            DeviceFamily::GameController,
            "game-controller-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Stylus => result(
            DeviceFamily::Stylus,
            "stylus-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::GenericBle => result(
            DeviceFamily::GenericBle,
            "bluetooth",
            VisualConfidence::Fallback,
        ),
        DeviceFamily::AirPods => result(
            DeviceFamily::AirPods,
            "airpods-generic",
            VisualConfidence::Category,
        ),
        DeviceFamily::Unknown => {
            // Class-of-device major class 0x04 is audio/video. It is only a category hint.
            if input
                .class
                .is_some_and(|class| ((class >> 8) & 0x1f) == 0x04)
            {
                result(
                    DeviceFamily::Headset,
                    "headset-generic",
                    VisualConfidence::Category,
                )
            } else {
                result(
                    DeviceFamily::Unknown,
                    "bluetooth",
                    VisualConfidence::Fallback,
                )
            }
        }
    }
}
