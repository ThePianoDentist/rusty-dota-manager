extern crate rand;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Position{
	pub x: u32,
	pub y: u32
}

pub trait Distance{
	fn distance_between(&self, other_point: Position) -> f32;
	fn x_distance(&self, other_point: Position) -> u32;
	fn y_distance(&self, other_point: Position) -> u32;
	fn x_difference(&self, other_point: Position) -> i32;
	fn y_difference(&self, other_point: Position) -> i32;
}

impl Distance for Position{
	fn x_distance(&self, other_point: Position) -> u32{
		(self.x as i32 - other_point.x as i32).abs() as u32
	}
	
	fn y_distance(&self, other_point: Position) -> u32{
		(self.y as i32 - other_point.y as i32).abs() as u32
	}
	
	fn x_difference(&self, other_point: Position) -> i32{
		self.x as i32 - other_point.x as i32
	}
	
	fn y_difference(&self, other_point: Position) -> i32{
		self.y as i32 - other_point.y as i32
	}
	
	fn distance_between(&self, other_point: Position) -> f32{
		(self.y_distance(other_point).pow(2) as f32 + self.x_distance(other_point).pow(2) as f32).sqrt().abs()
	}
}

pub trait CoordManipulation{
	fn small_random_pos_offset(&mut self) -> Position;
	fn swap_x_y(&self) -> Position;
	fn alter_y(&self, i32) -> Position;
	fn alter_x(&self, i32) -> Position;
	fn update(&mut self, u32, u32);
	fn update_x(&mut self, u32);
	fn update_y(&mut self, u32);
	fn add_x(&mut self, i32);
	fn add_y(&mut self, i32);
}

impl CoordManipulation for Position{
	fn small_random_pos_offset(&mut self) -> Position{
		let rand_num = || rand::thread_rng().gen_range(0, 9) as i32 - 4;
		let new_x = (self.x as i32 + rand_num()) as u32;
		let new_y = (self.y as i32 + rand_num()) as u32;
		Position{x: new_x, y: new_y}
	}
	
	fn swap_x_y(&self) -> Position{
		Position{x: self.y, y: self.x}
	}
	
	fn alter_y(&self, y_change: i32) -> Position{
		Position{x: self.x, y: (self.y as i32 + y_change).abs() as u32}  // need do handling of if give negative coord better
	}
	
	fn alter_x(&self, x_change: i32) -> Position{
		Position{x: (self.x as i32 + x_change).abs() as u32, y: self.y}  // need do handling of if give negative coord better
	}
	
	fn update(&mut self, new_x: u32, new_y: u32){
		self.x = new_x;
		self.y = new_y;
	}
	
	fn update_x(&mut self, new_x: u32){
		self.x = new_x;
	}
	
	fn update_y(&mut self, new_y: u32){
		self.y = new_y;
	}
	
	fn add_x(&mut self, add: i32){
		let x = self.x as i32;
		self.x = (x + add) as u32;
	}
	
	fn add_y(&mut self, add: i32){
		let y = self.y as i32;
		self.y = (y + add) as u32;
	}
}

#[derive(Copy, Clone)]
pub struct Velocity{
	pub x: i32,
	pub y: i32
}
