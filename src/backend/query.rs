
#[derive(Debug, Copy, Clone, Reflect)]
pub struct RayCastResult {
    pub entity: Entity,
    pub toi: f32,
    pub normal: Vec3,
    pub point: Vec3,
}

#[derive(Debug, Copy, Clone, Reflect)]
pub struct ShapeCastResult {
    pub entity: Entity,
    pub toi: f32,
    pub normal1: Vec3,
    pub normal2: Vec3,
    pub point1: Vec3,
    pub point2: Vec3,
}

pub struct QueryFilter {
    pub exclude: HashSet<Entity>,
}