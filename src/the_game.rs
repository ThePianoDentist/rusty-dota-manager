extern crate rand;
extern crate opengl_graphics;
use std::collections::HashMap;
use position::*;
use anhero::*;
use neutral_creeps::*;
use hero_ai::*;

pub const MAX_COORD: f32  = 600.0;
pub const TIME_TO_TICK: u64 = 40;
pub const TOWER_RANGE: f32 = 30.; // heroes also need to know tower ranges for decision making

#[derive(Copy, Clone, PartialEq)]
pub enum Lane{
	Top,
	Mid,
	Bot
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side{
	Radiant,
	Dire
}

pub struct CreepClashPositions{
	pub top_clash: Position,
	pub mid_clash: Position,
	pub bot_clash: Position
}

pub struct Game {
	pub game_tick: u64,
	pub game_time: u64,
	pub teams: [Team; 2],
	pub xp_range: f32,
	pub commentary_string: String,
	pub time_to_tick: u64,
	pub creep_clash_positions: CreepClashPositions
}

pub struct Team{
	pub side: Side,
	pub towers: Vec<Tower>,
	pub fountain: Fountain,
	pub throne: Throne,
	pub lane_creeps: Vec<Creep>,
	pub heroes: [Hero; 5],
	pub neutrals: HashMap<&'static str, NeutralCamp>,
	pub current_decision: TeamAction,
	pub decisions: HashMap<TeamAction, f32>,
	pub should_change_decision: bool,
}

#[derive(Copy, Clone)]
pub struct Creep {
	pub lane: Lane,
	pub hp: f32,
	pub attack_damage: f32,
	pub melee_attack: bool,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub position: Position,
	pub velocity: Velocity,
	pub move_speed: f32,
	pub can_action: bool,
	pub range: f32,
	pub armour: f32,

	pub gold: f32, //un-used. but it means can use same function for heroes and creeps
}

// Buildings
#[derive(Copy, Clone)]
pub struct Tower{
	pub lane: Lane,
	pub tier: u8,
	pub max_hp: f32,
	pub hp: f32,
	pub attack_damage: f32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub can_action: bool,
	pub position: Position,
	pub range: f32,
	pub armour: f32,

	pub gold: f32, //un-used. but it means can use same function for heroes and creeps and towers
}

pub struct Throne{
	pub max_hp: f32,
	pub hp: f32,
	pub position: Position,
}

pub struct Fountain{
	pub attack_damage: f32,
	pub attack_cooldown: f32,
	pub attack_rate: f32,
	pub can_action: bool,
	pub position: Position,
	pub range: f32,
	pub hp: f32, // temp hack to keep consistent with tower code

	pub gold: f32, //un-used. but it means can use same function for heroes and creeps and towers
}

pub trait TimeTick {
	fn new_game_tick(&mut self);
}

impl TimeTick for Game{
	fn new_game_tick(&mut self){
		self.game_tick += 1
	}
}

pub trait ResetStuff{
	fn reset_all_attack_cooldown(&mut self);
	fn respawn_neutrals(&mut self);
	fn fountain_heal(&mut self);
}

impl ResetStuff for Game{
	fn reset_all_attack_cooldown(&mut self){
		for i in 0..2{
			for creep in &mut self.teams[i].lane_creeps{creep.can_action = true;};
			for tower in &mut self.teams[i].towers{tower.can_action = true;};
			for hero in &mut self.teams[i].heroes{
				hero.attacked_by_tower = false;
				hero.respawn_timer -= 1;
				if hero.respawn_timer <=0 {hero.can_action = true};
				};
			self.teams[i].fountain.can_action = true;
		}
	}

	fn respawn_neutrals(&mut self){
		for i in 0..2{
			for (camp_name, camp) in &mut self.teams[i].neutrals{
				camp.respawn(); // checks if camp neg hp. is so respawn with maxhp
			}
		}
	}

	fn fountain_heal(&mut self){
		for i in 0..2{
			let team = &mut self.teams[i];
			for hero in &mut team.heroes{
				if hero.position == team.fountain.position{
					hero.hp += 50.;
					if hero.hp > hero.max_hp{hero.hp = hero.max_hp}
				}
			}
		}
	}
}

pub trait AttackCreeps{
	fn attack_enemy_creeps(&mut self, &mut Vec<Creep>, &mut CreepClashPositions);
	fn attack_neutral(&mut self, neutral: &mut NeutralCamp);
	fn attack_all_neutrals(&mut self, neutrals: &mut HashMap<&'static str, NeutralCamp>);
}

macro_rules! impl_AttackCreeps {
    ($T:ident) => {
		impl AttackCreeps for $T{
			fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>, creep_clash_positions: &mut CreepClashPositions){
				for creep in enemy_creeps.iter_mut(){
					if self.position.distance_between(creep.position) < self.range{
						// hmm i think this means need to split macro for creeps
						match creep.lane{
							Lane::Top => {creep_clash_positions.top_clash = creep.position},
							Lane::Mid => {creep_clash_positions.mid_clash = creep.position},
							Lane::Bot => {creep_clash_positions.bot_clash = creep.position},
						};
						self.attack_cooldown -= 1.;
						self.can_action = false;
						if self.attack_cooldown <= 0.0{
							creep.hp -= self.attack_damage;
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
				if self.attack_cooldown <= 0.0{
					neutral.hp -= self.attack_damage;
					if neutral.hp <= 0.{
						self.gold += 40.
					}
					self.attack_cooldown += self.attack_rate;
				}
			}

			fn attack_all_neutrals(&mut self, neutrals: &mut HashMap<&'static str, NeutralCamp>){
				for (camp_name, mut camp) in neutrals{
					if self.position.distance_between(camp.position) <= self.range{
						self.attack_neutral(&mut camp);
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
				for hero in enemy_heroes.iter_mut(){
					if self.position.distance_between(hero.position) < self.range && self.can_action{
						self.can_action = false;
						self.attack_cooldown -= 1.;
						if self.attack_cooldown <= 0.0{
							hero.hp -= self.attack_damage;
							self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
						}
						break;
					}
				}
			}

			fn attack_hero(&mut self, hero: &mut Hero){
				let distance_to_hero = self.position.distance_between(hero.position);
				if self.position.distance_between(hero.position) < self.range && self.can_action{
					self.can_action = false;
					self.attack_cooldown -= 1.;
					if self.attack_cooldown <= 0.0{
						hero.hp -= self.attack_damage;
						self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
					}
				}
			}
		}
	}
}

impl AttackClosestHero for Tower{
	fn attack_closest_hero(&mut self, enemy_heroes: &mut [Hero]){
		for hero in enemy_heroes.iter_mut(){
			if self.position.distance_between(hero.position) < self.range && self.can_action{
				self.can_action = false;
				self.attack_cooldown -= 1.;
				if self.attack_cooldown <= 0.0{
					hero.hp -= self.attack_damage;
					hero.attacked_by_tower = true;
					self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
				}
				break;
			}
		}
	}

	fn attack_hero(&mut self, hero: &mut Hero){
		let distance_to_hero = self.position.distance_between(hero.position);
		if self.position.distance_between(hero.position) < self.range && self.can_action{
			self.can_action = false;
			self.attack_cooldown -= 1.;
			if self.attack_cooldown <= 0.0{
				hero.hp -= self.attack_damage;
				hero.attacked_by_tower = true;
				self.attack_cooldown += self.attack_rate; //bug with attacking too fast, it sorts of cycles all the way through once without realising...do two attacks in one tick??
			}
		}
	}
}

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
				for tower in enemy_towers.iter_mut(){
					if self.position.distance_between(tower.position) < self.range{
						self.can_action = false;
						self.attack_cooldown -= 1.;
						if self.attack_cooldown <= 0.0{
							tower.hp -= self.attack_damage;
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
					if self.attack_cooldown <= 0.0{
						throne.hp -= self.attack_damage;
						self.attack_cooldown += self.attack_rate;
					}
				};
			}
        }
    }
}

impl_AttackBuilding!(Creep);

pub trait Travel{
	fn travel(&mut self, time_to_tick: &u64);
}

macro_rules! impl_Travel{
	($T:ident) =>
	{
		impl Travel for $T{  // do handling to keep in bounds here
			fn travel(&mut self, time_to_tick: &u64){
				self.position.y += self.move_speed * (6. / (150. * *time_to_tick as f32)) * self.velocity.y;
				self.position.x += self.move_speed * (6. / (150. * *time_to_tick as f32)) * self.velocity.x;
			}
		}
	}
}

impl_Travel!(Hero);
impl_Travel!(Creep);

pub trait MoveDownLane{
	fn move_top_creep_radiant(&mut self, time_to_tick: &u64);
	fn move_mid_creep_radiant(&mut self, time_to_tick: &u64);
	fn move_bot_creep_radiant(&mut self, &NeutralCamp, &Vec<Creep>, ime_to_tick: &u64);
	fn move_top_creep_dire(&mut self, easy_camp: &NeutralCamp, time_to_tick: &u64);
	fn move_mid_creep_dire(&mut self, time_to_tick: &u64);
	fn move_bot_creep_dire(&mut self, time_to_tick: &u64);
}

impl MoveDownLane for Creep{
	fn move_top_creep_radiant(&mut self, time_to_tick: &u64){
		self.travel(time_to_tick);
		if self.position.small_random_pos_offset().y < (MAX_COORD / 8.){
			self.velocity = Velocity{x: 1., y: 0.};
		}
	}

	fn move_mid_creep_radiant(&mut self, time_to_tick: &u64){
		self.travel(time_to_tick);
	}

	fn move_bot_creep_radiant(&mut self, easy_camp: &NeutralCamp, enemy_creeps: &Vec<Creep>, time_to_tick: &u64){
		self.travel(time_to_tick);

		// check if should follow pull
		let ez_dist = self.position.distance_between(easy_camp.position);
		// so im computing this even when not doing, maybe move into the match?
		let to_camp_velocity = self.position.velocity_to(easy_camp.position);
		let closest_creeps = enemy_creeps.clone().into_iter().
			filter(|&x| x.lane == self.lane).collect::<Vec<Creep>>();
		let mut closest_creep_pos = None;
		if closest_creeps.len() > 0{  // must be a nicer way
			closest_creep_pos = Some(closest_creeps[0].position);

		}
		self.velocity = match (ez_dist, self.position.small_random_pos_offset().x, closest_creep_pos){
			(_, _, c) if self.position.distance_between(closest_creep_pos.unwrap()) < 50. => {self.position.velocity_to(closest_creep_pos.unwrap())}
			(a, _, _) if a < 30. => to_camp_velocity,
			(_, b, _) if b > MAX_COORD *(8.0/9.0) => Velocity{x: 0.0, y: -1.0},
			_ => Velocity{x: 1., y: 0.}
		};
	}

	fn move_mid_creep_dire(&mut self, time_to_tick: &u64){
		self.travel(time_to_tick);
	}

	fn move_bot_creep_dire(&mut self, time_to_tick: &u64){
		self.travel(time_to_tick);
		if self.position.y > MAX_COORD *(7.0/8.0){
			self.velocity = Velocity{x: -1.0, y: 0.0};
		}
	}

	fn move_top_creep_dire(&mut self, easy_camp: &NeutralCamp, time_to_tick: &u64){
		self.travel(time_to_tick);

		// check if should follow pull
		let ez_dist = self.position.distance_between(easy_camp.position);
		let to_camp_velocity = self.position.velocity_to(easy_camp.position);
		self.velocity = match (ez_dist, self.position.small_random_pos_offset().x){
			(a, _) if a < 30. => to_camp_velocity,
			(_, b) if b < MAX_COORD / 8.0 => Velocity{x: 0.0, y: 1.0},
			_ => Velocity{x: -1.0, y: 0.}
		};
	}
}

pub trait MoveCreeps{
	fn move_creeps_radiant(&mut self, &Vec<Creep>, time_to_tick: &u64);
	fn move_creeps_dire(&mut self, time_to_tick: &u64);
}

impl MoveCreeps for Team{
	fn move_creeps_radiant(&mut self, enemy_creeps: &Vec<Creep>, time_to_tick: &u64){
		let easy_camp = self.neutrals.get("easy_camp").unwrap();
		for lane_creep in &mut self.lane_creeps{
			if lane_creep.can_action{
				match lane_creep.lane{
					Lane::Top => lane_creep.move_top_creep_radiant(time_to_tick),
					Lane::Mid => lane_creep.move_mid_creep_radiant(time_to_tick),
					Lane::Bot => lane_creep.move_bot_creep_radiant(easy_camp, enemy_creeps, time_to_tick),
				};
			}
		}
	}

	fn move_creeps_dire(&mut self, time_to_tick: &u64){
		let easy_camp = self.neutrals.get("easy_camp").unwrap();
		for lane_creep in &mut self.lane_creeps{
			if lane_creep.can_action{
				match lane_creep.lane{
					Lane::Top => lane_creep.move_top_creep_dire(easy_camp, time_to_tick),
					Lane::Mid => lane_creep.move_mid_creep_dire(time_to_tick),
					Lane::Bot => lane_creep.move_bot_creep_dire(time_to_tick),
				};
			}
		}
	}
}

pub trait KillOff{
	// this is kind of stupid because i have to loop in every function. should loop outside func maybe?
	fn kill_off_creeps(&mut self);
	fn kill_off_heroes(&mut self);
	fn kill_off_towers(&mut self);
}

impl KillOff for Game{
	fn kill_off_creeps(&mut self){
		for i in 0..2{
			let (rad, dire) = self.teams.split_at_mut(1);
			let (mut us, mut them) = match i{
				0 => (&mut rad[0], &mut dire[0]),
				_ => (&mut dire[0], &mut rad[0])
			};
			for creep in &mut us.lane_creeps{
				if creep.hp <= 0.{
					let mut heroes_in_xp_range = 0;
					for hero in &them.heroes{
						if hero.position.distance_between(creep.position) <= self.xp_range{
							heroes_in_xp_range += 1;
						}
					};
					for hero in &mut them.heroes{
						if hero.position.distance_between(creep.position) <= self.xp_range{
							hero.xp += 45. / heroes_in_xp_range as f32;
						}
					}
					// I dont think its safe/wise to do the delete in the loop. so just loop again down below?
				}
			}
			us.lane_creeps.retain(|&i| i.hp >= 0.);
		};
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

	fn kill_off_towers(&mut self){
		for i in 0..2{
			let (rad, dire) = self.teams.split_at_mut(1);
			let (mut us, mut them) = match i{
				0 => (&mut rad[0], &mut dire[0]),
				_ => (&mut dire[0], &mut rad[0])
			};
			for tower in &mut us.towers{
				if tower.hp <= 0.{
					for hero in &mut them.heroes{
						hero.gold += 200.// will also need gold for last hit on tower
					}
					// if tower died change team decisions
					them.update_decision_prob(TeamAction::GreedyFarmAllLanesSupportsDefensive, 1.);
					them.should_change_decision = true;
				}
			}
			us.towers.retain(|&i| i.hp >= 0.);
		}
	}
}

pub trait GetHeroInfo{
	fn get_other_hero_info(&mut self) -> Vec<HeroInfo>;
}

impl GetHeroInfo for Team{
	fn get_other_hero_info(&mut self) -> Vec<HeroInfo>{
		let mut other_heroes = vec!();
		for hero in &self.heroes{
			other_heroes.push(hero.hero_to_hero_info())
		}
		other_heroes
	}
}

pub trait SafeOffLane{
	fn safelane(&mut self) -> Lane;
	fn offlane(&mut self) -> Lane;
	fn actionfarm_safelane_offlane(&mut self) -> (Action, Action);
	fn farmsafe_xpoff(&mut self) -> (Action, Action);
}

impl SafeOffLane for Team{
	fn safelane(&mut self) -> Lane{
		match self.side{
			Side::Radiant => Lane::Bot,
			Side::Dire => Lane::Top
		}
	}

	fn offlane(&mut self) -> Lane{
		match self.side{
			Side::Radiant => Lane::Top,
			Side::Dire => Lane::Bot
		}
	}

	fn actionfarm_safelane_offlane(&mut self) -> (Action, Action){
		match self.side{
			Side::Radiant => (Action::FarmBotLane, Action::FarmTopLane),
			Side::Dire => (Action::FarmTopLane, Action::FarmBotLane)
		}
	}
	fn farmsafe_xpoff(&mut self) -> (Action, Action){
		match self.side{
			Side::Radiant => (Action::FarmBotLane, Action::GetXpTop),
			Side::Dire => (Action::FarmTopLane, Action::GetXpBot)
		}
	}
}
