use crate::{constraints::penetration::PenetrationConstraint, DELTA_TIME};

use bevy::prelude::*;

/// Amount of substeps used in XPBD simulation
pub struct XPBDSubsteps(pub u32);

impl Default for XPBDSubsteps {
    fn default() -> Self {
        Self(8)
    }
}

/// Substep delta time
pub(crate) struct SubDeltaTime(pub f32);

impl Default for SubDeltaTime {
    fn default() -> Self {
        Self(DELTA_TIME / XPBDSubsteps::default().0 as f32)
    }
}

#[derive(Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0.0, -9.81))
    }
}

/// Stores penetration constraints for potentially colliding dynamic-dynamic entity pairs.
#[derive(Debug, Default)]
pub(crate) struct DynamicPenetrationConstraints(pub Vec<PenetrationConstraint>);

/// Stores penetration constraints for potentially colliding dynamic-static entity pairs.
#[derive(Debug, Default)]
pub(crate) struct StaticPenetrationConstraints(pub Vec<PenetrationConstraint>);

/// Stores dynamic-dynamic contact data.
#[derive(Default, Debug)]
pub struct DynamicContacts(pub Vec<Contact>);

/// Stores dynamic-static contact data.
#[derive(Default, Debug)]
pub struct StaticContacts(pub Vec<Contact>);

/// Data related to a contact between two bodies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Contact {
    pub entity_a: Entity,
    pub entity_b: Entity,
    /// Local contact point a
    pub r_a: Vec2,
    /// Local contact point b
    pub r_b: Vec2,
    /// Contact normal from contact point a to b
    pub normal: Vec2,
    /// Penetration depth
    pub penetration: f32,
}