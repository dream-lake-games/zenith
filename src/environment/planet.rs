use crate::prelude::*;

#[derive(Bundle)]
pub struct PlanetBundle {
    name: Name,
    spatial: SpatialBundle,
    static_tx: StaticTx,
    texture: TextureManager<TextureTestPlanetState>,
}
impl PlanetBundle {
    pub fn new(name: &str, tx_kind: StaticTxKind, pos: Vec2, shape: Shape) -> Self {
        let points = shape.to_points();
        Self {
            name: Name::new(name.to_string()),
            spatial: spat_tran!(pos.x, pos.y, ZIX_PLANET + zix_nudge()),
            static_tx: StaticTx::from_kind_n_shape(tx_kind, shape),
            texture: TextureManager::new()
                .with_part_points(TextureTestPlanetPart::Inner, outline_points(&points, -6.0))
                .with_part_points(TextureTestPlanetPart::Outer, points),
        }
    }
}
