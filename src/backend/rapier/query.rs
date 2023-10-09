use super::rapier::prelude::*;
use bevy::prelude::*;

pub type SpatialQuery<'w, 's> = Res<'s, RapierContext>;
