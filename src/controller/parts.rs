

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Parts {
    pub parts: Vec<Entity>,
}