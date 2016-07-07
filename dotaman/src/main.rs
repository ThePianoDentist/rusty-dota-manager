extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate rand;
extern crate freetype;

use piston_window::*;
//use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, Texture as gTexture};
use graphics::math::Matrix2d;
pub mod the_game;
use the_game::*;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DARK_RED: [f32; 4] = [0.8, 0.1, 0.1, 1.0];
const DARK_GREEN: [f32; 4] = [0.2, 0.8, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

fn render_text(face: &mut freetype::Face, gl: &mut GlGraphics, t: Matrix2d, text: &str) {
    let mut x = 10;
    let mut y = 0;
    for ch in text.chars() {
        use graphics::*;
        
        face.load_char(ch as usize, freetype::face::RENDER).unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = gTexture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new()
        ).unwrap();
        Image::new_color(color::BLACK).draw(
            &texture,
            &Default::default(),
            t.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
            gl
        );

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}

pub struct App<'a>{
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
											   
					let square = rectangle::centered_square(0.0, 0.0, 5.0);
					
					if tower.side == Side::Dire{
						rectangle(DARK_RED, square, transform, gl);
					}
					else{
						rectangle(DARK_GREEN, square, transform, gl);
					}
					
					if tower.attacked{ellipse(YELLOW, rectangle::centered_square(0.0, 0.0, 2.0),
						 c.transform.trans(tower.position.x as f64, tower.position.y as f64), gl);};
				});
			}
		};
		
		for throne in &game.thrones{
			if throne.hp.is_positive(){
				self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(throne.position.x as f64, throne.position.y as f64);
											   
					let square = rectangle::centered_square(0.0, 0.0, 15.0);
					
					if throne.side == Side::Dire{
						ellipse(DARK_RED, square, transform, gl);
					}
					else{
						ellipse(DARK_GREEN, square, transform, gl);
					}
				});
			}
		};
		
		for fount in &game.fountains{
			self.gl.draw(args.viewport(), |c, gl| {
				let transform = c.transform.trans(fount.position.x as f64, fount.position.y as f64);
										   
				let square = rectangle::centered_square(0.0, 0.0, 5.0);
				
				if fount.side == Side::Dire{
					rectangle(DARK_RED, square, transform, gl);
				}
				else{
					rectangle(DARK_GREEN, square, transform, gl);
				}
			});
		};
		
		
		
        for creep in &game.lane_creeps{
			self.gl.draw(args.viewport(), |c, gl| {
				let transform = c.transform.trans(creep.position.x as f64, creep.position.y as f64);
										   
				let square = rectangle::centered_square(0.0, 0.0, 3.5);

				if creep.side == Side::Dire{
					ellipse(RED, square, transform, gl);
				}
				else{
					ellipse(GREEN, square, transform, gl);
				}
			});
		}
    }
    
    fn win_game(&mut self, game: &Game, args: &RenderArgs){
		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets").unwrap();
		let freetype = freetype::Library::init().unwrap();
		let font = assets.join("FiraSans-Regular.ttf");
		let mut face = freetype.new_face(&font, 0).unwrap();
		face.set_pixel_sizes(0, 48).unwrap();
		
		self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(0.0, 100.0);

					clear(color::WHITE, gl);
					render_text(&mut face, gl, transform, "RADIANT VICTORY!");
		});
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
	
	let radiant_fount = Fountain{side: Side::Radiant, attack_damage: 300,	attacked: false,
		 position: Position{x: (1.0*(MAX_COORD as f32)/16.7) as u32, y:MAX_COORD - (1.0*(MAX_COORD as f32)/16.7) as u32}};
		 
	let dire_fount = Fountain{side: Side::Dire, position: radiant_fount.position.swap_x_y(), .. radiant_fount};
	
	let radiant_throne = Throne{side: Side::Radiant, max_hp: 1000, hp: 1000,
		 position: Position{x: (2.3*(MAX_COORD as f32)/16.7) as u32, y:MAX_COORD - (2.3*(MAX_COORD as f32)/16.7) as u32}};
		 
	let dire_throne = Throne{side: Side::Dire, position: radiant_throne.position.swap_x_y(), .. radiant_throne};
		 
	let mut game = Game{
		game_tick: 0,
		lane_creeps: vec!(),
		towers: [tower, t2_dire_tower, t3_dire_tower, t1_rad_tower, t2_rad_tower, t3_rad_tower, t3_rad_top_tower,
		t2_rad_top_tower, t1_rad_top_tower, t3_rad_bot_tower, t2_rad_bot_tower, t1_rad_bot_tower, t3_dire_top_tower, t2_dire_top_tower,
		t1_dire_top_tower, t3_dire_bot_tower, t2_dire_bot_tower, t1_dire_bot_tower],
		fountains: [radiant_fount, dire_fount],
		thrones: [radiant_throne, dire_throne]
	};
	
	'outer: loop {
		if game.game_tick % 280 == 0 || game.game_tick == 0{
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
		game.fountains_attack();
		game.lane_creeps_attack();
		game.kill_off_creeps();
		if game.game_tick % 2 == 0{game.move_creeps();};
		while let Some(e) = events.next(&mut app.window) {
			if let Some(r) = e.render_args() {
				app.update_game(&game, &r);
				break;
			}
		}
		'inner: for throne in &game.thrones{
			if !throne.hp.is_positive(){
				'valid: while let Some(e) = events.next(&mut app.window){
					if let Some(r) = e.render_args() {
						app.win_game(&game, &r);
					} 
				}
				break 'outer;
			}
		}
		game.game_tick += 1;
	}
}
