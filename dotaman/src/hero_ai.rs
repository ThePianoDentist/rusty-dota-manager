use the_game::*;
use position::*;
use anhero::*;
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use neutral_creeps::*;

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
    GankMid,
    GankBot,
    FollowHeroOne,
    FollowHeroTwo,
    FollowHeroThree,
    FollowHeroFour,
    FollowHeroFive,
    StackAncients,
    StackJungle,
    FarmFriendlyJungle,
    FarmEnemyJungle,
    FarmFriendlyAncients,
    FarmEnemyAncients,
    //follow_teammate(Hero),
    //run_away_from(Position),
    //go_to_rune(Side),
}

pub trait Decisions{
    fn process_decision(&mut self, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>, &Vec<Tower>, &mut [Hero; 5],
        &Vec<HeroInfo>, &mut HashMap<&'static str, NeutralCamp>, &mut HashMap<&'static str, NeutralCamp>,Position);

    fn change_decision(&mut self);

    fn should_change_decision(&mut self) -> bool;
}

impl Decisions for Hero{
    fn process_decision(&mut self, our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>,
    our_towers: &Vec<Tower>, their_heroes: &mut [Hero; 5], our_friends: &Vec<HeroInfo>,
     our_neutrals: &mut HashMap<&'static str, NeutralCamp>, their_neutrals: &mut HashMap<&'static str, NeutralCamp>, fountain_position: Position){
        let friend_one = our_friends.into_iter().filter(|&x| x.priority == 1).collect::<Vec<&HeroInfo>>()[0];
        let friend_two = our_friends.into_iter().filter(|&x| x.priority == 2).collect::<Vec<&HeroInfo>>()[0];
        let friend_three = our_friends.into_iter().filter(|&x| x.priority == 3).collect::<Vec<&HeroInfo>>()[0];
        let friend_four = our_friends.into_iter().filter(|&x| x.priority == 4).collect::<Vec<&HeroInfo>>()[0];
        let friend_five = our_friends.into_iter().filter(|&x| x.priority == 5).collect::<Vec<&HeroInfo>>()[0];
        match self.decisions[0].action{
            Action::FarmTopLane => self.farm_lane(Lane::Top, our_creeps, their_creeps, their_towers),
            Action::FarmMidLane => self.farm_lane(Lane::Mid, our_creeps, their_creeps, their_towers),
            Action::FarmBotLane => self.farm_lane(Lane::Bot, our_creeps, their_creeps, their_towers),
            Action::MoveToFountain => self.move_directly_to(&fountain_position),
            Action::DefendTopTower => self.move_to_defend_tower(Lane::Top, our_towers),
            Action::DefendMidTower => self.move_to_defend_tower(Lane::Mid, our_towers),
            Action::DefendBotTower => self.move_to_defend_tower(Lane::Bot, our_towers),
            Action::GankTop => self.gank_lane(Lane::Top, their_creeps, their_heroes),
            Action::GankMid => self.gank_lane(Lane::Mid, their_creeps, their_heroes),
            Action::GankBot => self.gank_lane(Lane::Bot, their_creeps, their_heroes),
            Action::FollowHeroOne => self.follow_hero(friend_one),
            Action::FollowHeroTwo => self.follow_hero(friend_two),
            Action::FollowHeroThree => self.follow_hero(friend_three),
            Action::FollowHeroFour => self.follow_hero(friend_four),
            Action::FollowHeroFive => self.follow_hero(friend_five),
            Action::FarmFriendlyJungle => self.farm_jungle(our_neutrals),
            Action::FarmEnemyJungle => self.farm_jungle(their_neutrals),
            Action::FarmFriendlyAncients => self.farm_ancients(our_neutrals),
            Action::FarmEnemyAncients => self.farm_ancients(their_neutrals),
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
