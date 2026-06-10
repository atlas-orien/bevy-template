use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::{
    QueryFilter as RapierQueryFilter, QueryFilterFlags as RapierQueryFilterFlags,
    ReadRapierContext, ShapeCastOptions as RapierShapeCastOptions,
};

use crate::{
    PhysicsCollider3d, PhysicsPointProjection3d, PhysicsQueryFilter, PhysicsRayHit3d,
    PhysicsShapeCastHit3d, PhysicsShapeCastHitDetails3d,
};

use super::convert;

#[derive(SystemParam)]
pub struct PhysicsQuery3d<'w, 's> {
    context: ReadRapierContext<'w, 's>,
}

impl PhysicsQuery3d<'_, '_> {
    pub fn cast_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_time_of_impact: f32,
        solid: bool,
        filter: PhysicsQueryFilter,
    ) -> Option<PhysicsRayHit3d> {
        let context = self.context.single().ok()?;
        let (entity, intersection) = context.cast_ray_and_get_normal(
            origin,
            direction,
            max_time_of_impact,
            solid,
            query_filter(filter),
        )?;

        Some(PhysicsRayHit3d {
            entity,
            time_of_impact: intersection.time_of_impact,
            point: intersection.point,
            normal: intersection.normal,
        })
    }

    pub fn intersect_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_time_of_impact: f32,
        solid: bool,
        filter: PhysicsQueryFilter,
    ) -> Vec<PhysicsRayHit3d> {
        let Ok(context) = self.context.single() else {
            return Vec::new();
        };
        let mut hits = Vec::new();
        context.intersect_ray(
            origin,
            direction,
            max_time_of_impact,
            solid,
            query_filter(filter),
            |entity, intersection| {
                hits.push(PhysicsRayHit3d {
                    entity,
                    time_of_impact: intersection.time_of_impact,
                    point: intersection.point,
                    normal: intersection.normal,
                });
                true
            },
        );
        hits
    }

    pub fn intersect_point(&self, point: Vec3, filter: PhysicsQueryFilter) -> Vec<Entity> {
        let Ok(context) = self.context.single() else {
            return Vec::new();
        };
        let mut hits = Vec::new();
        context.intersect_point(point, query_filter(filter), |entity| {
            hits.push(entity);
            true
        });
        hits
    }

    pub fn project_point(
        &self,
        point: Vec3,
        max_distance: f32,
        solid: bool,
        filter: PhysicsQueryFilter,
    ) -> Option<PhysicsPointProjection3d> {
        let context = self.context.single().ok()?;
        let (entity, projection) =
            context.project_point(point, max_distance, solid, query_filter(filter))?;

        Some(PhysicsPointProjection3d {
            entity,
            point: projection.point,
            is_inside: projection.is_inside,
        })
    }

    pub fn intersect_shape(
        &self,
        position: Vec3,
        rotation: Quat,
        shape: &PhysicsCollider3d,
        filter: PhysicsQueryFilter,
    ) -> Vec<Entity> {
        let Ok(context) = self.context.single() else {
            return Vec::new();
        };
        let Some(shape) = convert::collider(shape) else {
            return Vec::new();
        };
        let mut hits = Vec::new();
        context.intersect_shape(
            position,
            rotation,
            &*shape.raw,
            query_filter(filter),
            |entity| {
                hits.push(entity);
                true
            },
        );
        hits
    }

    pub fn cast_shape(
        &self,
        position: Vec3,
        rotation: Quat,
        velocity: Vec3,
        shape: &PhysicsCollider3d,
        max_time_of_impact: f32,
        filter: PhysicsQueryFilter,
    ) -> Option<PhysicsShapeCastHit3d> {
        let context = self.context.single().ok()?;
        let shape = convert::collider(shape)?;
        let options = RapierShapeCastOptions {
            max_time_of_impact,
            ..Default::default()
        };
        let (entity, hit) = context.cast_shape(
            position,
            rotation,
            velocity,
            &*shape.raw,
            options,
            query_filter(filter),
        )?;

        Some(PhysicsShapeCastHit3d {
            entity,
            time_of_impact: hit.time_of_impact,
            details: hit.details.map(|details| PhysicsShapeCastHitDetails3d {
                witness1: details.witness1,
                witness2: details.witness2,
                normal1: details.normal1,
                normal2: details.normal2,
            }),
        })
    }
}

fn query_filter(filter: PhysicsQueryFilter) -> RapierQueryFilter<'static> {
    let mut flags = RapierQueryFilterFlags::empty();

    if filter.exclude_sensors {
        flags |= RapierQueryFilterFlags::EXCLUDE_SENSORS;
    }
    if filter.exclude_solids {
        flags |= RapierQueryFilterFlags::EXCLUDE_SOLIDS;
    }

    RapierQueryFilter {
        flags,
        groups: filter.groups.and_then(convert::collision_groups),
        exclude_collider: filter.exclude_collider,
        exclude_rigid_body: filter.exclude_rigid_body,
        predicate: None,
    }
}
