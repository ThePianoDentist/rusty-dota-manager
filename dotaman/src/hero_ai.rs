use the_game::*;
use position::*;
use anhero::*;

#[derive(PartialEq)]
pub struct Decision{
    pub action: Action,
    pub probability: f32,
}

#[derive(PartialEq)]
pub enum Action{
    FARM_TOP_LANE,
    farm_bot_lane,
    farm_mid_lane,
    move_to_own_creeps,
    move_to_fountain,
    //attack_hero(Hero),
    //attack_tower(Tower),
    farm_friendly_jungle,
    farm_enemy_jungle,
    //follow_teammate(Hero),
    //run_away_from(Position),
    //go_to_rune(Side),
}

pub trait make_decision{
    fn make_decision(&mut self, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>);
}

impl make_decision for Hero{
    fn make_decision(&mut self, our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>){
        match self.decisions[0].action{
            Action::FARM_TOP_LANE => self.farm_top_lane(our_creeps, their_creeps, their_towers),
            _ => self.farm_top_lane(our_creeps, their_creeps, their_towers)
        };
    }
}
