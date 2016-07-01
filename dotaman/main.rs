struct Creep {
	side: String,
	lane: String,
	hitpoints: u32,
	attack_damage: u32,
	melee_attack: bool,
	x_coord: u32,
	y_coord: u32
}

struct LaneCreeps{
	radiant_top_creeps: Vec<Creep>,
	radiant_mid_creeps: Vec<Creep>,
	radiant_bot_creeps: Vec<Creep>,
	dire_top_creeps: Vec<Creep>,
	dire_mid_creeps: Vec<Creep>,
	dire_bot_creeps: Vec<Creep>,
}

struct Game {
		game_time: u64,
		//radiant_towers array?,
		//radiant_top_creeps: Vec<Creep>,
		//radiant_mid_creeps: Vec<Creep>,
		//radiant_bot_creeps: Vec<Creep>,
		//dire_top_creeps: Vec<Creep>,
		//dire_mid_creeps: Vec<Creep>,
		//dire_bot_creeps: Vec<Creep>,
		lane_creeps: LaneCreeps,
	}
	
trait TimeTick {
	fn new_game_time(&mut self);
}

impl TimeTick for Game{
	fn new_game_time(&mut self){
		self.game_time = self.game_time + 1
	}
}

//fn move_creeps(game){
	//for lane_creep_type in game.lane_creeps{
		//match{
			
		//}
	//}
//}

fn main() {
    println!("Hello, world!");
    
    //let mut radiant_top_creeps = vec!();
	//let mut radiant_mid_creeps = vec!();
	//let mut radiant_bot_creeps = vec!();
	//let mut dire_top_creeps = vec!();
	//let mut dire_mid_creeps = vec!();
	//let mut dire_bot_creeps = vec!();
	
	let mut lane_creeps = LaneCreeps{
			radiant_top_creeps: vec!(),
			radiant_mid_creeps: vec!(),
			radiant_bot_creeps:vec!(),
			dire_top_creeps: vec!(),
			dire_mid_creeps: vec!(),
			dire_bot_creeps: vec!()
		};
	
	let mut game = Game{
		game_time: 0,
		lane_creeps: lane_creeps,
	};
	
	loop {
		game.game_time += 1;
		if game.game_time % 30 == 0{
			for _ in 1..5{
				let new_radiant_top_creep = Creep{
					side: "radiant".to_string(),
					lane: "top".to_string(),
					hitpoints: 150,
					attack_damage: 40,
					melee_attack: true,
					x_coord: 0,//4000,
					y_coord: 0,//2000,
				};
				game.lane_creeps.radiant_top_creeps.push(new_radiant_top_creep);
				let new_radiant_bot_creep = Creep{lane: "bot".to_string(), .. new_radiant_top_creep};
				game.lane_creeps.radiant_mid_creeps.push(new_radiant_bot_creep);
				let new_radiant_mid_creep = Creep{lane: "mid".to_string(), .. new_radiant_top_creep};
				game.lane_creeps.radiant_bot_creeps.push(new_radiant_mid_creep);
				let new_dire_top_creep = Creep{side: "dire".to_string(), .. new_radiant_top_creep};
				game.lane_creeps.dire_top_creeps.push(new_dire_top_creep);
				let new_dire_mid_creep = Creep{lane: "mid".to_string(), .. new_dire_top_creep};
				game.lane_creeps.dire_mid_creeps.push(new_dire_mid_creep);
				let new_dire_bot_creep = Creep{lane: "bot".to_string(), .. new_dire_top_creep};
				game.lane_creeps.dire_bot_creeps.push(new_dire_bot_creep);
			};
		}
		println!("game time {}", game.game_time);
		for (i, radiant_creep) in game.lane_creeps.radiant_mid_creeps.iter().enumerate(){
			//println!("radiant_creeps {}", radiant_creep.side);
			println!("radiant_creeps {}", i);
		}
	}
}
