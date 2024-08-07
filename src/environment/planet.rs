use crate::prelude::*;

#[derive(Bundle)]
pub struct PlanetBundle {
    name: Name,
    spatial: SpatialBundle,
    static_tx: StaticTx,
    texture: TextureManager<TextureTestPlanetState>,
    mirage_texture: MirageTextureManager,
}
impl PlanetBundle {
    pub fn new(
        name: &str,
        tx_kind: StaticTxKind,
        pos: Vec2,
        shape: Shape,
        room_state: &RoomState,
    ) -> Self {
        let canonical_points = shape.to_points();
        let mirage_shapes = room_state
            .mirage_offsets()
            .into_iter()
            .map(|p| shape.clone().with_offset(p));
        let mut all_shapes: Vec<Shape> = vec![shape.clone()];
        all_shapes.extend(mirage_shapes);
        Self {
            name: Name::new(name.to_string()),
            spatial: spat_tran!(pos.x, pos.y, ZIX_PLANET + zix_nudge()),
            static_tx: StaticTx::from_kind_n_shapes(tx_kind, all_shapes),
            texture: TextureManager::new()
                .with_part_points(
                    TextureTestPlanetPart::Inner,
                    outline_points(&canonical_points, -6.0),
                )
                .with_part_points(TextureTestPlanetPart::Outer, canonical_points),
            mirage_texture: MirageTextureManager::room_offsets(&room_state),
        }
    }
}
