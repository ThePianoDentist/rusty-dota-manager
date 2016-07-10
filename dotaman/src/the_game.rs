extern crate rand;
extern crate opengl_graphics;
use position::*;
use anhero::*;

pub const MAX_COORD: f32  = 600.0;

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
	pub game_time: f32,
	pub teams: [Team; 2],
}

pub struct Team{
	pub side: Side,
	pub towers: Vec<Tower>,
	pub fountain: Fountain,
	pub throne: Throne,
	pub lane_creeps: Vec<Creep>,
	pub heroes: [Hero; 5]
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
	pub can_action: bool,
	pub range: f32,
}

// Buildings
#[derive(Copy, Clone)]
pub struct Tower{
	pub lane: Lane,
	pub tier: u8,
	pub max_hp: u32,
	pub hp: i32,
	pub attack_damage: u32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub can_action: bool,
	pub position: Position,
	pub range: f32,
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
	pub can_action: bool,
	pub position: Position,
	pub range: f32,
	pub hp: i32 // temp hack to keep consistent with tower code
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
			for creep in &mut self.teams[i].lane_creeps{creep.can_action = true;};
			for tower in &mut self.teams[i].towers{tower.can_action = true;};
			for hero in &mut self.teams[i].heroes{
				hero.respawn_timer -= 1;
				if hero.respawn_timer <=0 {hero.can_action = true};
				};
			self.teams[i].fountain.can_action = true;
		}
	}
}

pub trait AttackCreeps{
	fn attack_enemy_creeps(&mut self, &mut Vec<Creep>);
}


macro_rules! impl_AttackCreeps {
    ($T:ident) => {
		impl AttackCreeps for $T{
			fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
				for creep in &mut enemy_creeps.iter_mut(){
					if self.position.distance_between(creep.position) < self.range{
						self.attack_cooldown -= 1.;
						self.can_action = false;
						if self.attack_cooldown < 0.0{
							creep.hp -= self.attack_damage as i32;
							self.attack_cooldown += self.attack_rate;
						}
						break;
					}
				}
			}
		}
	}
}

impl_AttackCreeps!(Tower);
impl_AttackCreeps!(Fountain);
impl_AttackCreeps!(Creep);

pub trait AttackClosestHero{
	fn attack_closest_hero(&mut self, &mut [Hero]);

	fn attack_hero(&mut self, &mut Hero);
}

macro_rules! impl_AttackClosestHero{
	($T: ident) => {
	impl AttackClosestHero for $T{
			fn attack_closest_hero(&mut self, enemy_heroes: &mut [Hero]){
				for hero in &mut enemy_heroes.iter_mut(){
					if self.position.distance_between(hero.position) < self.range && self.can_action{
						self.can_action = false;
						self.attack_cooldown -= 1.;
						if self.attack_cooldown < 0.0{
							hero.hp -= self.attack_damage as f32;
							self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
						}
						break;
					}
				}
			}

			fn attack_hero(&mut self, hero: &mut Hero){
				if self.position.distance_between(hero.position) < self.range && self.can_action{
					self.can_action = false;
					self.attack_cooldown -= 1.;
					if self.attack_cooldown < 0.0{
						hero.hp -= self.attack_damage as f32;
						self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
					}
				}
			}
		}
	}
}

impl_AttackClosestHero!(Tower);
impl_AttackClosestHero!(Creep);
impl_AttackClosestHero!(Fountain);

pub trait AttackBuilding{
	fn attack_towers(&mut self, &mut Vec<Tower>);
	fn attack_throne(&mut self, &mut Throne);
}

macro_rules! impl_AttackBuilding {
    ($T:ident) => {
        impl AttackBuilding for $T {
            fn attack_towers(&mut self, enemy_towers: &mut Vec<Tower>){
				for tower in &mut enemy_towers.iter_mut(){
					if self.position.distance_between(tower.position) < self.range{
						self.can_action = false;
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
				if self.position.distance_between(throne.position) < self.range{
					self.can_action = false;
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

pub trait Travel{
	fn travel(&mut self);
}

macro_rules! impl_Travel{
	($T:ident) =>
	{
		impl Travel for $T{  // do handling to keep in bounds here
			fn travel(&mut self){
				self.position.y += self.velocity.y;
				self.position.x += self.velocity.x;
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
		if self.position.small_random_pos_offset().y < (MAX_COORD / 8.){
			self.velocity = Velocity{x: 1., y: 0.};
		}
	}

	fn move_mid_creep_radiant(&mut self){
		self.travel();
	}

	fn move_bot_creep_radiant(&mut self){
		self.travel();
		if self.position.small_random_pos_offset().x > MAX_COORD *(7.0/8.0){
			self.velocity = Velocity{x: 0., y: -1.};
		}
	}

	fn move_mid_creep_dire(&mut self){
		self.travel();
	}

	fn move_bot_creep_dire(&mut self){
		self.travel();
		if self.position.y > MAX_COORD *(7.0/8.0){
			self.velocity = Velocity{x: -1.0, y: 0.0};
		}
	}

	fn move_top_creep_dire(&mut self){
		self.travel();
		if self.position.x < MAX_COORD / 8.0{
			self.velocity = Velocity{x: 0.0, y: 1.0};
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
			if lane_creep.can_action{
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
			if lane_creep.can_action{
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
	fn kill_off_heroes(&mut self);
}

impl KillOff for Game{
	fn kill_off_creeps(&mut self){
		for i in 0..2{self.teams[i].lane_creeps.retain(|&i| i.hp > 0)};
	}

	fn kill_off_heroes(&mut self){
		for i in 0..2{
			for hero in self.teams[i].heroes.iter_mut(){
				if hero.hp <= 0.{
					let radiant_fount_pos = Position{x: (MAX_COORD/16.7).round(), y:(MAX_COORD - (MAX_COORD/16.7)).round()};
					hero.position = match i{
						0 => radiant_fount_pos,
						1 => radiant_fount_pos.swap_x_y(),
						_ => radiant_fount_pos //not possible
					};
					hero.respawn_timer = 60;
					hero.can_action = false;
					hero.gold -= 20.; //add handling to not let overflow and proper func
					hero.hp = hero.max_hp;
					hero.mana = hero.max_mana;
				}
			}
		};
	}
}
