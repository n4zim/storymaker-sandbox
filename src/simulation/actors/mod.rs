/*
 * StoryMaker - Living world generation tool
 * Copyright © 2022-2023 Nazim Lachter
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

pub mod animations;
pub mod movement;
pub mod pathfinding;
pub mod spawner;

#[derive(Component)]
pub struct Actor {
  gender: ActorGender,
  direction: ActorDirection,
  posture: ActorPosture,
  posture_reverse: bool,
  path: Vec<TilePos>,
}

impl Actor {
  fn new(gender: ActorGender) -> Actor {
    Actor {
      gender,
      direction: ActorDirection::Bottom,
      posture: ActorPosture::Idle,
      posture_reverse: false,
      path: Vec::new(),
    }
  }

  fn get_texture_index(&self) -> TileTextureIndex {
    TileTextureIndex(
      8 + self.gender.to_u32() * 32
        + self.direction.to_u32()
        + 8 * self.posture.to_u32(),
    )
  }

  fn set_next_posture(&mut self) {
    self.posture = match self.posture {
      ActorPosture::LeftFoot => ActorPosture::Idle,
      ActorPosture::Idle => {
        self.posture_reverse = !self.posture_reverse;
        if self.posture_reverse {
          ActorPosture::LeftFoot
        } else {
          ActorPosture::RightFoot
        }
      }
      ActorPosture::RightFoot => ActorPosture::Idle,
    };
  }
}

pub enum ActorGender {
  Male,
  Female,
  Other,
}

impl ActorGender {
  fn new_with_index(index: i32) -> Option<Self> {
    match index {
      2 => Some(Self::Male),
      1 => Some(Self::Female),
      0 => Some(Self::Other),
      _ => None,
    }
  }

  fn to_u32(&self) -> u32 {
    match self {
      Self::Male => 2,
      Self::Female => 1,
      Self::Other => 0,
    }
  }
}

pub enum ActorDirection {
  Top,
  TopRight,
  Right,
  BottomRight,
  Bottom,
  BottomLeft,
  Left,
  TopLeft,
}

impl ActorDirection {
  fn to_u32(&self) -> u32 {
    match self {
      ActorDirection::Top => 0,
      ActorDirection::TopRight => 1,
      ActorDirection::Right => 2,
      ActorDirection::BottomRight => 3,
      ActorDirection::Bottom => 4,
      ActorDirection::BottomLeft => 5,
      ActorDirection::Left => 6,
      ActorDirection::TopLeft => 7,
    }
  }
}

pub enum ActorPosture {
  LeftFoot,
  Idle,
  RightFoot,
}

impl ActorPosture {
  fn to_u32(&self) -> u32 {
    match self {
      ActorPosture::LeftFoot => 0,
      ActorPosture::Idle => 1,
      ActorPosture::RightFoot => 2,
    }
  }
}
