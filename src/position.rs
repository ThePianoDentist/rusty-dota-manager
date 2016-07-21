extern crate rand;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub struct Position{
	pub x: f32,
	pub y: f32
}

pub trait Distance{
	fn distance_between(&self, other_point: Position) -> f32;
	fn x_distance(&self, other_point: Position) -> f32;
	fn y_distance(&self, other_point: Position) -> f32;
	fn x_difference(&self, other_point: Position) -> f32;
	fn y_difference(&self, other_point: Position) -> f32;
}

impl Distance for Position{
	fn x_distance(&self, other_point: Position) -> f32{
		(self.x - other_point.x).abs()
	}

	fn y_distance(&self, other_point: Position) -> f32{
		(self.y - other_point.y).abs()
	}

	fn x_difference(&self, other_point: Position) -> f32{
		self.x - other_point.x
	}

	fn y_difference(&self, other_point: Position) -> f32{
		self.y - other_point.y
	}

	fn distance_between(&self, other_point: Position) -> f32{
		(self.y_distance(other_point).powf(2.) + self.x_distance(other_point).powf(2.)).sqrt().abs()
	}
}

pub trait CoordManipulation{
	fn small_random_pos_offset(&mut self) -> Position;
	fn swap_x_y(&self) -> Position;
	fn alter_y(&self, f32) -> Position;
	fn alter_x(&self, f32) -> Position;
	fn update(&mut self, f32, f32);
	fn update_x(&mut self, f32);
	fn update_y(&mut self, f32);
	fn add_x(&mut self, f32);
	fn add_y(&mut self, f32);
	fn 	velocity_to(&mut self, towards: Position) -> Velocity;
}

impl CoordManipulation for Position{
	fn small_random_pos_offset(&mut self) -> Position{
		let rand_num = || (rand::thread_rng().gen_range(0, 9)  - 4) as f32;
		let new_x = self.x  + rand_num();
		let new_y = self.y  + rand_num();
		Position{x: new_x, y: new_y}
	}

	fn swap_x_y(&self) -> Position{
		Position{x: self.y, y: self.x}
	}

	fn alter_y(&self, y_change: f32) -> Position{
		Position{x: self.x, y: (self.y  + y_change).abs()}  // need do handling of if give negative coord better
	}

	fn alter_x(&self, x_change: f32) -> Position{
		Position{x: (self.x  + x_change).abs() , y: self.y}  // need do handling of if give negative coord better
	}

	fn update(&mut self, new_x: f32, new_y: f32){
		self.x = new_x;
		self.y = new_y;
	}

	fn update_x(&mut self, new_x: f32){
		self.x = new_x;
	}

	fn update_y(&mut self, new_y: f32){
		self.y = new_y;
	}

	fn add_x(&mut self, add: f32){
		self.x += add;
	}

	fn add_y(&mut self, add: f32){
		self.y += add;
	}

	fn velocity_to(&mut self, towards: Position) -> Velocity{
		Velocity{x: towards.x - self.x, y: towards.y - self.y}
	}
}

#[derive(Copy, Clone)]
pub struct Velocity{
	pub x: f32,
	pub y: f32
}
