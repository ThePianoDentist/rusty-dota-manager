use the_game::*;
use position::*;
use anhero::*;
extern crate rand;
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub struct Decision{
    pub action: Action,
    pub probability: f32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Action{
    FarmTopLane,
    FarmBotLane,
    FarmMidLane,
    DefendTopTower,
    DefendMidTower,
    DefendBotTower,
    MoveToFountain,
    GankTop,
    //attack_hero(Hero),
    //attack_tower(Tower),
    farm_friendly_jungle,
    farm_enemy_jungle,
    //follow_teammate(Hero),
    //run_away_from(Position),
    //go_to_rune(Side),
}

pub trait Decisions{
    fn process_decision(&mut self, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>, &Vec<Tower>, &mut [Hero; 5], Position);

    fn change_decision(&mut self);

    fn should_change_decision(&mut self) -> bool;
}

impl Decisions for Hero{
    fn process_decision(&mut self, our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>,
    our_towers: &Vec<Tower>, their_heroes: &mut [Hero; 5], fountain_position: Position){
        match self.decisions[0].action{
            Action::FarmTopLane => self.farm_lane(Lane::Top, our_creeps, their_creeps, their_towers),
            Action::FarmMidLane => self.farm_lane(Lane::Mid, our_creeps, their_creeps, their_towers),
            Action::FarmBotLane => self.farm_lane(Lane::Bot, our_creeps, their_creeps, their_towers),
            Action::MoveToFountain => self.move_directly_to(&fountain_position),
            Action::DefendTopTower => self.move_to_defend_tower(Lane::Top, our_towers),
            Action::DefendMidTower => self.move_to_defend_tower(Lane::Mid, our_towers),
            Action::DefendBotTower => self.move_to_defend_tower(Lane::Bot, our_towers),
            Action::GankTop => self.gank_lane(Lane::Top, their_creeps, their_heroes),
            _ => self.farm_lane(Lane::Top, our_creeps, their_creeps, their_towers)
        };
    }

    fn change_decision(&mut self){
        let rand_num = rand::thread_rng().gen_range(0, 101) as f32;
        let mut prob_counter = 0.0;
        for decision in &self.decisions{
            prob_counter += decision.probability;
            match prob_counter{
                prob if rand_num > prob => {},
                _ => self.current_decision = *decision,
            }
        }
    }

    fn should_change_decision(&mut self) -> bool{
        false
    }
}
