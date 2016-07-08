extern crate rand;
extern crate opengl_graphics;
use rand::Rng;
//use opengl_graphics::{Texture as gTexture};

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

pub struct Game {
	pub game_tick: u64,
	pub game_time: f64,
	pub teams: [Team; 2]
}

pub struct Team{
	pub side: Side,
	pub towers: [Tower; 9],
	pub fountain: Fountain,
	pub throne: Throne,
	pub lane_creeps: Vec<Creep>,
	pub heroes: [Hero; 1]
}

#[derive(Copy, Clone)]
pub struct Creep {
	pub lane: Lane,
	pub hp: i32,
	pub attack_damage: u32,
	pub melee_attack: bool,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub position: Position,
	pub can_move: bool,
}

/// Buildings
pub struct Tower{
	pub lane: Lane,
	pub tier: u8,
	pub max_hp: u32,
	pub hp: i32,
	pub attack_damage: u32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub attacked_this_turn: bool,
	pub position: Position,
}

pub struct Throne{
	pub max_hp: u32,
	pub hp: i32,
	pub position: Position,
}

pub struct Fountain{
	pub attack_damage: u32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub attacked_this_turn: bool,
	pub position: Position
}

/// Heros
pub struct Hero{
	pub name: &'static str,
	pub pic: opengl_graphics::Texture,
	pub level: u32,
	pub base_int: i32,
	pub base_str: i32,
	pub base_agi: i32,
	pub int_gain: f32,
	pub str_gain: f32,
	pub agi_gain: f32,
	pub base_attack_damage: f32,
	pub move_speed: i32,
	pub hp_regen: f32,
	pub mana_regen: f32,
	
	pub gold: f32,
	pub hp: f32,
	pub mana: f32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub position: Position,
	pub armour: f32,
}

pub struct Skill{
	pub name: str,
}
	
pub trait TimeTick {
	fn new_game_tick(&mut self);
}

impl TimeTick for Game{
	fn new_game_tick(&mut self){
		self.game_tick += 1
	}
}

pub trait ResetAllAttacking{
	fn reset_all_attack_cooldown(&mut self);
}

impl ResetAllAttacking for Game{
	fn reset_all_attack_cooldown(&mut self){
		for i in 0..2{
			for creep in &mut self.teams[i].lane_creeps{creep.can_move = true;};
			for tower in &mut self.teams[i].towers{tower.attacked_this_turn = false;};
			self.teams[i].fountain.attacked_this_turn = false;
		}
	}
}

pub trait AttackCreeps{
	fn attack_enemy_creeps(&mut self, &mut Vec<Creep>);
}

impl AttackCreeps for Tower{
	fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
		if self.hp.is_positive(){
			for creep in &mut enemy_creeps.iter_mut(){
				if self.position.distance_between(creep.position) < 12.0{
					self.attack_cooldown -= 1.;
					if self.attack_cooldown < 0.0{
						creep.hp -= self.attack_damage as i32;
						self.attacked_this_turn = true;
						self.attack_cooldown += self.attack_rate;
					}
					break;
				}
			}
		}
	}
}

impl AttackCreeps for Fountain{
	fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
		for creep in &mut enemy_creeps.iter_mut(){
			if self.position.distance_between(creep.position) < 12.0{
				self.attack_cooldown -= 1.;
				if self.attack_cooldown < 0.0{
					creep.hp -= self.attack_damage as i32;
					self.attacked_this_turn = true;
					self.attack_cooldown += self.attack_rate;
				}
				break;
			}
		}
	}
}

impl AttackCreeps for Creep{
	fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
		if self.hp.is_positive(){
			for creep in &mut enemy_creeps.iter_mut(){
				if self.position.distance_between(creep.position) < 12.0{
					self.attack_cooldown -= 1.;
					self.can_move = false;
					if self.attack_cooldown < 0.0{
						creep.hp -= self.attack_damage as i32;
						self.attack_cooldown += self.attack_rate; //bug with attacking too fast?
					}
					break;
				}
			}
		}
	}
}
	
pub trait AttackBuilding{
	fn attack_towers(&mut self, &mut [Tower]);
	fn attack_throne(&mut self, &mut Throne);
}

impl AttackBuilding for Creep{
	fn attack_towers(&mut self, enemy_towers: &mut [Tower]){
		for tower in &mut enemy_towers.iter_mut(){
			if tower.hp.is_positive() && self.position.distance_between(tower.position) < 12.0{
				self.can_move = false;
				self.attack_cooldown -= 1.;
				if self.attack_cooldown < 0.0{
					tower.hp -= self.attack_damage as i32;
					self.attack_cooldown += self.attack_rate;
				}
				break;
			};
		};
	}
	
	fn attack_throne(&mut self, throne: &mut Throne){
		if throne.hp.is_positive() && self.position.distance_between(throne.position) < 12.0{
			self.can_move = false;
			self.attack_cooldown -= 1.;
			if self.attack_cooldown < 0.0{
				throne.hp -= self.attack_damage as i32;
				self.attack_cooldown += self.attack_rate;
			}
		};
	}
}

pub trait Move{
	fn move_top_creep_radiant(&mut self);
	fn move_mid_creep_radiant(&mut self);
	fn move_bot_creep_radiant(&mut self);
	fn move_top_creep_dire(&mut self);
	fn move_mid_creep_dire(&mut self);
	fn move_bot_creep_dire(&mut self);
}

impl Move for Creep{
	fn move_top_creep_radiant(&mut self){
		if self.position.y > (MAX_COORD / 8){
			self.position.y -= 1;
		}
		else{
			self.position.x +=1
		}
	}

	fn move_mid_creep_radiant(&mut self){
		if 0 < self.position.y{
			if self.position.x < MAX_COORD{
				self.position.y -= 1;
				self.position.x += 1
			};
		};
	}

	fn move_bot_creep_radiant(&mut self){
		if self.position.x < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.x += 1;
		}
		else{
			if 0 < self.position.y{
				self.position.y -=1;
			}
		}
	}

	fn move_mid_creep_dire(&mut self){
		if self.position.y  < MAX_COORD{
			if 0 < self.position.x{
				self.position.y += 1;
				self.position.x -= 1
			};
		};
	}

	fn move_bot_creep_dire(&mut self){
		if self.position.y < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.y += 1;
		}
		else{
			if 0 < self.position.x{
				self.position.x -=1
			}
		}
	}

	fn move_top_creep_dire(&mut self){
		if self.position.x > MAX_COORD / 8{
			self.position.x -= 1;
		}
		else{
			self.position.y +=1
		}
	}
}

pub trait MoveCreeps{
	fn move_creeps_radiant(&mut self);
	fn move_creeps_dire(&mut self);
}

impl MoveCreeps for Team{
	fn move_creeps_radiant(&mut self){
		for lane_creep in &mut self.lane_creeps{
			if lane_creep.can_move{
				match lane_creep.lane{
					Lane::Top => lane_creep.move_top_creep_radiant(),
					Lane::Mid => lane_creep.move_mid_creep_radiant(),
					Lane::Bot => lane_creep.move_bot_creep_radiant(),
				};
			}
		}
	}
	
	fn move_creeps_dire(&mut self){
		for lane_creep in &mut self.lane_creeps{
			if lane_creep.can_move{
				match lane_creep.lane{
					Lane::Top => lane_creep.move_top_creep_dire(),
					Lane::Mid => lane_creep.move_mid_creep_dire(),
					Lane::Bot => lane_creep.move_bot_creep_dire(),
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
		for i in 0..2{self.teams[i].lane_creeps.retain(|&i| i.hp > 0)};
	}
}
