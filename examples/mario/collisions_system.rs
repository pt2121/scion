use legion::*;
use scion::core::components::{
    material::Material,
    maths::{
        collider::{Collider, ColliderMask},
        transform::Transform,
    },
};

use crate::Hero;

#[system(for_each)]
pub(crate) fn collider(
    collider: &mut Collider,
    hero: &mut Hero,
    material: &mut Material,
    transform: &Transform,
) {
    if let Material::Color(_c) = material {
        collider.collisions().iter().for_each(|collision| {
            match collision.mask() {
                ColliderMask::Death => std::process::exit(0),
                ColliderMask::Landscape => {
                    if collision.coordinates().y() > transform.global_translation().y() {
                        hero.landed = true;
                    }
                }
                _ => {}
            }
        });
    }
}
