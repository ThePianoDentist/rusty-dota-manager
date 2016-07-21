pub struct Item{
    pub name: &'static str,
    pub cost: f32,
    pub str_gain: f32,
    pub int_gain: f32,
    pub agi_gain: f32,
    pub magic_resist_gain: f32,
    pub attack_rate_change: f32,
    pub damage_gain: f32,
    pub ms_gain: f32,
    pub active_ability: ItemAbility,
}

pub struct ItemAbility{
    pub name: &'static str,
}
