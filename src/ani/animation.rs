use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The duration of an animation or clip.
#[derive(Debug, Clone, Copy, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Debug)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub enum AnimationDuration {
    PerFrame(u32),
    PerRepetition(u32),
}

/// The repeat behavior of an animation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub enum AnimationRepeat {
    #[default]
    Loop,
    Times(usize),
}

/// The direction of an animation or clip.
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub enum AnimationDirection {
    #[default]
    Forwards,
    Backwards,
    PingPong,
}

/// A clip in an animation.
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Debug)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub struct AnimationClip {
    /// The indices of the frames in the atlas that make up the clip.
    pub atlas_indices: Vec<usize>,
    /// The duration of the clip.
    pub duration: AnimationDuration,
    /// The direction of the clip.
    #[cfg_attr(feature = "serde", serde(default))]
    pub direction: AnimationDirection,
}

/// An animation.
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[reflect(Debug)]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub struct Animation {
    /// The clips that make up the animation.
    pub clips: Vec<AnimationClip>,
    /// The repeat behavior of the animation.
    #[cfg_attr(feature = "serde", serde(default))]
    pub repeat: AnimationRepeat,
    /// The direction of the animation.
    #[cfg_attr(feature = "serde", serde(default))]
    pub direction: AnimationDirection,
}
