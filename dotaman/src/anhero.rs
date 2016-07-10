use the_game::*;
use position::*;
extern crate opengl_graphics;
extern crate rand;
use rand::Rng;
use hero_ai::*;

const MAGIC_NUMBER: f32 = MAX_COORD *(7.0/8.0);
static TOP_LANE_VERTEX: Position = Position{x: MAX_COORD - MAGIC_NUMBER, y: MAX_COORD - MAGIC_NUMBER};
static BOT_LANE_VERTEX: Position = Position{x: MAGIC_NUMBER, y: MAGIC_NUMBER};

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
	pub velocity: Velocity,
	pub respawn_timer: i32,
	pub range: f32,
    pub decisions: Vec<Decision>,
}

pub struct Skill{
	pub name: str,
}

impl_AttackBuilding!(Hero);
impl_AttackClosestHero!(Hero);
//impl_AttackCreeps!(Hero); only heroes have gold
impl AttackCreeps for Hero{
	fn attack_enemy_creeps(&mut self, enemy_creeps: &mut Vec<Creep>){
		for creep in &mut enemy_creeps.iter_mut(){
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
}

trait MoveToAttackTower{
	fn move_to_attack_tower(&mut self, &mut Vec<Tower>, Lane);
}

impl MoveToAttackTower for Hero{
	fn move_to_attack_tower(&mut self, towers: &mut Vec<Tower>, lane: Lane){
		// bugged until implement killing off of towers
		let ref closest_tower = towers.clone().into_iter().filter(|&x| x.lane == lane).collect::<Vec<Tower>>()[0];
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
    fn farm_top_lane(&mut self, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>);
}
impl Farm for Hero{
	// can get exceptions if all a lane of lane creeps dead. fix pls
    fn farm_top_lane(&mut self, our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>){
		let ref closest_enemy_creeps = their_creeps.clone().into_iter().
			filter(|&x| x.lane == Lane::Top).collect::<Vec<Creep>>();
		let ref closest_friendly_creeps = our_creeps.clone().into_iter().
			filter(|&x| x.lane == Lane::Top).collect::<Vec<Creep>>();
		let closest_towers = their_towers.clone().into_iter().filter(|&x| x.lane == Lane::Top).collect::<Vec<Tower>>();
		let (dist_enemy, dist_friendly, dist_tower) = (self.position.distance_between(closest_enemy_creeps[0].position),
			self.position.distance_between(closest_friendly_creeps[0].position),
			self.position.distance_between(closest_towers[0].position));
		match (dist_friendly, dist_enemy, dist_tower){
			(df, de, _) if df <= self.range && de <= self.range => self.attack_enemy_creeps(their_creeps),
			(df, _, _) if df > self.range => self.move_towards_creeps(Lane::Top, &our_creeps),
			(_, de, dt) if de > self.range && dt < de => self.move_to_attack_tower(their_towers, Lane::Top),
			_ => self.move_towards_creeps(Lane::Top, &our_creeps), //shouldnt be possible
		}
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
