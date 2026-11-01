use bevy::ecs::entity::{Entities, EntityAllocator};
use bevy::ecs::system::SystemParam;
use bevy::ecs::system::{SystemBuffer, SystemMeta};
use bevy::ecs::world::{CommandQueue, DeferredWorld};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<LateCommandBuffer>();
}

/// Like [`Commands`], but applied during [`UpdateSystems::ApplyCommands`] instead of
/// at the next sync point.
///
/// Example usage: `late.commands().entity(entity).despawn_recursive()`.
#[derive(SystemParam)]
pub struct LateCommands<'w, 's> {
    queue: Deferred<'s, LateCommandQueue>,
    allocator: &'w EntityAllocator,
    entities: &'w Entities,
}

impl LateCommands<'_, '_> {
    pub fn commands(&mut self) -> Commands<'_, '_> {
        Commands::new_from_entities(&mut self.queue.0, self.allocator, self.entities)
    }
}

#[derive(Default)]
struct LateCommandQueue(CommandQueue);

impl SystemBuffer for LateCommandQueue {
    fn queue(&mut self, _system_meta: &SystemMeta, mut world: DeferredWorld) {
        if let Some(mut a) = world.get_resource_mut::<LateCommandBuffer>() {
            a.0.append(&mut self.0);
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct LateCommandBuffer(#[reflect(ignore)] CommandQueue);

impl Configure for LateCommandBuffer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            apply_late_commands.in_set(UpdateSystems::ApplyCommands),
        );
    }
}

fn apply_late_commands(mut commands: Commands, mut late_commands: ResMut<LateCommandBuffer>) {
    commands.append(&mut late_commands.0);
}
