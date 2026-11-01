use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/5893>.
pub trait ValExtAdd: Sized {
    fn add(
        &self,
        other: Self,
        scale_factor: f32,
        physical_base_value: f32,
        physical_target_size: Vec2,
        em_size: EmSize,
        rem_size: RemSize,
    ) -> Result<Self, ValArithmeticError>;
}

impl ValExtAdd for Val {
    fn add(
        &self,
        other: Self,
        scale_factor: f32,
        physical_base_value: f32,
        physical_target_size: Vec2,
        em_size: EmSize,
        rem_size: RemSize,
    ) -> Result<Self, ValArithmeticError> {
        Ok(Px(self.resolve(
            scale_factor,
            physical_base_value,
            physical_target_size,
            em_size,
            rem_size,
        )? + other.resolve(
            scale_factor,
            physical_base_value,
            physical_target_size,
            em_size,
            rem_size,
        )?))
    }
}
