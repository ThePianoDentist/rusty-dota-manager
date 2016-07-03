extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use piston::window::WindowSettings;
use piston_window::{Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
use rand::Rng;


const MAX_COORD: u32  = 600;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        //use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 20.0);
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
			
			//image(rust_logo, c.transform, gl);
        });
    }
    
    fn update_creeps(&mut self, lane_creeps: &Vec<Creep>, args: &RenderArgs) {
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
		
		self.gl.draw(args.viewport(), |_, gl| {
				// Clear the screen.
				clear(GREEN, gl);
			});
        for creep in lane_creeps{
			self.gl.draw(args.viewport(), |c, gl| {
				//let num: f64 = rand::thread_rng().gen_range(0.0, 700.0);
				let x = (creep.x_coord) as f64;
				let y = (creep.y_coord) as f64; 
				let transform = c.transform.trans(x, y);
										   
				let square = rectangle::square(0.0, 0.0, 3.0);

				// Draw a box rotating around the middle of the screen.
				if creep.side == Side::Dire{
					ellipse(RED, square, transform, gl);
				}
				else{
					ellipse(BLUE, square, transform, gl);
				}
			});
		}
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
	hitpoints: i32,
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
	if lane_creep.y_coord > (MAX_COORD / 8){
		lane_creep.y_coord -= 1;
	}
	else{
		lane_creep.x_coord +=1
	}
}

fn move_mid_creeps_radiant(lane_creep: &mut Creep){
	if 0 < lane_creep.y_coord{
		if lane_creep.x_coord < MAX_COORD{
			lane_creep.y_coord -= 1;
			lane_creep.x_coord += 1
		};
	};
}

fn move_bot_creeps_radiant(lane_creep: &mut Creep){
	if lane_creep.x_coord < (MAX_COORD as f32 *(7.0/8.0)) as u32{
		lane_creep.x_coord += 1;
	}
	else{
		if 0 < lane_creep.y_coord{
			lane_creep.y_coord -=1;
		}
	}
}

fn move_mid_creeps_dire(lane_creep: &mut Creep){
	if lane_creep.y_coord  < MAX_COORD{
		if 0 < lane_creep.x_coord{
			lane_creep.y_coord += 1;
			lane_creep.x_coord -= 1
		};
	};
}

fn move_bot_creeps_dire(lane_creep: &mut Creep){
	if lane_creep.y_coord < (MAX_COORD as f32 *(7.0/8.0)) as u32{
		lane_creep.y_coord += 1;
	}
	else{
		if 0 < lane_creep.x_coord{
			lane_creep.x_coord -=1
		}
	}
}

fn move_top_creeps_dire(lane_creep: &mut Creep){
	if lane_creep.x_coord > MAX_COORD / 8{
		lane_creep.x_coord -= 1;
	}
	else{
		lane_creep.y_coord +=1
	}
}

fn lane_creeps_attack(lane_creeps: &mut Vec<Creep>){
	let mut creeps_to_destroy: Vec<usize> = vec!();
	let mut creeps_to_not_destroy: Vec<Creep> = vec!();
	let clone = lane_creeps.clone();
	for our_creep in clone.clone(){
		let our_side: Side = our_creep.side;
		//let mut i_want_to_moveit_moveit: bool = true;
		for (i, other_creep) in lane_creeps.iter_mut().enumerate(){
			if other_creep.side != our_side{
				let (x_distance_sq, y_distance_sq) : (u32, u32) = ((other_creep.x_coord as i32 - our_creep.x_coord as i32).pow(2) as u32, (other_creep.y_coord as i32 - our_creep.y_coord as i32).pow(2) as u32);
				if x_distance_sq < 4{
					if y_distance_sq < 4{
						other_creep.hitpoints -= our_creep.attack_damage as i32;
					};
				}
			}
		}
	};
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
					Lane::Bot => move_bot_creeps_radiant(lane_creep)},
				_ => match lane_creep.lane{
						Lane::Top => move_top_creeps_dire(lane_creep),
						Lane::Mid => move_mid_creeps_dire(lane_creep),
						Lane::Bot => move_bot_creeps_dire(lane_creep)
					},
			};
		}
	}
}

trait AttackCreeps{
	fn attack_creeps(&mut self);
}

impl AttackCreeps for Game{
	fn attack_creeps(&mut self){
		lane_creeps_attack(&mut self.lane_creeps)
	}
}

trait KillOffCreeps{
	fn kill_off_creeps(&mut self);
}

impl KillOffCreeps for Game{
	fn kill_off_creeps(&mut self){
		&mut self.lane_creeps.retain(|&i| i.hitpoints > 0);
	}
}

fn main() {
    println!("Hello, world!");

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust.png");
    let rust_logo = Texture::from_path(
            &mut window,
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
	).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
            break;
        }
    }
	
	let mut game = Game{
		game_time: 0,
		lane_creeps: vec!(),
	};
	
	loop {
		game.game_time += 1;
		//if game.game_time > 300{break;};
		if game.game_time % 280 == 0 || game.game_time == 1{
			for _ in 1..5{
				let mut new_radiant_top_creep = Creep{
					side: Side::Radiant,
					lane: Lane::Top,
					hitpoints: 150,
					attack_damage: 160,
					melee_attack: true,
					x_coord: MAX_COORD / 8,//4000,
					y_coord: (MAX_COORD as f32 *(7.0/8.0)) as u32,//2000,
				};
				let mut new_radiant_bot_creep = Creep{lane: Lane::Bot, .. new_radiant_top_creep};
				let mut new_radiant_mid_creep = Creep{lane: Lane::Mid, .. new_radiant_top_creep};
				let mut new_dire_top_creep = Creep{side: Side::Dire, x_coord: (MAX_COORD as f32 *(7.0/8.0)) as u32,
					 y_coord: MAX_COORD / 8, attack_damage: 20, .. new_radiant_top_creep};
				let mut new_dire_bot_creep = Creep{lane: Lane::Bot, .. new_dire_top_creep};
				let mut new_dire_mid_creep = Creep{lane: Lane::Mid, .. new_dire_top_creep};
				game.lane_creeps.push(new_radiant_top_creep);
				game.lane_creeps.push(new_radiant_bot_creep);
				game.lane_creeps.push(new_radiant_mid_creep);
				game.lane_creeps.push(new_dire_top_creep);
				game.lane_creeps.push(new_dire_mid_creep);		
				game.lane_creeps.push(new_dire_bot_creep);
			};
		}
		println!("game time {}", game.game_time);
		game.attack_creeps();
		game.kill_off_creeps();
		game.move_creeps();
		while let Some(e) = events.next(&mut window) {
			if let Some(r) = e.render_args() {
				app.update_creeps(&game.lane_creeps, &r);
				break;
			}
		}
		//app.update_creeps(&game.lane_creeps, &b);
	}
}
