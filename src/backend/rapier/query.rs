
use bevy::prelude::*;
use super::rapier::prelude::*;

pub type SpatialQuery<'w, 's> = Res<'s, RapierContext>;