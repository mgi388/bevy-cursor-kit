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

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<bevy_spritesheet_animation::animation::AnimationDuration> for AnimationDuration {
    fn from(duration: bevy_spritesheet_animation::animation::AnimationDuration) -> Self {
        match duration {
            bevy_spritesheet_animation::animation::AnimationDuration::PerFrame(duration) => {
                AnimationDuration::PerFrame(duration)
            }
            bevy_spritesheet_animation::animation::AnimationDuration::PerRepetition(duration) => {
                AnimationDuration::PerRepetition(duration)
            }
        }
    }
}

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<AnimationDuration> for bevy_spritesheet_animation::animation::AnimationDuration {
    fn from(duration: AnimationDuration) -> Self {
        match duration {
            AnimationDuration::PerFrame(duration) => {
                bevy_spritesheet_animation::animation::AnimationDuration::PerFrame(duration)
            }
            AnimationDuration::PerRepetition(duration) => {
                bevy_spritesheet_animation::animation::AnimationDuration::PerRepetition(duration)
            }
        }
    }
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

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<bevy_spritesheet_animation::animation::AnimationRepeat> for AnimationRepeat {
    fn from(repeat: bevy_spritesheet_animation::animation::AnimationRepeat) -> Self {
        match repeat {
            bevy_spritesheet_animation::animation::AnimationRepeat::Loop => AnimationRepeat::Loop,
            bevy_spritesheet_animation::animation::AnimationRepeat::Times(times) => {
                AnimationRepeat::Times(times)
            }
        }
    }
}

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<AnimationRepeat> for bevy_spritesheet_animation::animation::AnimationRepeat {
    fn from(repeat: AnimationRepeat) -> Self {
        match repeat {
            AnimationRepeat::Loop => bevy_spritesheet_animation::animation::AnimationRepeat::Loop,
            AnimationRepeat::Times(times) => {
                bevy_spritesheet_animation::animation::AnimationRepeat::Times(times)
            }
        }
    }
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

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<bevy_spritesheet_animation::animation::AnimationDirection> for AnimationDirection {
    fn from(direction: bevy_spritesheet_animation::animation::AnimationDirection) -> Self {
        match direction {
            bevy_spritesheet_animation::animation::AnimationDirection::Forwards => {
                AnimationDirection::Forwards
            }
            bevy_spritesheet_animation::animation::AnimationDirection::Backwards => {
                AnimationDirection::Backwards
            }
            bevy_spritesheet_animation::animation::AnimationDirection::PingPong => {
                AnimationDirection::PingPong
            }
        }
    }
}

#[cfg(feature = "bevy_spritesheet_animation")]
impl From<AnimationDirection> for bevy_spritesheet_animation::animation::AnimationDirection {
    fn from(direction: AnimationDirection) -> Self {
        match direction {
            AnimationDirection::Forwards => {
                bevy_spritesheet_animation::animation::AnimationDirection::Forwards
            }
            AnimationDirection::Backwards => {
                bevy_spritesheet_animation::animation::AnimationDirection::Backwards
            }
            AnimationDirection::PingPong => {
                bevy_spritesheet_animation::animation::AnimationDirection::PingPong
            }
        }
    }
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
