use position::*;

pub struct NeutralCamp{
    pub position: Position,
    pub hp: f32,
    pub max_hp: f32,
    pub max_gold: u32,
    pub stacked: u8,
}

pub struct Neutrals{
    pub hard_camp_1: NeutralCamp,
    pub hard_camp_2: NeutralCamp,
    pub medium_camp_1: NeutralCamp,
    pub medium_camp_2: NeutralCamp,
    pub easy_camp_1: NeutralCamp,
    pub ancient_camp_2: NeutralCamp
}

pub trait JustDoingWatNeutralsDo{
    fn respawn(&mut self);
}

impl JustDoingWatNeutralsDo for NeutralCamp{
    fn respawn(&mut self){
        if self.hp < 0.{
            self.hp = self.max_hp;
        };
    }
}
