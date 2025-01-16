use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Hotspot data for a cursor.
///
/// This struct is used to store hotspot information for a cursor. The hotspot
/// is the point in the cursor image that is used as the "click point" when
/// interacting with a cursor.
///
/// A hotspot is defined as a pair of `(x, y)` coordinates, where `(0, 0)` is
/// the top-left corner of the cursor's image.
#[derive(Clone, Component, Debug, Default, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Component, Debug, Default)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub struct CursorHotspots {
    /// The default hotspot for the cursor.
    ///
    /// This is used when a frame does not have an entry in the `overrides` map.
    #[cfg_attr(feature = "serde", serde(default))]
    pub default: (u16, u16),
    /// Overrides the hotspot for specific frames.
    ///
    /// The key is the frame index and the value is the hotspot for that frame.
    ///
    /// If a frame index is not present in this map, the `default` hotspot
    /// should be used.
    #[cfg_attr(feature = "serde", serde(default))]
    pub overrides: HashMap<usize, (u16, u16)>,
}

impl CursorHotspots {
    /// Returns the hotspot for the given frame index or the default hotspot.
    ///
    /// If the frame index is not present in the `overrides` map, the `default`
    /// hotspot is returned.
    pub fn get_or_default(&self, index: usize) -> (u16, u16) {
        self.overrides.get(&index).copied().unwrap_or(self.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_default() {
        let hotspots = CursorHotspots {
            default: (10, 0),
            overrides: HashMap::new(),
        };

        assert_eq!(hotspots.get_or_default(0), (10, 0));
        assert_eq!(hotspots.get_or_default(1), (10, 0));
        assert_eq!(hotspots.get_or_default(2), (10, 0));

        let hotspots = CursorHotspots {
            default: (10, 0),
            overrides: vec![(1, (1, 1))].into_iter().collect(),
        };

        assert_eq!(hotspots.get_or_default(0), (10, 0));
        assert_eq!(hotspots.get_or_default(1), (1, 1));
        assert_eq!(hotspots.get_or_default(2), (10, 0));
    }
}
