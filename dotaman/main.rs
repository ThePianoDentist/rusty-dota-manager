extern crate piston_window;

use piston_window::*;

#[derive(Copy, Clone)]
enum Lane{
	Top,
	Mid,
	Bot
}

#[derive(Copy, Clone)]
enum Side{
	Radiant,
	Dire
}

#[derive(Copy, Clone)]
struct Creep {
	side: Side,
	lane: Lane,
	hitpoints: u32,
	attack_damage: u32,
	melee_attack: bool,
	x_coord: u32,
	y_coord: u32
}

struct Game {
		game_time: u64,
		lane_creeps: Vec<Creep>,
	}
	
trait TimeTick {
	fn new_game_time(&mut self);
}

impl TimeTick for Game{
	fn new_game_time(&mut self){
		self.game_time = self.game_time + 1
	}
}

fn move_top_creeps_radiant(lane_creep: &mut Creep){
	if lane_creep.y_coord > 2000{
		lane_creep.y_coord -= 1;
	}
	else{
		lane_creep.x_coord +=1
	}
}

fn move_mid_creeps_radiant(lane_creep: &mut Creep){
	lane_creep.y_coord -= 1;
	lane_creep.x_coord += 1
}

fn move_bot_creeps_radiant(lane_creep: &mut Creep){
	if lane_creep.x_coord < 14000{
		lane_creep.x_coord += 1;
	}
	else{
		lane_creep.y_coord -=1
	}
}

fn move_mid_creeps_dire(lane_creep: &mut Creep){
	lane_creep.y_coord += 1;
	lane_creep.x_coord -= 1;
}

fn move_bot_creeps_dire(lane_creep: &mut Creep){
	if lane_creep.y_coord < 14000{
		lane_creep.y_coord += 1;
	}
	else{
		lane_creep.x_coord -=1
	}
}

fn move_top_creeps_dire(lane_creep: &mut Creep){
	if lane_creep.x_coord > 2000{
		lane_creep.x_coord -= 1;
	}
	else{
		lane_creep.y_coord +=1
	}
}

trait MoveCreeps{
	fn move_creeps(&mut self);
}

impl MoveCreeps for Game{
	fn move_creeps(&mut self){
		for lane_creep in &mut self.lane_creeps{
			match lane_creep.side{
				Side::Radiant => match lane_creep.lane{
					Lane::Top => move_top_creeps_radiant(lane_creep),
					Lane::Mid => move_mid_creeps_radiant(lane_creep),
					_ => move_bot_creeps_radiant(lane_creep),},
				_ => match lane_creep.lane{
						Lane::Top => move_top_creeps_dire(lane_creep),
						Lane::Mid => move_mid_creeps_dire(lane_creep),
						_ => move_bot_creeps_dire(lane_creep),
					},
			};
		}
	}
}

fn main() {
    println!("Hello, world!");
    
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    };
	
	let mut game = Game{
		game_time: 0,
		lane_creeps: vec!(),
	};
	
	loop {
		game.game_time += 1;
		if game.game_time > 300{break;};
		if game.game_time % 30 == 0{
			for _ in 1..5{
				let new_radiant_top_creep = Creep{
					side: Side::Radiant,
					lane: Lane::Top,
					hitpoints: 150,
					attack_damage: 40,
					melee_attack: true,
					x_coord: 0,//4000,
					y_coord: 16000,//2000,
				};
				let new_radiant_bot_creep = Creep{lane: Lane::Bot, .. new_radiant_top_creep};
				let new_radiant_mid_creep = Creep{lane: Lane::Mid, .. new_radiant_top_creep};
				let new_dire_top_creep = Creep{side: Side::Dire, x_coord: 16000, y_coord: 0, .. new_radiant_top_creep};
				let new_dire_bot_creep = Creep{lane: Lane::Bot, .. new_dire_top_creep};
				let new_dire_mid_creep = Creep{lane: Lane::Mid, .. new_dire_top_creep};
				game.lane_creeps.push(new_radiant_top_creep);
				game.lane_creeps.push(new_radiant_bot_creep);
				game.lane_creeps.push(new_radiant_mid_creep);
				game.lane_creeps.push(new_dire_top_creep);
				game.lane_creeps.push(new_dire_mid_creep);		
				game.lane_creeps.push(new_dire_bot_creep);
			};
		}
		println!("game time {}", game.game_time);
		game.move_creeps();
		for (i, creep) in game.lane_creeps.iter().enumerate(){
			//println!("radiant_creeps {}", radiant_creep.side);
			println!("creeps {} {}", creep.x_coord, creep.y_coord);
		}
	}
}
