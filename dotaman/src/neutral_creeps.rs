use position::*;
use the_game::*;

pub struct NeutralCamp{
    pub position: Position,
    pub hp: f32,
    pub max_hp: f32,
    pub max_gold: u32,
    pub stacked: u8,
    pub attack_damage: f32, // hmm this is currently damage per tick. is that fine?
    pub aggro_position: Option<Position>,
    pub home_position: Position,
    pub velocity: Velocity,
    pub move_speed: f32,
}

pub struct Neutrals{
    pub hard_camp_1: NeutralCamp,
    pub hard_camp_2: NeutralCamp,
    pub medium_camp_1: NeutralCamp,
    pub medium_camp_2: NeutralCamp,
    pub easy_camp_1: NeutralCamp,
    pub ancient_camp_2: NeutralCamp
}

impl_Travel!(NeutralCamp);

pub trait JustDoingWatNeutralsDo{
    fn respawn(&mut self);
    fn move_directly_to(&mut self, &Position, time_to_tick: &u64);
    fn chase_aggro(&mut self, time_to_tick: &u64);
}

impl JustDoingWatNeutralsDo for NeutralCamp{
    fn respawn(&mut self){
        if self.hp < 0.{
            self.position = self.home_position;
            self.aggro_position = None;
            self.hp = self.max_hp;
        };
    }

    fn move_directly_to(&mut self, position: &Position, time_to_tick: &u64){
		let (x_diff, y_diff) = (self.position.x - position.x,
								self.position.y - position.y);

		self.velocity.x = match x_diff{
			x if x > 0. =>  -1.,
			x if x < 0. => 1.,
			_ => 0. // must be 0, in line with
		};

		self.velocity.y = match y_diff{
			y if y > 0. => -1.,
			y if y < 0. => 1.,
			_ => 0. // must be 0, in line with
		};

        self.travel(time_to_tick);
	}

    fn chase_aggro(&mut self, time_to_tick: &u64){
        // because of floating point co-ords, calculations mean they dont exactly match
        if self.aggro_position.is_some() && self.position.distance_between(self.aggro_position.unwrap()) < 2.{
            self.aggro_position = None;
        }

        let aggro_pos = self.aggro_position;
        let creeps_house = self.home_position;
        if aggro_pos.is_some(){
            self.move_directly_to(&aggro_pos.unwrap(), time_to_tick);
        }
        else{
            self.move_directly_to(&creeps_house, time_to_tick);
        }
    }
}
