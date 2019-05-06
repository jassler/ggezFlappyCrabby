use crate::entity::Entity;
use crate::Sprite;
use ggez::nalgebra::{Point2, Vector2};
use std::collections::VecDeque;

const NUM_PIPES: usize = 3;
const SEGMENTS: usize = 4;
/// Count total segments. The pipe lengths and tops.
pub const TOTAL: usize = (SEGMENTS * 2) + 2;

const PIPE_SPEED: f32 = 3.5;

const GAP: f32 = 45.0;
pub const PIPE_DV: f32 = 0.15;

pub fn pipe_position(t: f32) -> f32 {
    GAP + 10.0 + (t.sin() + 1.0) * ((600.0 / 4.0) - ((GAP + 10.0) * 2.0))

}

#[derive(Debug)]
pub struct PipeTracker {
    pipes_seen: usize,
    top: VecDeque<f32>,
    time: f32,
}

impl PipeTracker {
    pub fn new() -> Self {
        PipeTracker {
            pipes_seen: 1,
            top: VecDeque::new(),
            time: 0.0,
        }
    }

    fn get_pipe_top(&mut self) -> f32 {
        self.pipes_seen += 1;
        pipe_position(self.time)
    }

    fn init_get_pipe_top(&mut self) -> f32 {
				self.time += PIPE_DV;
        let result = self.get_pipe_top();
        self.pipes_seen = 0;
        self.top.push_back(result);
        result
    }

    // Returns the direction pipe has to move.
    pub fn get_pipe_difference(&mut self) -> f32 {
        // need to go back by the number of integers of other pipes.

        let last_pos = self.top.front().expect("Pipe wasn't placed!").clone();
        let now_pos = self.get_pipe_top();

        if (self.pipes_seen == 10) {
            self.top.pop_front();
            self.top.push_back(now_pos);
            self.pipes_seen = 0;
						self.time += PIPE_DV;
        }
        now_pos - last_pos
    }
}

fn create_pipe_bottom(
    sprite_base: Sprite,
    sprite_top: Sprite,
    x: f32,
    top: f32,
    total_dist: f32,
) -> Vec<Entity> {
    let top_height = sprite_top.height;

    let mut pipe_top = Entity::new().add_physics(false);
    pipe_top.sprite = Some(sprite_top);
    pipe_top.position = Point2::new(x, top);
    pipe_top.is_pipe = true;
    let pipe_top = pipe_top
        .scroller(total_dist)
        .set_velocity(ggez::nalgebra::Vector2::new(-PIPE_SPEED, 0.0));

    let segments = SEGMENTS;
    let mut p = (0..segments)
        .into_iter()
        .map(|i| {
            let mut pipe_bottom = Entity::new().add_physics(false);
            pipe_bottom.is_pipe = true;
            pipe_bottom.sprite = Some(sprite_base.clone());
            pipe_bottom.position =
                Point2::new(x, top + top_height + (sprite_base.height * (i as f32)));
            pipe_bottom
                .scroller(total_dist)
                .set_velocity(ggez::nalgebra::Vector2::new(-PIPE_SPEED, 0.0))
        })
        .collect::<Vec<Entity>>();
    p.push(pipe_top);
    p
}

pub fn create_pipes(
    sprite_base: Sprite,
    sprite_top: Sprite,
    pipe_tracker: &mut PipeTracker,
    x: f32,
) -> Vec<Entity> {
    let number_of_pipes = NUM_PIPES;
    let width = sprite_top.width;
    let space_width = width * 1.5;
    let total_dist = (width + space_width) * (number_of_pipes as f32);

    let gap = GAP;
    (0..number_of_pipes)
        .into_iter()
        .flat_map(|i| {
            let top = pipe_tracker.init_get_pipe_top();
            println!("REAL FIRST POS: {:?}", top);
            let mut bottom = create_pipe_bottom(
                sprite_base.clone(),
                sprite_top.clone(),
                x + (space_width + width) * (i as f32),
                top,
                total_dist,
            );
            bottom.extend(create_pipe_top(
                sprite_base.clone(),
                sprite_top.clone(),
                x + (space_width + width) * (i as f32),
                top - gap,
                total_dist,
            ));
            bottom
        })
        .collect()
}

fn create_pipe_top(
    sprite_base: Sprite,
    sprite_top: Sprite,
    x: f32,
    top: f32,
    total_dist: f32,
) -> Vec<Entity> {
    let top_height = sprite_top.height;

    let mut pipe_top = Entity::new().add_physics(false);
    let mut sp_top = sprite_top;
    sp_top.scale.y = -1.0;
    pipe_top.sprite = Some(sp_top);
    pipe_top.position = Point2::new(x, top);
    pipe_top.is_pipe = true;
    let pipe_top = pipe_top
        .scroller(total_dist)
        .set_velocity(ggez::nalgebra::Vector2::new(-PIPE_SPEED, 0.0));

    let segments = SEGMENTS;
    let mut p = (0..segments)
        .into_iter()
        .map(|i| {
            let mut pipe_bottom = Entity::new().add_physics(false);
            pipe_bottom.sprite = Some(sprite_base.clone());
            pipe_bottom.is_pipe = true;
            pipe_bottom.position =
                Point2::new(x, top - top_height - (sprite_base.height * (i as f32)));
            pipe_bottom
                .scroller(total_dist)
                .set_velocity(ggez::nalgebra::Vector2::new(-PIPE_SPEED, 0.0))
        })
        .collect::<Vec<Entity>>();
    p.push(pipe_top);
    p
}