use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14236>.
pub trait TriggerExtGetTarget {
    fn get_target(&self) -> Option<Entity>;
}

impl<'w, 't, E: Event> TriggerExtGetTarget for On<'w, 't, E> {
    fn get_target(&self) -> Option<Entity> {
        Some(self.target()).filter(|&x| x != Entity::PLACEHOLDER)
    }
}
