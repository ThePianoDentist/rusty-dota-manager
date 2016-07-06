extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate rand;

use piston_window::*;
//use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, Texture as gTexture};
pub mod the_game;
use the_game::*;

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
