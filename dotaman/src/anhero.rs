use the_game::*;
use position::*;
extern crate opengl_graphics;
extern crate rand;
use rand::Rng;
use hero_ai::*;
use std::collections::HashMap;
use neutral_creeps::*;

const MAGIC_NUMBER: f32 = MAX_COORD *(7.0/8.0);
static TOP_LANE_VERTEX: Position = Position{x: MAX_COORD - MAGIC_NUMBER, y: MAX_COORD - MAGIC_NUMBER};
static BOT_LANE_VERTEX: Position = Position{x: MAGIC_NUMBER, y: MAGIC_NUMBER};
static RADIANT_FOUNT_POS: Position = Position{x: MAX_COORD/20.0, y:MAX_COORD - (MAX_COORD/20.0)};
static DIRE_FOUNT_POS: Position = Position{x: MAX_COORD - MAX_COORD/20.0, y:MAX_COORD/20.0};

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
    pub decisions: Vec<Decision>,
	pub current_decision: Decision,
	pub side: Side,
	pub priority: u32,
}

// This is so that when we are looping through our own heroes, we are still allowed access to info on them
pub struct HeroInfo{
	pub position: Position,
	pub priority: u32,
	pub hp: f32,
	pub respawn_timer: i32,
	pub current_decision: Decision,
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

impl_AttackBuilding!(Hero);
impl_AttackClosestHero!(Hero);
//impl_AttackCreeps!(Hero); only heroes have gold
impl AttackCreeps for Hero{
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
		let (dist_enemy, dist_friendly, dist_tower) = (self.position.distance_between(closest_enemy_creeps[0].position),
			self.position.distance_between(closest_friendly_creeps[0].position),
			self.position.distance_between(closest_towers[0].position));
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

		let mut distance_from = 9001.;
		let mut ancient_camp = None;
		for (camp_name, camp) in neutral_creeps{
			match (camp_name, self.position.distance_between(camp.position)){
				(n, d) if n.to_string() == "ancient_camp" => {ancient_camp = Some(camp);
					distance_from = d
				},
				_ => {},
			}
		}
		// currently if all creeps dead will just sit there afk
		//let mut ancient_camp = neutral_creeps.get_mut("ancient_camp"); // do I need the if let some style if I know this there?
		if ancient_camp.is_some(){
			if distance_from > self.range{
				self.move_directly_to(&ancient_camp.unwrap().position);
			}
			else{
				self.attack_neutral(ancient_camp.unwrap());
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
		let closest_enemy_creep = &their_creeps.clone().into_iter().
			filter(|&x| x.lane == lane).collect::<Vec<Creep>>()[0];
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

pub trait Follow{
	fn follow_hero(&mut self, &HeroInfo);//, usize);
}

impl Follow for Hero{
	fn follow_hero(&mut self, friend: &HeroInfo){//, priority: usize){
		self.move_defensively_to(&friend.position);
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
