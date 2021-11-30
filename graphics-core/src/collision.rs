// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[cfg(test)]
mod tests;

use crate::space::{Point, Triangle, Mesh, Line};

type Ray = Line;

pub trait Collidable {}
