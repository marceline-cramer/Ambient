use super::Bindings;
use crate::shared::{implementation::unsupported, wit};

impl wit::server_physics::Host for Bindings {
    fn add_force(
        &mut self,
        _entity: wit::types::EntityId,
        _force: wit::types::Vec3,
    ) -> anyhow::Result<()> {
        unsupported()
    }

    fn add_impulse(
        &mut self,
        _entity: wit::types::EntityId,
        _force: wit::types::Vec3,
    ) -> anyhow::Result<()> {
        unsupported()
    }

    fn add_radial_impulse(
        &mut self,
        _position: wit::types::Vec3,
        _impulse: f32,
        _radius: f32,
        _falloff_radius: Option<f32>,
    ) -> anyhow::Result<()> {
        unsupported()
    }

    fn add_force_at_position(
        &mut self,
        _entity: wit::types::EntityId,
        _force: wit::types::Vec3,
        _position: wit::types::Vec3,
    ) -> anyhow::Result<()> {
        unsupported()
    }

    fn add_impulse_at_position(
        &mut self,
        _entity: wit::types::EntityId,
        _force: wit::types::Vec3,
        _position: wit::types::Vec3,
    ) -> anyhow::Result<()> {
        unsupported()
    }

    fn get_velocity_at_position(
        &mut self,
        _entity: wit::types::EntityId,
        _position: wit::types::Vec3,
    ) -> anyhow::Result<wit::types::Vec3> {
        unsupported()
    }

    fn set_gravity(&mut self, _gravity: wit::types::Vec3) -> anyhow::Result<()> {
        unsupported()
    }

    fn unfreeze(&mut self, _entity: wit::types::EntityId) -> anyhow::Result<()> {
        unsupported()
    }

    fn freeze(&mut self, _entity: wit::types::EntityId) -> anyhow::Result<()> {
        unsupported()
    }

    fn start_motor(&mut self, _entity: wit::types::EntityId, _velocity: f32) -> anyhow::Result<()> {
        unsupported()
    }

    fn stop_motor(&mut self, _entity: wit::types::EntityId) -> anyhow::Result<()> {
        unsupported()
    }

    fn raycast_first(
        &mut self,
        _origin: wit::types::Vec3,
        _direction: wit::types::Vec3,
    ) -> anyhow::Result<Option<(wit::types::EntityId, f32)>> {
        unsupported()
    }

    fn raycast(
        &mut self,
        _origin: wit::types::Vec3,
        _direction: wit::types::Vec3,
    ) -> anyhow::Result<Vec<(wit::types::EntityId, f32)>> {
        unsupported()
    }

    fn move_character(
        &mut self,
        _entity: wit::types::EntityId,
        _displacement: wit::types::Vec3,
        _min_dist: f32,
        _elapsed_time: f32,
    ) -> anyhow::Result<wit::server_physics::CharacterCollision> {
        unsupported()
    }
}

impl wit::server_message::Host for Bindings {
    fn send(
        &mut self,
        _: wit::server_message::Target,
        _: String,
        _: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
