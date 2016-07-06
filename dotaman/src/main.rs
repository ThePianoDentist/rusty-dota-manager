extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;
extern crate gfx_device_gl;

use piston_window::*;
//use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, Texture as gTexture};
use rand::Rng;

const MAX_COORD: u32  = 600;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DARK_RED: [f32; 4] = [0.8, 0.1, 0.1, 1.0];
const DARK_GREEN: [f32; 4] = [0.2, 0.8, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    window: PistonWindow,
    background: &'a opengl_graphics::Texture,
}

impl<'a> App<'a> {
	fn render(&mut self, args: &RenderArgs) {
        let background = self.background;
		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREEN, gl);
			
			image(background, c.transform, gl);
		});
	}
    
    fn update_game(&mut self, game: &Game, args: &RenderArgs) {
		let background = self.background;
		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREEN, gl);
			
			image(background, c.transform, gl);
		});
		
		for tower in &game.towers{
			if tower.hp.is_positive(){
				self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(tower.position.x as f64, tower.position.y as f64);
											   
					let square = rectangle::square(0.0, 0.0, 10.0);
					
					if tower.side == Side::Dire{
						rectangle(DARK_RED, square, transform, gl);
					}
					else{
						rectangle(DARK_GREEN, square, transform, gl);
					}
					
					if tower.attacked{ellipse(YELLOW, rectangle::square(0.0, 0.0, 4.0),
						 c.transform.trans(tower.position.x as f64 + 3.0, tower.position.y as f64+ 3.0), gl);};
				});
			}
		};
		
        for creep in &game.lane_creeps{
			self.gl.draw(args.viewport(), |c, gl| {
				let transform = c.transform.trans(creep.position.x as f64, creep.position.y as f64);
										   
				let square = rectangle::square(0.0, 0.0, 5.0);

				if creep.side == Side::Dire{
					ellipse(RED, square, transform, gl);
				}
				else{
					ellipse(GREEN, square, transform, gl);
				}
			});
		}
    }
}

#[derive(Copy, Clone)]
struct Position{
	x: u32,
	y: u32
}

trait Distance{
	fn distance_between(&self, other_point: Position) -> f32;
	fn x_distance(&self, other_point: Position) -> u32;
	fn y_distance(&self, other_point: Position) -> u32;
}

impl Distance for Position{
	fn x_distance(&self, other_point: Position) -> u32{
		(self.x as i32 - other_point.x as i32).abs() as u32
	}
	
	fn y_distance(&self, other_point: Position) -> u32{
		(self.y as i32 - other_point.y as i32).abs() as u32
	}
	
	fn distance_between(&self, other_point: Position) -> f32{
		(self.y_distance(other_point).pow(2) as f32 + self.x_distance(other_point).pow(2) as f32).sqrt().abs()
	}
}

trait CoordManipulation{
	fn small_random_pos_offset(&mut self) -> Position;
	fn swap_x_y(&mut self) -> Position;
	fn alter_y(&self, i32) -> Position;
	fn alter_x(&self, i32) -> Position;
}

impl CoordManipulation for Position{
	fn small_random_pos_offset(&mut self) -> Position{
		let rand_num = || rand::thread_rng().gen_range(0, 8) as i32 - 4;
		let new_x = (self.x as i32 + rand_num()) as u32;
		let new_y = (self.y as i32 + rand_num()) as u32;
		Position{x: new_x, y: new_y}
	}
	
	fn swap_x_y(&mut self) -> Position{
		Position{x: self.y, y: self.x}
	}
	
	fn alter_y(&self, y_change: i32) -> Position{
		Position{x: self.x, y: (self.y as i32 + y_change).abs() as u32}  // need do handling of if give negative coord better
	}
	
	fn alter_x(&self, x_change: i32) -> Position{
		Position{x: (self.x as i32 + x_change).abs() as u32, y: self.y}  // need do handling of if give negative coord better
	}
}

#[derive(Copy, Clone)]
enum Lane{
	Top,
	Mid,
	Bot
}

#[derive(Copy, Clone, PartialEq)]
enum Side{
	Radiant,
	Dire
}

#[derive(Copy, Clone)]
struct Creep {
	side: Side,
	lane: Lane,
	hp: i32,
	attack_damage: u32,
	melee_attack: bool,
	attacking: bool,
	position: Position,
}

struct Game {
	game_tick: u64,
	lane_creeps: Vec<Creep>,
	towers: [Tower; 18],
}
	
trait TimeTick {
	fn new_game_tick(&mut self);
}

impl TimeTick for Game{
	fn new_game_tick(&mut self){
		self.game_tick += 1
	}
}

struct Tower{
	side: Side,
	lane: Lane,
	tier: u8,
	max_hp: u32,
	hp: i32,
	attack_damage: u32,
	attacked: bool,
	position: Position,
}

trait Attack{
	fn towers_attack(&mut self);
	
	fn lane_creeps_attack(&mut self);
}

impl Attack for Game{
	fn towers_attack(&mut self){
		for tower in &mut self.towers{
			tower.attacked = false;
			if tower.hp.is_positive(){
				for creep in &mut self.lane_creeps{
					if tower.position.distance_between(creep.position) < 12.0 && creep.side != tower.side && !tower.attacked{
						creep.hp -= tower.attack_damage as i32;
						tower.attacked = true;
						break;
					}
				}
			}
		}
	}
	
	fn lane_creeps_attack(&mut self){
		let clone = self.lane_creeps.clone();
		for (i, our_creep) in clone.clone().iter().enumerate(){
			let mut our_creep_attacked = false;
			let our_side: Side = our_creep.side;
			for other_creep in &mut self.lane_creeps{
				if other_creep.side != our_side && our_creep.position.distance_between(other_creep.position) < 12.0{
					other_creep.hp -= our_creep.attack_damage as i32;
					our_creep_attacked = true;
					break;
				};
			}
			
			if !our_creep_attacked{
				for tower in &mut self.towers{
					if tower.side != our_side && tower.hp.is_positive() && our_creep.position.distance_between(tower.position) < 12.0{
						tower.hp -= our_creep.attack_damage as i32;
						our_creep_attacked = true;
						break;
					};
				};
			}
			self.lane_creeps[i].attacking = our_creep_attacked;
			
		};
	}
}

trait Move{
	fn move_top_creeps_radiant(&mut self);
	fn move_mid_creeps_radiant(&mut self);
	fn move_bot_creeps_radiant(&mut self);
	fn move_top_creeps_dire(&mut self);
	fn move_mid_creeps_dire(&mut self);
	fn move_bot_creeps_dire(&mut self);
}

impl Move for Creep{
	fn move_top_creeps_radiant(&mut self){
		if self.position.y > (MAX_COORD / 8){
			self.position.y -= 1;
		}
		else{
			self.position.x +=1
		}
	}

	fn move_mid_creeps_radiant(&mut self){
		if 0 < self.position.y{
			if self.position.x < MAX_COORD{
				self.position.y -= 1;
				self.position.x += 1
			};
		};
	}

	fn move_bot_creeps_radiant(&mut self){
		if self.position.x < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.x += 1;
		}
		else{
			if 0 < self.position.y{
				self.position.y -=1;
			}
		}
	}

	fn move_mid_creeps_dire(&mut self){
		if self.position.y  < MAX_COORD{
			if 0 < self.position.x{
				self.position.y += 1;
				self.position.x -= 1
			};
		};
	}

	fn move_bot_creeps_dire(&mut self){
		if self.position.y < (MAX_COORD as f32 *(7.0/8.0)) as u32{
			self.position.y += 1;
		}
		else{
			if 0 < self.position.x{
				self.position.x -=1
			}
		}
	}

	fn move_top_creeps_dire(&mut self){
		if self.position.x > MAX_COORD / 8{
			self.position.x -= 1;
		}
		else{
			self.position.y +=1
		}
	}
}

trait MoveCreeps{
	fn move_creeps(&mut self);
}

impl MoveCreeps for Game{
	fn move_creeps(&mut self){
		for lane_creep in &mut self.lane_creeps{
			if !lane_creep.attacking{
				match lane_creep.side{
					Side::Radiant => match lane_creep.lane{
						Lane::Top => lane_creep.move_top_creeps_radiant(),
						Lane::Mid => lane_creep.move_mid_creeps_radiant(),
						Lane::Bot => lane_creep.move_bot_creeps_radiant()},
					_ => match lane_creep.lane{
							Lane::Top => lane_creep.move_top_creeps_dire(),
							Lane::Mid => lane_creep.move_mid_creeps_dire(),
							Lane::Bot => lane_creep.move_bot_creeps_dire()
						},
				};
			}
		}
	}
}

trait KillOff{
	fn kill_off_creeps(&mut self);
}

impl KillOff for Game{
	fn kill_off_creeps(&mut self){
		&mut self.lane_creeps.retain(|&i| i.hp > 0);
	}
}

fn main() {
    println!("Hello, world!");

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "spinning-square",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    window.set_ups(60);
        
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rsz_dota_minimap.jpg");
    let rust_logo = gTexture::from_path(
            //&mut window.factory,  these are here for piston window but not gl window
            &rust_logo,
            //Flip::None,
            //&TextureSettings::new()
	).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        window: window,
        background: &rust_logo
    };

    let mut events = app.window.events();
    while let Some(e) = events.next(&mut app.window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
            break;
        }
    }
	
	
	let tower = Tower{
			side: Side::Dire,
			lane: Lane::Mid,
			tier: 1,
			max_hp: 200,
			hp: 200,
			attack_damage: 30,
			attacked: false,
			position: Position{x: (MAX_COORD/2),
				y: (MAX_COORD/2),
				},
		};
	let t2_dire_pos = Position{x: (MAX_COORD/2) + (MAX_COORD/8), y: (MAX_COORD/2) - (MAX_COORD/8)};
	let t2_dire_tower = Tower{tier: 2, max_hp: 300, hp: 300, position: t2_dire_pos, .. tower};
	let t3_dire_tower = Tower{tier: 3, max_hp: 400, hp: 400,
		 position: Position{x: (MAX_COORD/2) + (MAX_COORD/4), y: (MAX_COORD/2) - (MAX_COORD/4)}, .. tower};
	
	let t1_rad_tower = Tower{side: Side::Radiant,
		 position: Position{x: (MAX_COORD/2) - (MAX_COORD/16), y: (MAX_COORD/2) + (MAX_COORD/16)}, .. tower};
	let t2_rad_tower = Tower{side: Side::Radiant,
		 position: Position{x: (MAX_COORD/2) - (MAX_COORD/16) - (MAX_COORD/8), y: (MAX_COORD/2) + (MAX_COORD/16) + (MAX_COORD/8)},
			  .. t2_dire_tower};
	let t3_rad_tower = Tower{side: Side::Radiant,
		 position: Position{x: (MAX_COORD/2) - (MAX_COORD/16) - (MAX_COORD/4), y: (MAX_COORD/2) + (MAX_COORD/16) + (MAX_COORD/4)},
			  .. t3_dire_tower};
	
	let t3_rad_top_tower = Tower{lane: Lane::Top, position: Position{x: MAX_COORD/12, y: (MAX_COORD as f32 *(12.0/16.0)) as u32},.. t3_rad_tower};
	let t2_rad_top_tower = Tower{tier: 2, position: t3_rad_top_tower.position.alter_y(-((MAX_COORD/8) as i32)), .. t3_rad_top_tower};
	let t1_rad_top_tower = Tower{tier: 1, position: t2_rad_top_tower.position.alter_y(-((MAX_COORD/8) as i32)), .. t2_rad_top_tower};
	
	let t3_rad_bot_tower = Tower{lane: Lane::Bot, position: Position{x: MAX_COORD/4, y: (MAX_COORD as f32 *(14.0/16.0)) as u32},.. t3_rad_tower};
	let t2_rad_bot_tower = Tower{tier: 2, position: t3_rad_bot_tower.position.alter_x((MAX_COORD/4) as i32), .. t3_rad_bot_tower};
	let t1_rad_bot_tower = Tower{tier: 1, position: t2_rad_bot_tower.position.alter_x((MAX_COORD/4) as i32), .. t2_rad_bot_tower};
	
	let t3_dire_top_tower = Tower{lane: Lane::Top, position: Position{x: MAX_COORD - MAX_COORD/4, y: (MAX_COORD as f32 *(2.0/16.0)) as u32},.. t3_dire_tower};
	let t2_dire_top_tower = Tower{tier: 2, position: t3_dire_top_tower.position.alter_x(-((MAX_COORD/4) as i32)), .. t3_dire_top_tower};
	let t1_dire_top_tower = Tower{tier: 1, position: t2_dire_top_tower.position.alter_x(-((MAX_COORD/4) as i32)), .. t2_dire_top_tower};
	
	let t3_dire_bot_tower = Tower{lane: Lane::Bot, position: Position{x: MAX_COORD - MAX_COORD/8, y: (MAX_COORD as f32 *(6.0/16.0)) as u32},.. t3_dire_tower};
	let t2_dire_bot_tower = Tower{tier: 2, position: t3_dire_bot_tower.position.alter_y((MAX_COORD/6) as i32), .. t3_dire_bot_tower};
	let t1_dire_bot_tower = Tower{tier: 1, position: t2_dire_bot_tower.position.alter_y((MAX_COORD/6) as i32), .. t2_dire_bot_tower};
	
	let mut game = Game{
		game_tick: 0,
		lane_creeps: vec!(),
		towers: [tower, t2_dire_tower, t3_dire_tower, t1_rad_tower, t2_rad_tower, t3_rad_tower, t3_rad_top_tower,
		t2_rad_top_tower, t1_rad_top_tower, t3_rad_bot_tower, t2_rad_bot_tower, t1_rad_bot_tower, t3_dire_top_tower, t2_dire_top_tower,
		t1_dire_top_tower, t3_dire_bot_tower, t2_dire_bot_tower, t1_dire_bot_tower],
	};
	
	loop {
		game.game_tick += 1;
		if game.game_tick % 280 == 0 || game.game_tick == 1{
			for _ in 1..5{
				let mut position = Position{
						x: MAX_COORD / 8,
						y: (MAX_COORD as f32 *(7.0/8.0)) as u32
						};
				let new_radiant_top_creep = Creep{
					side: Side::Radiant,
					lane: Lane::Top,
					hp: 150,
					attack_damage: 5,
					attacking: false,	
					melee_attack: true,
					position: position.small_random_pos_offset(),
				};
				let new_radiant_bot_creep = Creep{lane: Lane::Bot, position: position.small_random_pos_offset(), .. new_radiant_top_creep};
				let new_radiant_mid_creep = Creep{lane: Lane::Mid, position: position.small_random_pos_offset(), .. new_radiant_top_creep};
				let new_dire_top_creep = Creep{side: Side::Dire,
					 position: position.swap_x_y().small_random_pos_offset(),
					 attack_damage: 3, .. new_radiant_top_creep};
				let new_dire_bot_creep = Creep{lane: Lane::Bot, position: position.swap_x_y().small_random_pos_offset(), .. new_dire_top_creep};
				let new_dire_mid_creep = Creep{lane: Lane::Mid, position: position.swap_x_y().small_random_pos_offset(), .. new_dire_top_creep};
				game.lane_creeps.push(new_radiant_top_creep);
				game.lane_creeps.push(new_radiant_bot_creep);
				game.lane_creeps.push(new_radiant_mid_creep);
				game.lane_creeps.push(new_dire_top_creep);
				game.lane_creeps.push(new_dire_mid_creep);		
				game.lane_creeps.push(new_dire_bot_creep);
			};
		}
		println!("game time {}", game.game_tick);
		game.towers_attack();
		game.lane_creeps_attack();
		game.kill_off_creeps();
		if game.game_tick % 2 == 0{game.move_creeps();};
		while let Some(e) = events.next(&mut app.window) {
			if let Some(r) = e.render_args() {
				app.update_game(&game, &r);
				break;
			}
		}
	}
}
