extern crate rand;
extern crate opengl_graphics;
use rand::Rng;
use position::*;

pub const MAX_COORD: u32  = 600;

#[derive(Copy, Clone, PartialEq)]
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
	pub teams: [Team; 2],
	pub top_lane_vertex: Position,
	pub bot_lane_vertex: Position,
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
	pub velocity: Velocity,
	pub can_move: bool,
}

// Buildings
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

// Heroes
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
	
	pub can_move: bool,
	pub max_mana: f32,
	pub max_hp: f32,
	pub attack_damage: f32,
	pub gold: f32,
	pub hp: f32,
	pub mana: f32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub position: Position,
	pub armour: f32,
	pub velocity: Velocity,
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

macro_rules! impl_AttackBuilding { 
    ($T:ident) => {
        impl AttackBuilding for $T {
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
    }
}

impl_AttackBuilding!(Creep);
impl_AttackBuilding!(Hero);

pub trait Travel{
	fn travel(&mut self);
}

macro_rules! impl_Travel{
	($T:ident) =>
	{
		impl Travel for $T{  // do handling to keep in bounds here
			fn travel(&mut self){
				self.position.y = (self.position.y as i32 + self.velocity.y) as u32;
				self.position.x = (self.position.x as i32 + self.velocity.x) as u32;
			}
		}
	}
}

impl_Travel!(Hero);
impl_Travel!(Creep);

pub trait MoveDownLane{
	fn move_top_creep_radiant(&mut self);
	fn move_mid_creep_radiant(&mut self);
	fn move_bot_creep_radiant(&mut self);
	fn move_top_creep_dire(&mut self);
	fn move_mid_creep_dire(&mut self);
	fn move_bot_creep_dire(&mut self);
}

impl MoveDownLane for Creep{
	fn move_top_creep_radiant(&mut self){
		self.travel();
		if self.position.small_random_pos_offset().y < (MAX_COORD / 8){
			self.velocity = Velocity{x: 1, y: 0};
		}
	}

	fn move_mid_creep_radiant(&mut self){
		self.travel();
	}

	fn move_bot_creep_radiant(&mut self){
		self.travel();
		if self.position.small_random_pos_offset().x > (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.velocity = Velocity{x: 0, y: -1};
		}
	}

	fn move_mid_creep_dire(&mut self){
		self.travel();
	}

	fn move_bot_creep_dire(&mut self){
		self.travel();
		if self.position.y > (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.velocity = Velocity{x: -1, y: 0};
		}
	}

	fn move_top_creep_dire(&mut self){
		self.travel();
		if self.position.x < MAX_COORD / 8{
			self.velocity = Velocity{x: 0, y: 1};
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

pub trait MoveHero{
	fn random_move(&mut self);
	
	fn move_towards_creeps(&mut self, Lane, &Vec<Creep>, bot_corner: &Position, top_corner: &Position);
	
	fn move_directly_to_creep(&mut self, closest_creep: &Creep);
	
	fn move_directly_to(&mut self, destination: Position, creep: &Creep, correct_x: bool);
}

impl MoveHero for Hero{
	fn random_move(&mut self){
		let rand_dir = || rand::thread_rng().gen_range(-1, 2) as i32;
		let rand_x = rand_dir();
		let rand_y = rand_dir();
		self.position.y = match self.position.y{
			600 => (self.position.y as i32 - 1) as u32,
			0 => self.position.y + 1,
			_ => (self.position.y as i32 + rand_y) as u32
		};
		self.position.x = match self.position.x{
			600 => (self.position.x as i32 - 1) as u32,
			0 => self.position.x + 1,
			_ => (self.position.x as i32 + rand_x) as u32
		};
	}
	
	fn move_directly_to_creep(&mut self, closest_creep: &Creep){
		//let closest_creep = lane_creeps.into_iter().filter(|&x| x.lane == lane).collect::<Vec<&Creep>>()[0];  //change to retain
		let (x_diff, y_diff) = (self.position.x as i32 - closest_creep.position.x as i32,
								self.position.y as i32 - closest_creep.position.y as i32);
		
		self.position.x = match x_diff{
			x if x > 0 => (self.position.x - 1) as u32,
			x if x < 0 => self.position.x + 1,
			_ => self.position.x // must be 0, in line with
		};
		
		self.position.y = match y_diff{
			y if y > 0 => (self.position.y - 1) as u32,
			y if y < 0 => self.position.y + 1,
			_ => self.position.y // must be 0, in line with
		};
	}
	
	fn move_directly_to(&mut self, destination: Position, creep: &Creep, correct_x: bool){
		
		if correct_x{
			match self.position.x_distance(creep.position){
				x if x <= 10 => {
					self.move_directly_to_creep(creep)
				},
				_ => {
					self.position.x = (self.position.x as i32 + creep.velocity.y) as u32;
					self.position.y = (self.position.y as i32 + creep.velocity.x) as u32;
				},
			}
		}
		
		else{
			match self.position.y_distance(creep.position){
				y if y <= 10 => {
					self.move_directly_to_creep(creep);
				},
				_ => {
					self.position.x = (self.position.x as i32 + creep.velocity.y) as u32;
					self.position.y = (self.position.y as i32 + creep.velocity.x) as u32;
				},
			}
		}
		
		
		//let (x_diff, y_diff) = (self.position.x as i32 - destination.x as i32,
								//self.position.y as i32 - destination.y as i32);
		
		//self.position.x = match x_diff{
			//x if x > 0 => (self.position.x - 1) as u32,
			//x if x < 0 => self.position.x + 1,
			//_ => self.position.x // must be 0, in line with
		//};
		
		//self.position.y = match y_diff{
			//y if y > 0 => (self.position.y - 1) as u32,
			//y if y < 0 => self.position.y + 1,
			//_ => self.position.y // must be 0, in line with
		//};
	}
	
	// assume first creep in vector is in first wave so closest. yolo
	fn move_towards_creeps(&mut self, lane: Lane, lane_creeps: &Vec<Creep>, top_corner: &Position, bot_corner: &Position){
		let closest_creeps = lane_creeps.into_iter().filter(|&x| x.lane == lane).collect::<Vec<&Creep>>();  //change to retain
		if closest_creeps.len() > 0{
			let closest_creep = closest_creeps[0];
			let correct_corner = match lane{
				Lane::Bot => Some(bot_corner),
				Lane::Top => Some(top_corner),
				_ => None
			};
			let distance_away = self.position.distance_between(closest_creep.position);
			if !correct_corner.is_none() && self.position.distance_between(closest_creep.position) > 100.0 {
				let corner = *correct_corner.unwrap();
				let (x_diff_corner, y_diff_corner) = (self.position.x_difference(corner),
				 self.position.y_difference(corner));
				match (x_diff_corner, y_diff_corner){
					(x, y) if x.abs() <= 10 => self.move_directly_to(corner, &closest_creep, true),
					(x, y) if y.abs() <= 10 => self.move_directly_to(corner, &closest_creep, false),
					(x, y) if x.abs() > y.abs() && y > 0 => self.position.add_y(-1),
					(x, y) if x.abs() > y.abs() && y < 0 => self.position.add_y(1),
					//(x, y) if x.abs() > y.abs() && -30 < x && x < 30 => self.move_directly_to_creep(lane, lane_creeps),
					(x, y) if x.abs() < y.abs() && x > 0 => self.position.add_x(-1),
					(x, y) if x.abs() < y.abs() && x < 0 => self.position.add_x(1),
					//(x, y) if x.abs() < y.abs() && -30 < y && y < 30 => self.move_directly_to_creep(lane, lane_creeps),
					_ => self.move_directly_to_creep(&closest_creep)
				}
			}
			else{
				self.move_directly_to_creep(&closest_creep);
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
