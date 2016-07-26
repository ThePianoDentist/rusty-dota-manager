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

#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
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
    GetRuneTop,
    GetRuneBot,
    PullEasy,
    GetXpTop,
    GetXpMid,
    GetXpBot,
    ZoneBot,
    ZoneTop,
    ZoneMid
}

#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum TeamAction{
    GreedyFarmAllLanesSupportsDefensive,
    GreedyFarmAllLanesSupportsGanking,
    DefendTopTowerFive,
    DefendMidTowerFive,
    DefendBotTowerFive,
    DefendTowerFour,
    FiveManTop,
    FiveManBot,
    FiveManMid,
    FourManAttackTower,
    GankEnemyJungle,
    StandardLaning,
    AggroLaning,
    DualLaningOff,
    DualLaningMid,
    Roshing,
    IndividualChoice,
}

pub trait TeamDecisions{
    fn process_team_decision(&mut self);
    fn should_change_team_decision(&mut self, game_time: u64, game_tick: u64) -> bool;
    fn change_team_decision(&mut self);
    fn greedy_farming(&mut self);
    fn greedy_farm_sup_gank(&mut self);
    fn five_man_lane(&mut self, Lane);
    fn five_man_def(&mut self, Lane);
    fn update_decision_prob(&mut self, update_action: TeamAction, new_prob: f32);
    fn update_multi_decisions_prob(&mut self, updates: Vec<(TeamAction, f32)>);
    fn standard_lanes(&mut self);
    fn dual_off(&mut self);
}

impl TeamDecisions for Team{
    fn process_team_decision(&mut self){
        match self.current_decision{
            TeamAction::FiveManTop => self.five_man_lane(Lane::Top),
            TeamAction::FiveManBot => self.five_man_lane(Lane::Bot),
            TeamAction::FiveManMid => self.five_man_lane(Lane::Mid),
            TeamAction::GreedyFarmAllLanesSupportsDefensive => self.greedy_farming(),
            TeamAction::GreedyFarmAllLanesSupportsGanking => self.greedy_farm_sup_gank(),
            TeamAction::DefendTopTowerFive => self.five_man_def(Lane::Top),
            TeamAction::DefendMidTowerFive => self.five_man_def(Lane::Mid),
            TeamAction::DefendBotTowerFive => self.five_man_def(Lane::Bot),
            TeamAction::StandardLaning => self.standard_lanes(),
            /*
            TeamAction::DefendTowerFour,
            TeamAction::FourManAttackTower,
            TeamAction::GankEnemyJungle,
            TeamAction::StandardLaning,
            TeamAction::AggroLaning,
            TeamAction::DualLaningOff,
            TeamAction::DualLaningMid,
            TeamAction::Roshing,
            TeamAction::IndividualChoice,*/
            _ => self.five_man_lane(Lane::Top)
        }
    }

    fn should_change_team_decision(&mut self, game_time: u64, game_tick: u64) -> bool{
        let mut change = self.should_change_decision;
        if !change{
            if game_time == 400{
                if self.side == Side::Radiant{
                self.update_decision_prob(TeamAction::FiveManTop, 1.0);
                }
                else{
                    self.update_decision_prob(TeamAction::FiveManMid, 1.0);
                }
                change = true;
            }
        }
        let our_friends = self.get_other_hero_info();
        let fountain_pos = self.fountain.position;
        for hero in &mut self.heroes{
            // individual hero level decision changes
            let hero_decision = hero.should_change_decision(fountain_pos, game_tick, &our_friends);
            if hero_decision{
                hero.change_decision();
                //change = true;
            }
        }
        change
    }

    fn change_team_decision(&mut self){
        println!("Chaning decision");
    }

    fn update_decision_prob(&mut self, update_action: TeamAction, new_prob: f32){
        let old_prob = self.decisions.get(&update_action).unwrap().clone(); // is this the right place to use clone?
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                a if a == update_action => new_prob,
                _ => *prob * (1. - (new_prob + old_prob))  // scale all the other probabilites
            };
        }
    }

    fn update_multi_decisions_prob(&mut self, updates: Vec<(TeamAction, f32)>){
        let mut total_new_prob = 0.;
        let mut total_old_prob = 0.;
        for (update_action, new_prob) in updates.clone(){
            total_new_prob += new_prob;
            for (action, old_prob) in self.decisions.iter(){
                match *action{
                    a if a == update_action => total_old_prob += *old_prob,
                    _ => {}
                }
            }
        }

        // is there another expression for (1 - sum_new/1 - sum_old)?

        for (update_action, new_prob) in updates{
            let old_prob = self.decisions.get(&update_action).unwrap().clone(); // is this the right place to use clone?
            for (action, prob) in self.decisions.iter_mut(){
                *prob = match *action{
                    a if a == update_action => new_prob,
                    _ => *prob * (1. - total_new_prob)/(1. - total_old_prob)  // scale all the other probabilites
                };
            }
        }
    }

    fn greedy_farming(&mut self){
        let (farm_safelane, farm_offlane) = self.actionfarm_safelane_offlane();

        for hero in self.heroes.iter_mut(){
            match hero.priority{
                4 | 5 => {hero.update_decision_prob(Action::FollowHeroOne, 0.5); // hmmm this doesnt make 5050 chance. as existing probs are just diluted
                          hero.update_decision_prob(Action::FollowHeroTwo, 0.5) // need method for updating multiple probs at once
                         },
                3 => hero.update_decision_prob(farm_offlane, 1.),
                2 => hero.update_decision_prob(Action::FarmMidLane, 1.),
                _ => hero.update_decision_prob(farm_safelane, 1.),
            }
        }
    }

    fn greedy_farm_sup_gank(&mut self){
        let (farm_safelane, farm_offlane) = self.actionfarm_safelane_offlane();
        for hero in self.heroes.iter_mut(){
            match hero.priority{
                4 | 5 => {hero.update_decision_prob(Action::GankMid, 0.5); // hmmm this doesnt make 5050 chance. as existing probs are just diluted
                          hero.update_decision_prob(Action::GankBot, 0.5) // need method for updating multiple probs at once
                         },
                3 => hero.update_decision_prob(farm_offlane, 1.),
                2 => hero.update_decision_prob(Action::FarmMidLane, 1.),
                _ => hero.update_decision_prob(farm_safelane, 1.),
            }
        }

    }

    fn five_man_lane(&mut self, lane: Lane){
        let action = match lane{
            Lane::Top => Action::FarmTopLane,
            Lane::Mid => Action::FarmMidLane,
            Lane::Bot => Action::FarmBotLane,
        };
        for hero in self.heroes.iter_mut(){
            match hero.priority{
                4 | 5 => hero.update_decision_prob(action, 1.),
                3 => hero.update_decision_prob(action, 1.),
                2 => hero.update_decision_prob(action, 1.),
                _ => hero.update_decision_prob(action, 1.),
            }
        }
    }

    fn five_man_def(&mut self, lane: Lane){
        let action = match lane{
            Lane::Top => Action::DefendTopTower,
            Lane::Mid => Action::DefendMidTower,
            Lane::Bot => Action::DefendBotTower,
        };
        for hero in self.heroes.iter_mut(){
            hero.update_decision_prob(action, 1.);
        }
    }

    fn standard_lanes(&mut self){
        let (safelane, offlane) = (self.safelane(), self.offlane());
        let zone_action = match safelane{
            Lane::Bot => Action::ZoneBot,
            _ => Action::ZoneTop,
        };
        let (action_safelane, action_offlane) = self.farmsafe_xpoff();
        for hero in self.heroes.iter_mut(){
            match hero.priority{
                5 => hero.update_decision_prob(Action::FollowHeroOne, 1.),
                4 => hero.update_multi_decision_prob(vec!((Action::FollowHeroOne, 0.33), (Action::PullEasy, 0.33), (zone_action, 0.33))),
                3 => hero.update_decision_prob(action_offlane, 1.),
                2 => hero.update_decision_prob(Action::FarmMidLane, 1.),
                _ => hero.update_decision_prob(action_safelane, 1.),
            }
        }
    }

    fn dual_off(&mut self){
        let (action_safelane, action_offlane) = self.farmsafe_xpoff();
        for hero in self.heroes.iter_mut(){
            match hero.priority{
                5 => hero.update_decision_prob(Action::FollowHeroOne, 1.),
                4 => hero.update_decision_prob(Action::FollowHeroTwo, 1.),
                3 => hero.update_decision_prob(action_offlane, 1.),
                2 => hero.update_decision_prob(Action::FarmMidLane, 1.),
                _ => hero.update_decision_prob(action_safelane, 1.),
            }
        }
    }
}

pub trait ChangeDecision{
    fn change_decision(&mut self);
}

impl ChangeDecision for Hero{
    fn change_decision(&mut self){
        self.should_change_decision = false;
        let rand_num = rand::thread_rng().gen_range(1, 101) as f32 / 100.;
        let mut prob_counter = 0.0;
        for (action, prob) in &mut self.decisions.iter(){
            prob_counter += *prob;
            match prob_counter{
                p if rand_num > p => {},
                _ => {println!("{}: {:?}", self.name, action); self.current_decision = *action; break},
            }
        }
    }
}

impl ChangeDecision for Team{
    fn change_decision(&mut self){
        self.should_change_decision = false;
        let rand_num = rand::thread_rng().gen_range(1, 101) as f32 / 100.;
        let mut prob_counter = 0.0;
        for (action, prob) in &mut self.decisions.iter(){
            prob_counter += *prob;
            match prob_counter{
                p if rand_num > p => {},
                _ => {println!("{:?}: {:?}", self.side, action); self.current_decision = *action; break},
            }
        }
    }
}

pub trait Decisions{
    fn process_decision(&mut self, Side, &CreepClashPositions, &f32, &mut Vec<Creep>, &mut Vec<Creep>, &mut Vec<Tower>, &Vec<Tower>, &mut [Hero; 5],
        &Vec<HeroInfo>, &mut HashMap<&'static str, NeutralCamp>, &mut HashMap<&'static str, NeutralCamp>,Position,
    time_to_tick: &u64);

    fn should_change_decision(&mut self, Position, u64, our_friends: &Vec<HeroInfo>) -> bool;

    fn update_decision_prob(&mut self, Action, f32);
    fn update_multi_decision_prob(&mut self, Vec<(Action, f32)>);

    fn check_if_should_heal(&mut self) -> bool;

    fn check_if_healed_fountain(&mut self, Position, u64, our_friends: &Vec<HeroInfo>) -> bool;

    fn set_out_of_base_decisions(&mut self, our_friends: &Vec<HeroInfo>);
    fn set_out_of_base_support_decisions(&mut self, our_friends: &Vec<HeroInfo>);
    fn set_out_of_base_mid_decisions(&mut self,our_friends: &Vec<HeroInfo>);
    fn set_out_of_base_carry_decisions(&mut self, our_friends: &Vec<HeroInfo>);
    fn set_out_of_base_offlane_decisions(&mut self, our_friends: &Vec<HeroInfo>);
}

impl Decisions for Hero{
    fn process_decision(&mut self, side: Side, creep_clash_pos: &CreepClashPositions, xp_range: &f32,
        our_creeps: &mut Vec<Creep>, their_creeps: &mut Vec<Creep>, their_towers: &mut Vec<Tower>,
    our_towers: &Vec<Tower>, their_heroes: &mut [Hero; 5], our_friends: &Vec<HeroInfo>,
     our_neutrals: &mut HashMap<&'static str, NeutralCamp>, their_neutrals: &mut HashMap<&'static str, NeutralCamp>,
      fountain_position: Position, time_to_tick: &u64){
        let friend_one = our_friends.into_iter().filter(|&x| x.priority == 1).collect::<Vec<&HeroInfo>>()[0];
        let friend_two = our_friends.into_iter().filter(|&x| x.priority == 2).collect::<Vec<&HeroInfo>>()[0];
        let friend_three = our_friends.into_iter().filter(|&x| x.priority == 3).collect::<Vec<&HeroInfo>>()[0];
        let friend_four = our_friends.into_iter().filter(|&x| x.priority == 4).collect::<Vec<&HeroInfo>>()[0];
        let friend_five = our_friends.into_iter().filter(|&x| x.priority == 5).collect::<Vec<&HeroInfo>>()[0];
        match self.current_decision{
            Action::FarmTopLane => self.farm_lane(Lane::Top, our_creeps, their_creeps, their_towers, time_to_tick),
            Action::FarmMidLane => self.farm_lane(Lane::Mid, our_creeps, their_creeps, their_towers, time_to_tick),
            Action::FarmBotLane => self.farm_lane(Lane::Bot, our_creeps, their_creeps, their_towers, time_to_tick),
            Action::MoveToFountain => self.move_directly_to(&fountain_position, time_to_tick),
            Action::DefendTopTower => self.move_to_defend_tower(Lane::Top, our_towers, time_to_tick),
            Action::DefendMidTower => self.move_to_defend_tower(Lane::Mid, our_towers, time_to_tick),
            Action::DefendBotTower => self.move_to_defend_tower(Lane::Bot, our_towers, time_to_tick),
            Action::GankTop => self.gank_lane(Lane::Top, their_creeps, their_heroes, their_towers, creep_clash_pos, time_to_tick),
            Action::GankMid => self.gank_lane(Lane::Mid, their_creeps, their_heroes, their_towers, creep_clash_pos, time_to_tick),
            Action::GankBot => self.gank_lane(Lane::Bot, their_creeps, their_heroes, their_towers, creep_clash_pos, time_to_tick),
            Action::FollowHeroOne => self.follow_hero(friend_one, their_heroes, time_to_tick),
            Action::FollowHeroTwo => self.follow_hero(friend_two, their_heroes, time_to_tick),
            Action::FollowHeroThree => self.follow_hero(friend_three, their_heroes, time_to_tick),
            Action::FollowHeroFour => self.follow_hero(friend_four, their_heroes, time_to_tick),
            Action::FollowHeroFive => self.follow_hero(friend_five, their_heroes, time_to_tick),
            Action::FarmFriendlyJungle => self.farm_jungle(our_neutrals, time_to_tick),
            Action::FarmEnemyJungle => self.farm_jungle(their_neutrals, time_to_tick),
            Action::FarmFriendlyAncients => self.farm_ancients(our_neutrals, time_to_tick),
            Action::FarmEnemyAncients => self.farm_ancients(their_neutrals, time_to_tick),
            Action::PullEasy => self.pull_easy(our_neutrals, side, time_to_tick),
            Action::GetXpTop => self.get_xp(Lane::Top, &their_creeps, xp_range, &fountain_position, &creep_clash_pos, time_to_tick),
            Action::GetXpMid => self.get_xp(Lane::Mid, &their_creeps, xp_range, &fountain_position, &creep_clash_pos, time_to_tick),
            Action::GetXpBot => self.get_xp(Lane::Bot, &their_creeps, xp_range, &fountain_position, &creep_clash_pos, time_to_tick),
            Action::ZoneTop => self.zone_out(Lane::Top, their_heroes, &our_creeps, xp_range, time_to_tick, &creep_clash_pos),
            Action::ZoneMid => self.zone_out(Lane::Mid, their_heroes, &our_creeps, xp_range, time_to_tick, &creep_clash_pos),
            Action::ZoneBot => self.zone_out(Lane::Bot, their_heroes, &our_creeps, xp_range, time_to_tick, &creep_clash_pos),
            _ => {println!("dude wgtf"); self.farm_lane(Lane::Top, our_creeps, their_creeps, their_towers, time_to_tick)}
        };
    }

    fn should_change_decision(&mut self, fountain_position: Position, game_tick: u64, our_friends: &Vec<HeroInfo>) -> bool{
        if !self.should_change_decision{
            self.should_change_decision = self.check_if_should_heal();
        }
        if !self.should_change_decision && self.position.distance_between(fountain_position) < 1.0{
            self.should_change_decision = self.check_if_healed_fountain(fountain_position, game_tick, our_friends);
        }
        self.should_change_decision
    }

    fn update_decision_prob(&mut self, update_action: Action, new_prob: f32){
        let old_prob = self.decisions.get(&update_action).unwrap().clone(); // is this the right place to use clone?
        let decision_count = self.decisions.len().clone();
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                a if a == update_action => new_prob,
                _ => *prob * (1. - (new_prob + old_prob))  // scale all the other probabilites
            };
        }
    }

    fn update_multi_decision_prob(&mut self, updates: Vec<(Action, f32)>){
        let mut total_new_prob = 0.;
        let mut total_old_prob = 0.;
        for (update_action, new_prob) in updates.clone(){
            total_new_prob += new_prob;
            for (action, old_prob) in self.decisions.iter(){
                match *action{
                    a if a == update_action => total_old_prob += *old_prob,
                    _ => {}
                }
            }
        }

        for (update_action, new_prob) in updates{
            let old_prob = self.decisions.get(&update_action).unwrap().clone(); // is this the right place to use clone?
            for (action, prob) in self.decisions.iter_mut(){
                *prob = match *action{
                    a if a == update_action => new_prob,
                    _ => *prob * (1. - total_new_prob)/(1. - total_old_prob)  // scale all the other probabilites
                };
            }
        }
    }

    fn check_if_should_heal(&mut self) -> bool{
        if self.hp < self.max_hp / 3. && self.current_decision != Action::MoveToFountain{
            self.update_decision_prob(Action::MoveToFountain, 1.0);
            true
        }
        else{false}
    }

    fn check_if_healed_fountain(&mut self, fountain_position: Position, game_tick: u64, our_friends: &Vec<HeroInfo>) -> bool {
        //will there be a bug if fountain diving
        if game_tick > 100 && self.position.distance_between(fountain_position) < 1.0
        && self.hp >= self.max_hp{
            //change team decision
            self.set_out_of_base_decisions(our_friends);
            true
        }
        else{
            false
        }
         // makes sure we dont switch decision right at start of game
    }

    fn set_out_of_base_decisions(&mut self, our_friends: &Vec<HeroInfo>){
        match self.priority{
            4 | 5 => self.set_out_of_base_support_decisions(our_friends),
            3 => self.set_out_of_base_offlane_decisions(our_friends),
            2 => self.set_out_of_base_mid_decisions(our_friends),
            _ => self.set_out_of_base_carry_decisions(our_friends),
        }
    }

    fn set_out_of_base_support_decisions(&mut self, our_friends: &Vec<HeroInfo>){
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                Action::FarmFriendlyJungle => 0.25,
                Action::FarmFriendlyAncients => 0.25,
                Action::GankMid => 0.25,
                Action::FollowHeroThree => 0.25,
                _ => 0.
            }
        }
    }

    fn set_out_of_base_mid_decisions(&mut self, our_friends: &Vec<HeroInfo>){
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                Action::GankTop => 0.25,
                Action::FarmFriendlyJungle => 0.25,
                Action::GankMid => 0.25,
                Action::GankBot => 0.25,
                _ => 0.
            }
        }

        let friends_farming_mid = our_friends.iter().find(|&x| x.current_decision == Action::FarmMidLane);
        if friends_farming_mid.is_none(){
            self.update_decision_prob(Action::FarmMidLane, 0.9);
        }
    }

    fn set_out_of_base_carry_decisions(&mut self, our_friends: &Vec<HeroInfo>){
        let safelane = Action::FarmBotLane;
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                Action::FarmFriendlyJungle => 0.4,
                Action::FarmMidLane => 0.4,
                Action::GankTop => 0.1,
                Action::FarmTopLane => 0.1,
                _ => 0.
            }
        }
        let friends_farming_safe = our_friends.iter().find(|&x| x.current_decision == safelane);
        if friends_farming_safe.is_none(){
            self.update_decision_prob(safelane, 0.9);
        }
    }
    fn set_out_of_base_offlane_decisions(&mut self, our_friends: &Vec<HeroInfo>){
        for (action, prob) in self.decisions.iter_mut(){
            *prob = match *action{
                Action::FarmTopLane => 0.8,
                Action::GankBot => 0.1,
                Action::GankMid => 0.1,
                _ => 0.
            }
        }
    }
}
