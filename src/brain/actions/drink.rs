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

use crate::{
  brain::states::thirst, game::GameTick, markers, pathfinding::paths,
  world::WorldMap,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

#[derive(Component, ActionBuilder, Clone, Debug)]
pub struct Drink {
  speed: f32,
}

impl Drink {
  pub fn new(speed: f32) -> Self {
    Self { speed }
  }
}

pub fn action(
  mut events: EventReader<GameTick>,
  world: Res<WorldMap>,
  mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
  mut thirsts: Query<(&TilePos, &mut thirst::Thirst), Without<markers::Water>>,
  waters: Query<&TilePos, With<markers::Water>>,
) {
  for _clock in events.iter() {
    for (Actor(actor), mut state, action, span) in &mut query {
      let _guard = span.span().enter();
      let (position, mut thirst) =
        thirsts.get_mut(*actor).expect("actor has no thirst");
      //println!("Drink state: {:?} with thirst {:?}", state, thirst.current);
      match *state {
        ActionState::Requested => {
          debug!("[REQUEST] Drinking from {:?}", position);
          if let Some(path) =
            paths(&world, &position, &waters.iter().cloned().collect())
          {
            if path[path.len() - 2] == *position {
              *state = ActionState::Executing;
            } else {
              trace!("[REQUESTED] Not close enough to water");
              *state = ActionState::Failure;
            }
          } else {
            trace!("[REQUESTED] No path found to water from {:?}", position);
            *state = ActionState::Failure;
          }
        }
        ActionState::Executing => {
          thirst.current -= action.speed;
          if thirst.current <= 0.0 {
            thirst.current = 0.0;
            debug!("[EXECUTED] Drank from {:?}", position);
            *state = ActionState::Success;
          }
        }
        ActionState::Cancelled => {
          trace!("[CANCEL] Stopped drinking from {:?}", position);
          *state = ActionState::Failure;
        }
        _ => {}
      }
    }
  }
}
