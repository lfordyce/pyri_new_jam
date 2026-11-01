use std::mem;
use std::mem::MaybeUninit;
use bevy::ecs::bundle::DynamicBundle;
use bevy::ecs::component::ComponentId;
use bevy::ecs::component::Components;
use bevy::ecs::component::ComponentsRegistrator;
use bevy::ecs::component::StorageType;
use bevy::ptr::{MovingPtr, OwningPtr};

use crate::prelude::*;

/// A [`BundleEffect`] that applies an arbitrary function to its entity.
pub struct Patch<F: FnOnce(&mut EntityWorldMut)>(pub F);

// SAFETY: This internally relies on the `()` Bundle implementation, which is sound.
unsafe impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> Bundle for Patch<F> {
    fn component_ids(components: &mut ComponentsRegistrator) -> impl Iterator<Item = ComponentId> + use<Self> {
        <() as Bundle>::component_ids(components)
    }

    fn get_component_ids(components: &Components) -> impl Iterator<Item = Option<ComponentId>> {
        <() as Bundle>::get_component_ids(components)
    }

    // fn register_required_components(
    //     components: &mut ComponentsRegistrator,
    //     required_components: &mut RequiredComponents,
    // ) {
    //     <() as Bundle>::register_required_components(components, required_components)
    // }
}

impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> DynamicBundle for Patch<F> {
    type Effect = Self;

    unsafe fn get_components(
        ptr: MovingPtr<'_, Self>,
        func: &mut impl FnMut(StorageType, OwningPtr<'_>),
    ) {
        // <() as DynamicBundle>::get_components((), func);
        // self
        // unsafe { <() as DynamicBundle>::get_components((), func) };
        // Forget the pointer so that the value is available in `apply_effect`.
        mem::forget(ptr);
    }

    unsafe fn apply_effect(ptr: MovingPtr<'_, MaybeUninit<Self>>, entity: &mut EntityWorldMut) {
        // SAFETY: The pointer was not dropped in `get_components`, so the allocation is still
        // initialized.
        let add_observer = unsafe { ptr.assume_init() };
        if entity.is_despawned() {
            return;
        }
        add_observer.0(entity);
    }
}
