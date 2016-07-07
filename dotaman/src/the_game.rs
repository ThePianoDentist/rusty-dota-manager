extern crate rand;
use rand::Rng;

pub const MAX_COORD: u32  = 600;

#[derive(Copy, Clone)]
pub struct Position{
	pub x: u32,
	pub y: u32
}

pub trait Distance{
	fn distance_between(&self, other_point: Position) -> f32;
	fn x_distance(&self, other_point: Position) -> u32;
	fn y_distance(&self, other_point: Position) -> u32;
}

impl Distance for Position{
	fn x_distance(&self, other_point: Position) -> u32{
		(self.x as i32 - other_point.x as i32).abs() as u32
	}
	
	fn y_distance(&self, other_point: Position) -> u32{
		(self.y as i32 - other_point.y as i32).abs() as u32
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
}

impl CoordManipulation for Position{
	fn small_random_pos_offset(&mut self) -> Position{
		let rand_num = || rand::thread_rng().gen_range(0, 8) as i32 - 4;
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
}

#[derive(Copy, Clone)]
pub enum Lane{
	Top,
	Mid,
	Bot
}

#[derive(Copy, Clone, PartialEq)]
pub enum Side{
	Radiant,
	Dire
}

#[derive(Copy, Clone)]
pub struct Creep {
	pub side: Side,
	pub lane: Lane,
	pub hp: i32,
	pub attack_damage: u32,
	pub melee_attack: bool,
	pub attacking: bool,
	pub position: Position,
}

pub struct Game {
	pub game_tick: u64,
	pub lane_creeps: Vec<Creep>,
	pub towers: [Tower; 18],
	pub fountains: [Fountain; 2],
	pub thrones: [Throne; 2]
}
	
pub trait TimeTick {
	fn new_game_tick(&mut self);
}

impl TimeTick for Game{
	fn new_game_tick(&mut self){
		self.game_tick += 1
	}
}

pub struct Tower{
	pub side: Side,
	pub lane: Lane,
	pub tier: u8,
	pub max_hp: u32,
	pub hp: i32,
	pub attack_damage: u32,
	pub attacked: bool,
	pub position: Position,
}

pub trait Attack{
	fn towers_attack(&mut self);
	
	fn lane_creeps_attack(&mut self);
	
	fn fountains_attack(&mut self);
}

impl Attack for Game{
	fn towers_attack(&mut self){
		for tower in &mut self.towers{
			tower.attacked = false;
			if tower.hp.is_positive(){
				for creep in &mut self.lane_creeps{
					if tower.position.distance_between(creep.position) < 12.0 && creep.side != tower.side && !tower.attacked{
						creep.hp -= tower.attack_damage as i32;
						tower.attacked = true;
						break;
					}
				}
			}
		}
	}
	
	fn lane_creeps_attack(&mut self){
		let clone = self.lane_creeps.clone();
		for (i, our_creep) in clone.clone().iter().enumerate(){
			let mut our_creep_attacked = false;
			let our_side: Side = our_creep.side;
			for other_creep in &mut self.lane_creeps{
				if other_creep.side != our_side && our_creep.position.distance_between(other_creep.position) < 12.0{
					other_creep.hp -= our_creep.attack_damage as i32;
					our_creep_attacked = true;
					break;
				};
			}
			
			if !our_creep_attacked{
				for tower in &mut self.towers{
					if tower.side != our_side && tower.hp.is_positive() && our_creep.position.distance_between(tower.position) < 12.0{
						tower.hp -= our_creep.attack_damage as i32;
						our_creep_attacked = true;
						break;
					};
				};
			}
			
			if !our_creep_attacked{
				for throne in &mut self.thrones{
					if throne.side != our_side && throne.hp.is_positive() && our_creep.position.distance_between(throne.position) < 12.0{
						throne.hp -= our_creep.attack_damage as i32;
						our_creep_attacked = true;
						break;
					};
				};
			}
			self.lane_creeps[i].attacking = our_creep_attacked;
			
		};
	}
	
	fn fountains_attack(&mut self){
		for fountain in &mut self.fountains{
			fountain.attacked = false;
			for creep in &mut self.lane_creeps{
				if fountain.position.distance_between(creep.position) < 40.0 && creep.side != fountain.side && !fountain.attacked{
					creep.hp -= fountain.attack_damage as i32;
					fountain.attacked = true;
					break;
				}
			}
		}
	}
}

pub trait Move{
	fn move_top_creeps_radiant(&mut self);
	fn move_mid_creeps_radiant(&mut self);
	fn move_bot_creeps_radiant(&mut self);
	fn move_top_creeps_dire(&mut self);
	fn move_mid_creeps_dire(&mut self);
	fn move_bot_creeps_dire(&mut self);
}

impl Move for Creep{
	fn move_top_creeps_radiant(&mut self){
		if self.position.y > (MAX_COORD / 8){
			self.position.y -= 1;
		}
		else{
			self.position.x +=1
		}
	}

	fn move_mid_creeps_radiant(&mut self){
		if 0 < self.position.y{
			if self.position.x < MAX_COORD{
				self.position.y -= 1;
				self.position.x += 1
			};
		};
	}

	fn move_bot_creeps_radiant(&mut self){
		if self.position.x < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.x += 1;
		}
		else{
			if 0 < self.position.y{
				self.position.y -=1;
			}
		}
	}

	fn move_mid_creeps_dire(&mut self){
		if self.position.y  < MAX_COORD{
			if 0 < self.position.x{
				self.position.y += 1;
				self.position.x -= 1
			};
		};
	}

	fn move_bot_creeps_dire(&mut self){
		if self.position.y < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.y += 1;
		}
		else{
			if 0 < self.position.x{
				self.position.x -=1
			}
		}
	}

	fn move_top_creeps_dire(&mut self){
		if self.position.x > MAX_COORD / 8{
			self.position.x -= 1;
		}
		else{
			self.position.y +=1
		}
	}
}

pub trait MoveCreeps{
	fn move_creeps(&mut self);
}

impl MoveCreeps for Game{
	fn move_creeps(&mut self){
		for lane_creep in &mut self.lane_creeps{
			if !lane_creep.attacking{
				match lane_creep.side{
					Side::Radiant => match lane_creep.lane{
						Lane::Top => lane_creep.move_top_creeps_radiant(),
						Lane::Mid => lane_creep.move_mid_creeps_radiant(),
						Lane::Bot => lane_creep.move_bot_creeps_radiant()},
					_ => match lane_creep.lane{
							Lane::Top => lane_creep.move_top_creeps_dire(),
							Lane::Mid => lane_creep.move_mid_creeps_dire(),
							Lane::Bot => lane_creep.move_bot_creeps_dire()
						},
				};
			}
		}
	}
}

pub trait KillOff{
	fn kill_off_creeps(&mut self);
}

impl KillOff for Game{
	fn kill_off_creeps(&mut self){
		&mut self.lane_creeps.retain(|&i| i.hp > 0);
	}
}

pub struct Throne{
	pub max_hp: u32,
	pub hp: i32,
	pub side: Side,
	pub position: Position,
}

pub struct Fountain{
	pub side: Side,
	pub attack_damage: u32,
	pub attacked: bool,
	pub position: Position
}
