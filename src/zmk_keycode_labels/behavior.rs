use crate::layout_key::{KeycodeKind, Label, LayoutKey};
use std::collections::HashMap;
use zmk_studio_api::{
    Behavior,
    HidUsage,
    BehaviorBindingParametersSet,
    BehaviorParameterValueType,
};

use super::hid_usage::hid_usage_to_layout_key;

pub fn behavior_to_layout_key(behavior: &Behavior) -> Option<LayoutKey> {
    match behavior {
        Behavior::Transparent => None,

        Behavior::None => Some(LayoutKey {
            tap: Label::new(""),
            ..Default::default()
        }),
        Behavior::KeyPress(keycode) => Some(hid_usage_to_layout_key(*keycode)),
        Behavior::KeyToggle(keycode) => {
            let mut key = hid_usage_to_layout_key(*keycode);
            key.hold = Some(Label::new("Toggle"));
            Some(key)
        }
        Behavior::MomentaryLayer { layer_id } => Some(layer_layout_key("MO", *layer_id)),
        Behavior::ToggleLayer { layer_id } => Some(layer_layout_key("TG", *layer_id)),
        Behavior::ToLayer { layer_id } => Some(layer_layout_key("TO", *layer_id)),
        Behavior::StickyLayer { layer_id } => Some(layer_layout_key("SL", *layer_id)),
        Behavior::LayerTap { layer_id, tap } => {
            let tap_key = hid_usage_to_layout_key(*tap);
            Some(LayoutKey {
                tap: tap_key.tap,
                hold: Some(Label::with_short(
                    format!("L{}", layer_id),
                    format!("L{}", layer_id),
                )),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Special,
                layer_ref: Some(*layer_id as u8),
            })
        }
        Behavior::ModTap { hold, tap } => {
            let hold_key = hid_usage_to_layout_key(*hold);
            let tap_key = hid_usage_to_layout_key(*tap);
            Some(LayoutKey {
                tap: tap_key.tap,
                hold: Some(hold_key.tap),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Modifier,
                layer_ref: None,
            })
        }
        Behavior::StickyKey(keycode) => {
            let key = hid_usage_to_layout_key(*keycode);
            Some(LayoutKey {
                tap: Label::with_short(
                    format!("OS {}", key.tap.full),
                    format!("OS{}", key.tap.short.as_deref().unwrap_or(&key.tap.full)),
                ),
                kind: KeycodeKind::Modifier,
                ..Default::default()
            })
        }
        Behavior::CapsWord => Some(LayoutKey {
            tap: Label::with_short("Caps Word", "CW"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::KeyRepeat => Some(LayoutKey {
            tap: Label::with_short("Key Repeat", "Rep"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Reset => Some(LayoutKey {
            tap: Label::new("Reset"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Bootloader => Some(LayoutKey {
            tap: Label::with_short("Bootloader", "Boot"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::SoftOff => Some(LayoutKey {
            tap: Label::with_short("Soft Off", "Off"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::StudioUnlock => Some(LayoutKey {
            tap: Label::with_short("Studio Unlock", "Unlock"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::GraveEscape => Some(LayoutKey {
            tap: Label::with_short("Grave Esc", "G/E"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Bluetooth { command, .. } => {
            let label = match *command {
                0 => "BT Clr",
                1 => "BT Nxt",
                2 => "BT Prv",
                n => {
                    return Some(LayoutKey {
                        tap: Label::new(format!("BT {}", n)),
                        kind: KeycodeKind::Special,
                        ..Default::default()
                    })
                }
            };
            Some(LayoutKey {
                tap: Label::new(label),
                kind: KeycodeKind::Special,
                ..Default::default()
            })
        }
        Behavior::OutputSelection { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Out {}", value), format!("Out{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::ExternalPower { value } => Some(LayoutKey {
            tap: Label::with_short(format!("ExtPwr {}", value), format!("EP{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Backlight { command, .. } => Some(LayoutKey {
            tap: Label::with_short(format!("BL {}", command), format!("BL{}", command)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Underglow { command, .. } => Some(LayoutKey {
            tap: Label::with_short(format!("RGB {}", command), format!("RGB{}", command)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseKeyPress { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Mouse {}", value), format!("M{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseMove { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Move {}", value), format!("Mv{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseScroll { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Scroll {}", value), format!("Scr{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Unknown {
            behavior_id,
            param1,
            param2,
        } => {
            let label = if *param2 != 0 {
                format!("0x{:X} {} {}", behavior_id, param1, param2)
            } else if *param1 != 0 {
                format!("0x{:X} {}", behavior_id, param1)
            } else {
                format!("0x{:X}", behavior_id)
            };
            Some(LayoutKey {
                tap: Label::new(label),
                ..Default::default()
            })
        }
    }
}

pub fn behavior_to_layout_key_with_metadata(
    behavior: &Behavior,
    behavior_names: &HashMap<i32, String>,
    behavior_metadata: &HashMap<i32, Vec<BehaviorBindingParametersSet>>,
) -> Option<LayoutKey> {
    match behavior {
        Behavior::Unknown {
            behavior_id,
            param1,
            param2,
        } => unknown_behavior_to_layout_key(
                *behavior_id,
                *param1,
                *param2,
                behavior_names,
                behavior_metadata,
            ),
        _ => behavior_to_layout_key(behavior),
    }
}

fn unknown_behavior_to_layout_key(
    behavior_id: i32,
    param1: u32,
    param2: u32,
    behavior_names: &HashMap<i32, String>,
    behavior_metadata: &HashMap<i32, Vec<BehaviorBindingParametersSet>>,
) -> Option<LayoutKey> {
    let name = behavior_names.get(&behavior_id);
    let is_tap_dance = name
        .map(|n| {
            let n = n.to_lowercase();
            n.starts_with("td")
                || n.contains("tap_dance")
                || n.contains("tap-dance")
        })
        .unwrap_or(false);

    let (selected_param, selected_param_index) = if is_tap_dance {
        (param1, 1)
    } else if param2 != 0 {
        (param2, 2)
    } else {
        (param1, 1)
    };

    if selected_param != 0 {
        if let Some(metadata_sets) = behavior_metadata.get(&behavior_id) {
            if param_is_layer(metadata_sets, selected_param_index) {
                return Some(LayoutKey {
                    tap: Label::new(format!("L{}", selected_param)),
                    kind: KeycodeKind::Special,
                    layer_ref: Some(selected_param as u8),
                    ..Default::default()
                });
            }

            if param_is_hid(metadata_sets, selected_param_index) {
                let mut key = hid_usage_to_layout_key(HidUsage::from_encoded(selected_param));
                key.kind = KeycodeKind::Special;
                return Some(key);
            }
        }

        // metadata가 없으면 기존처럼 HID로 시도
        let mut key = hid_usage_to_layout_key(HidUsage::from_encoded(selected_param));
        key.kind = KeycodeKind::Special;
        return Some(key);
    }

    let custom_labels: HashMap<&str, &str> = HashMap::from([
        ("TD_RR", "R"),
        ("TD_EE", "E"),
        ("TD_OO", "O"),
        ("TD_PP", "P"),
        ("TD_QQ", "Q"),
        ("TD_TT", "T"),
        ("TD_WW", "W"),
        ("TD_LSET", "("),
        ("TD_RSET", ")"),
    ]);

    if let Some(name) = name {
    if let Some(label) = custom_labels.get(name.as_str()) {
        return Some(LayoutKey {
            tap: Label::new((*label).to_string()),
            kind: KeycodeKind::Special,
            ..Default::default()
        });
    }
}

    behavior_to_layout_key(&Behavior::Unknown {
        behavior_id,
        param1,
        param2,
    })
}

fn layer_layout_key(abbreviation: &str, layer_id: u32) -> LayoutKey {
    LayoutKey {
        tap: Label::with_short(
            format!("{} {}", abbreviation, layer_id),
            format!("{}{}", abbreviation, layer_id),
        ),
        kind: KeycodeKind::Special,
        layer_ref: Some(layer_id as u8),
        ..Default::default()
    }
}

fn param_is_layer(
    metadata_sets: &[BehaviorBindingParametersSet],
    param_index: u8,
) -> bool {
    metadata_sets.first().is_some_and(|first_set| {
        let params = if param_index == 1 {
            &first_set.param1
        } else {
            &first_set.param2
        };

        params.iter().any(|p| {
            matches!(
                p.value_type,
                Some(BehaviorParameterValueType::LayerId(_))
            )
        })
    })
}

fn param_is_hid(
    metadata_sets: &[BehaviorBindingParametersSet],
    param_index: u8,
) -> bool {
    metadata_sets.first().is_some_and(|first_set| {
        let params = if param_index == 1 {
            &first_set.param1
        } else {
            &first_set.param2
        };

        params.iter().any(|p| {
            matches!(
                p.value_type,
                Some(BehaviorParameterValueType::HidUsage(_))
            )
        })
    })
}
