use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowPhysicsBounds;
impl ComputedStates for ShowPhysicsBounds {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_physics_bounds {
            Some(Self)
        } else {
            None
        }
    }
}

fn draw_bounds(
    mut gz: Gizmos,
    static_txs: Query<(&GlobalTransform, &StaticTx)>,
    static_rxs: Query<(&GlobalTransform, &StaticRx)>,
    trigger_txs: Query<(&GlobalTransform, &TriggerTx)>,
    trigger_rxs: Query<(&GlobalTransform, &TriggerRx)>,
) {
    for (gt, tx) in &static_txs {
        let (tran, angle) = gt.tran_n_angle();
        tx.bounds
            .draw(tran, angle, &mut gz, tailwind::GRAY_400.into());
    }
    for (gt, rx) in &static_rxs {
        let (tran, angle) = gt.tran_n_angle();
        rx.bounds
            .draw(tran, angle, &mut gz, tailwind::AMBER_400.into());
    }
    for (gt, tx) in &trigger_txs {
        let (tran, angle) = gt.tran_n_angle();
        tx.bounds
            .draw(tran, angle, &mut gz, tailwind::GREEN_400.into());
    }
    for (gt, rx) in &trigger_rxs {
        let (tran, angle) = gt.tran_n_angle();
        rx.bounds
            .draw(tran, angle, &mut gz, tailwind::GREEN_400.into());
    }
}

pub(super) fn register_dphysics(app: &mut App) {
    app.add_computed_state::<ShowPhysicsBounds>();
    app.add_systems(
        FixedPostUpdate,
        draw_bounds
            .run_if(in_state(ShowPhysicsBounds))
            .after(AnimationSet),
    );
}
