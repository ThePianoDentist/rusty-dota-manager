use the_game::*;
use position::*;
extern crate opengl_graphics;
extern crate rand;
use rand::Rng;
use hero_ai::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use neutral_creeps::*;

const MAGIC_NUMBER: f32 = MAX_COORD *(7.0/8.0);
static TOP_LANE_VERTEX: Position = Position{x: MAX_COORD - MAGIC_NUMBER, y: MAX_COORD - MAGIC_NUMBER};
static BOT_LANE_VERTEX: Position = Position{x: MAGIC_NUMBER, y: MAGIC_NUMBER};
static RADIANT_FOUNT_POS: Position = Position{x: MAX_COORD/20.0, y:MAX_COORD - (MAX_COORD/20.0)};
static DIRE_FOUNT_POS: Position = Position{x: MAX_COORD - MAX_COORD/20.0, y:MAX_COORD/20.0};

#[derive(Copy, Clone, PartialEq)]
pub enum HeroType{
	Strength,
	Intelligence,
	Agility
}

// Heroes
pub struct Hero{
	pub name: &'static str,
	pub pic: opengl_graphics::Texture,
	pub hero_type: HeroType,
	pub level: u32,
	pub xp: f32,
	pub base_int: f32,
	pub base_str: f32,
	pub base_agi: f32,
	pub int_gain: f32,
	pub str_gain: f32,
	pub agi_gain: f32,
	pub base_attack_damage: f32,
	pub move_speed: f32,
	pub hp_regen: f32,
	pub mana_regen: f32,
	pub base_hp: f32,
	pub base_mana : f32,
	pub base_hp_regen: f32,
	pub base_mana_regen: f32,
	pub strength: f32,
	pub intelligence: f32,
	pub agility: f32,

	pub can_action: bool,
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
	pub velocity: Velocity, //not really being used
	pub respawn_timer: i32,
	pub range: f32,
    pub decisions: HashMap<Action, f32>,
	pub current_decision: Action,
	pub should_change_decision: bool,
	pub side: Side,
	pub priority: u32,
}

// This is so that when we are looping through our own heroes, we are still allowed access to info on them
pub struct HeroInfo{
	pub position: Position,
	pub priority: u32,
	pub hp: f32,
	pub respawn_timer: i32,
	pub current_decision: Action,
}

pub trait HeroToHeroInfo{
	fn hero_to_hero_info(&self) -> HeroInfo;
}

impl HeroToHeroInfo for Hero{
	fn hero_to_hero_info(&self) -> HeroInfo{
		HeroInfo{position: self.position, priority: self.priority, hp: self.hp, respawn_timer: self.respawn_timer,
		current_decision: self.current_decision}
	}
}

pub struct Skill{
	pub name: str,
}

pub enum Rune{
	DoubleDamage,
	Haste,
	Regen,
	Illusion,
	Bounty,
}

impl_AttackBuilding!(Hero);
impl_AttackClosestHero!(Hero);
impl_AttackCreeps!(Hero);
/*impl AttackCreeps for Hero{
	fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
		for creep in enemy_creeps.iter_mut(){
			if self.position.distance_between(creep.position) < self.range{
				self.attack_cooldown -= 1.;
				self.can_action = false;
				if self.attack_cooldown < 0.0{
					creep.hp -= self.attack_damage as i32;
					if creep.hp < 0{
						self.gold += 40.
					}
					self.attack_cooldown += self.attack_rate;
				}
				break;
			}
		}
	}

	fn attack_neutral(&mut self, neutral: &mut NeutralCamp){
		self.hp -= neutral.attack_damage;
		self.attack_cooldown -= 1.;
		self.can_action = false;
		if self.attack_cooldown < 0.0{
			neutral.hp -= self.attack_damage;
			if neutral.hp < 0.{
				self.gold += 40.
			}
			self.attack_cooldown += self.attack_rate;
		}
	}
}
*/

pub trait Stats{
	fn update_hp(&mut self);
	fn update_mana(&mut self);
	fn update_mana_regen(&mut self);
	fn update_hp_regen(&mut self);
	fn update_attack_damage(&mut self);
	fn update_level(&mut self);
}

impl Stats for Hero{
	fn update_hp(&mut self){
		let current_hp_percent = self.hp / self.max_hp;
		self.max_hp = self.base_hp + (self.base_str + self.level as f32 * self.str_gain) * 20.;
		self.hp = self.max_hp * current_hp_percent;
	}

	fn update_mana(&mut self){
		let current_mana_percent = self.mana / self.max_mana;
		self.max_mana = self.base_mana + (self.base_int + self.level as f32 * self.int_gain) * 12.;
		self.mana = self.max_mana * current_mana_percent;
	}

	fn update_mana_regen(&mut self){
		self.mana_regen = self.base_mana_regen + (self.base_int + self.level as f32 * self.int_gain) * 0.04;
	}

	fn update_hp_regen(&mut self){
		self.hp_regen = self.base_hp_regen + (self.base_str + self.level as f32 * self.str_gain) * 0.03;
	}

	fn update_attack_damage(&mut self){
		self.attack_damage = match self.hero_type{
			t if t == HeroType::Strength => self.base_attack_damage + self.strength,
			t if t == HeroType::Intelligence => self.base_attack_damage + self.intelligence,
			_ => self.base_attack_damage + self.agility,
		}
	}

	fn update_level(&mut self){
		self.level = self.level;
	}
}

trait MoveToAttackTower{
	fn move_to_attack_tower(&mut self, &mut Vec<Tower>, Lane);
}

impl MoveToAttackTower for Hero{
	fn move_to_attack_tower(&mut self, towers: &mut Vec<Tower>, lane: Lane){
		// bugged until implement killing off of towers
		let closest_tower = &towers.clone().into_iter().filter(|&x| x.lane == lane).collect::<Vec<Tower>>()[0];
		if self.position.distance_between(closest_tower.position) > self.range{
			self.move_directly_to(&closest_tower.position);
		}
		else{
			self.attack_towers(towers);
		}
	}
}
//impl_Travel!(Hero);

pub trait MoveHero{
	fn random_move(&mut self);

	fn move_towards_creeps(&mut self, Lane, &Vec<Creep>);

	fn move_directly_to_creep(&mut self, closest_creep: &Creep);

	fn move_to_creep_along_lane(&mut self, creep: &Creep, correct_x: bool);

	fn move_directly_to(&mut self, position: &Position);

	fn move_defensively_to(&mut self, position: &Position);

	//fn move_to_enemy_tower(&mute self, )
}

impl MoveHero for Hero{
	fn random_move(&mut self){
		if self.can_action{
			let rand_dir = || rand::thread_rng().gen_range(-1, 2) as f32;
			let rand_x = rand_dir();
			let rand_y = rand_dir();
			self.position.y = match self.position.y{
				y if y > 600. => y - 1.,
				y if y < 0. => y + 1.,
				_ => self.position.y + rand_y
			};
			self.position.x = match self.position.x{
				x if x > 600. => x - 1.,
				x if x < 0. => self.position.x + 1.,
				_ => self.position.x + rand_x
			};
		}
	}

	fn move_directly_to_creep(&mut self, closest_creep: &Creep){
		self.move_directly_to(&closest_creep.position);
	}

    fn move_directly_to(&mut self, position: &Position){
		let (x_diff, y_diff) = (self.position.x - position.x,
								self.position.y - position.y);

		self.position.x = match x_diff{
			x if x > 0. => self.position.x - 1.,
			x if x < 0. => self.position.x + 1.,
			_ => self.position.x // must be 0, in line with
		};

		self.position.y = match y_diff{
			y if y > 0. => self.position.y - 1.,
			y if y < 0. => self.position.y + 1.,
			_ => self.position.y // must be 0, in line with
		};
	}

	// Basically move to location but just their own fountain side so in defensive position
	fn move_defensively_to(&mut self, position: &Position){
		let our_fountain_pos = match self.side{
			Side::Radiant => RADIANT_FOUNT_POS,
			Side::Dire => DIRE_FOUNT_POS,
		};
		let (x_diff, y_diff) = ((position.x - our_fountain_pos.x), (position.y - our_fountain_pos.y));
		let mut offset_position = position.clone();
		offset_position.add_x(-(x_diff/x_diff.abs()) * 15. *x_diff.abs()/(x_diff.abs() + y_diff.abs()));
		offset_position.add_y(-(y_diff/y_diff.abs()) * 15. *y_diff.abs()/(y_diff.abs() + x_diff.abs()));
		if self.position.distance_between(offset_position) > 6.{
			self.move_directly_to(&offset_position);
		}
	}

	fn move_to_creep_along_lane(&mut self, creep: &Creep, correct_x: bool){
		let (xdiff, ydiff) = (self.position.x_distance(creep.position), self.position.y_distance(creep.position));
		if correct_x{
			match xdiff{
				x if x <= 10. => {
					self.move_directly_to_creep(creep)
				},
				_ => match ydiff{
					y if y<= 10. => {
						self.position.x -= creep.velocity.x;
						//self.position.y = (self.position.y as i32 + creep.velocity.y) as u32;
					},
					_ => {
					self.position.x += creep.velocity.y;
					self.position.y += creep.velocity.x;
				}
				},
			}
		}

		else{
			match self.position.y_distance(creep.position){
				y if y <= 10. => {
					self.move_directly_to_creep(creep);
				},
				_ => {
					self.position.x += creep.velocity.y;
					self.position.y += creep.velocity.x;
				},
			}
		}
	}

	// assume first creep in vector is in first wave so closest. yolo
	fn move_towards_creeps(&mut self, lane: Lane, lane_creeps: &Vec<Creep>){
		if self.can_action{
			let closest_creeps = lane_creeps.into_iter().filter(|&x| x.lane == lane).collect::<Vec<&Creep>>();  //change to retain
			if closest_creeps.len() > 0{
				let closest_creep = closest_creeps[0];
				let correct_corner = match lane{
					Lane::Bot => Some(BOT_LANE_VERTEX),
					Lane::Top => Some(TOP_LANE_VERTEX),
					_ => None
				};
				if !correct_corner.is_none() && self.position.distance_between(closest_creep.position) > 100.0 {
					let corner = correct_corner.unwrap();
					let (x_diff_corner, y_diff_corner) = (self.position.x_difference(corner),
					 self.position.y_difference(corner));
					match (x_diff_corner, y_diff_corner){
						(x, _) if x.abs() <= 10. => self.move_to_creep_along_lane(&closest_creep, true),
						(_, y) if y.abs() <= 10. => self.move_to_creep_along_lane(&closest_creep, false),
						(x, y) if x.abs() > y.abs() && y > 0. => self.position.add_y(-1.),
						(x, y) if x.abs() > y.abs() && y < 0. => self.position.add_y(1.),
						(x, y) if x.abs() < y.abs() && x > 0. => self.position.add_x(-1.),
						(x, y) if x.abs() < y.abs() && x < 0. => self.position.add_x(1.),
						_ => self.move_directly_to_creep(&closest_creep)
					}
				}
				else{
					self.move_directly_to_creep(&closest_creep);
				}
			}
		}
	}
}

pub trait Farm{
    fn farm_lane(&mut self, Lane, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>);
	fn farm_jungle(&mut self, &mut HashMap<&'static str, NeutralCamp>);
	fn farm_ancients(&mut self, &mut HashMap<&'static str, NeutralCamp>);
}

impl Farm for Hero{
	// can get exceptions if all a lane of lane creeps dead. fix pls
    fn farm_lane(&mut self, lane: Lane, our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>){
		let closest_enemy_creeps = &their_creeps.clone().into_iter().
			filter(|&x| x.lane == lane).collect::<Vec<Creep>>();
		let closest_friendly_creeps = &our_creeps.clone().into_iter().
			filter(|&x| x.lane == lane).collect::<Vec<Creep>>();
		let closest_towers = their_towers.clone().into_iter().filter(|&x| x.lane == lane).collect::<Vec<Tower>>();
		// assume that if no creeps on map for that lane, they must  be far away when they spawn
		//maybe bad behaviour
		let dist_enemy = match closest_enemy_creeps.len(){
			0 => 9001.,
			_ => self.position.distance_between(closest_enemy_creeps[0].position)
		};
		let dist_friendly = match closest_friendly_creeps.len(){
			0 => 9001.,
			_ => self.position.distance_between(closest_friendly_creeps[0].position)
		};
		let dist_tower = match closest_towers.len(){
			0 => 9001.,
			_ => self.position.distance_between(closest_towers[0].position)
		};
		match (dist_friendly, dist_enemy, dist_tower){
			(df, de, _) if df <= self.range && de <= self.range => self.attack_enemy_creeps(their_creeps),
			(df, _, _) if df > self.range => self.move_towards_creeps(lane, &our_creeps),
			(_, de, dt) if de > self.range && dt < de => self.move_to_attack_tower(their_towers, lane),
			_ => self.move_towards_creeps(lane, &our_creeps), //shouldnt be possible
		}
    }

	fn farm_jungle(&mut self, neutral_creeps: &mut HashMap<&'static str, NeutralCamp>){
		// currently if all creeps dead will just sit there afk
		let mut distance_from = 9000.;
		let mut closest_camp = None;
		for (camp_name, camp) in neutral_creeps{
			match (camp_name, self.position.distance_between(camp.position), camp.hp){
				(n, d, h) if n.to_string() != "ancient_camp" && d < distance_from && h > 0. => {closest_camp = Some(camp);
					distance_from = d
				},
				_ => {},
			}
		}
		if closest_camp.is_some(){
			if distance_from > self.range{
				self.move_directly_to(&closest_camp.unwrap().position);
			}
			else{
				self.attack_neutral(closest_camp.unwrap());
			}
		}
	}

	fn farm_ancients(&mut self, neutral_creeps: &mut HashMap<&'static str, NeutralCamp>){
		let ancient_camp = match neutral_creeps.entry("ancient_camp"){
			Vacant(_) => None,
			Occupied(entry) => Some(entry.into_mut()),
		};
		if ancient_camp.is_some(){
			let a = ancient_camp.unwrap();
			if self.position.distance_between(a.position) > self.range{
				self.move_directly_to(&a.position);
			}
			else{
				self.attack_neutral(a);
			}
		}
	}
}

pub trait DefendTower{
	fn move_to_defend_tower(&mut self, Lane, &Vec<Tower>);
}

impl DefendTower for Hero{
	fn move_to_defend_tower(&mut self, lane: Lane, our_towers: &Vec<Tower>){
		let closest_towers = our_towers.clone().into_iter().filter(|&x| x.lane == lane).collect::<Vec<Tower>>();
		if closest_towers.len() > 0{
			self.move_defensively_to(&closest_towers[0].position);
		}
		else{
			println!("no tower bro.should proablyb handle this");
		}
	}
}

pub trait Gank{
	fn gank_lane(&mut self, Lane, &mut Vec<Creep>, &mut [Hero; 5]);
}

impl Gank for Hero{
	fn gank_lane(&mut self, lane: Lane, their_creeps: &mut Vec<Creep>, enemy_heroes: &mut [Hero; 5]){
		let closest_enemy_creeps = &their_creeps.clone().into_iter().
			filter(|&x| x.lane == lane).collect::<Vec<Creep>>();
		if closest_enemy_creeps.len() > 0{
			let closest_enemy_creep = closest_enemy_creeps[0];
			let creep_diff = self.position.distance_between(closest_enemy_creep.position); // check creeps exist
			let mut hero_diff = 9001.;
			let mut closest_hero = None;
			for hero in enemy_heroes.iter_mut(){
				let distance = self.position.distance_between(hero.position);
				if distance < hero_diff{
					closest_hero = Some(hero);
					hero_diff = distance;
				}
			}
			match closest_hero{
				Some(hero) =>{
					match (creep_diff, hero_diff){
						(cd, _) if cd > 50. => self.move_directly_to(&closest_enemy_creep.position),
						(_, hd) if hd > self.range => self.move_directly_to(&hero.position),
						_ => self.attack_hero(hero)
					};
				}
				None => {}
			}
		}
	}
}

pub trait Follow{
	fn follow_hero(&mut self, &HeroInfo, enemy_heroes: &mut [Hero; 5]);
}

impl Follow for Hero{
	fn follow_hero(&mut self, friend: &HeroInfo, enemy_heroes: &mut [Hero; 5]){
		let mut lowest_distance: f32 = 9001.;
		for hero in enemy_heroes.iter_mut(){
			let distance_to = self.position.distance_between(hero.position);
			if distance_to < lowest_distance{
				match self.position.distance_between(hero.position){
					x if x < self.range => self.attack_hero(hero),
					x if x < 80. => self.move_directly_to(&hero.position), // maybe make this dynamic?
					_ => {}
				}
				lowest_distance = distance_to;
			}
		}
		if lowest_distance == 9001.{ // should probably replace memes with quality code at some point
			self.move_defensively_to(&friend.position);
		}
	}
}

pub trait Pull{
	fn pull_easy(&mut self, &mut HashMap<&'static str, NeutralCamp>, side: Side);
}

impl Pull for Hero{
	fn pull_easy(&mut self, camps: &mut HashMap<&'static str, NeutralCamp>, side: Side){
		let mut easy_camp = camps.get_mut("easy_camp").unwrap();
		if easy_camp.aggro_position.is_none(){
			if self.position.distance_between(easy_camp.position) > self.range{
				// so we need to not aggro the camp, until timing is right to pull to creeps
				self.move_directly_to(&easy_camp.position);
			}
			else{
				if easy_camp.position == easy_camp.home_position{
					let pull_position = match side{
						Side::Dire => Position{x: 176., y: 90.},
						Side::Radiant => Position{x: 451., y: 537.}
					};
					easy_camp.aggro_position = Some(pull_position);
				}
			}
		}
		else{
			match side{
				Side::Dire => self.position.y -= 1.,
				Side::Radiant => self.position.y += 1.,
			}
		};
	}
}
/*pub trait GetDecision{
	fn get_decision(self) -> &Decision;
}

impl GetDecision for Hero{
	fn get_decision(self) -> &Decision{
		&self.decisions[0]
	}
}*/
