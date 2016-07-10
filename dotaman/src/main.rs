extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate freetype;
extern crate rand;

use piston_window::*;
use opengl_graphics::{GlGraphics, Texture as gTexture};
use graphics::math::Matrix2d;
#[macro_use]
pub mod the_game;
pub mod position;
use position::*;
use the_game::*;
pub mod anhero;
use anhero::*;
pub mod hero_ai;
use hero_ai::*;

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
    background: &'a opengl_graphics::Texture, // no idea why the 'a necessary. but doesnt work without. im such bad progrummer lol
}

impl<'a> App<'a>{
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


		for i in 0..2{
			let team = &game.teams[i];
			//Draw Towers
			for tower in team.towers.iter(){
				self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(tower.position.x as f64, tower.position.y as f64);

					let square = rectangle::centered_square(0.0, 0.0, 5.0);
					match team.side{
						Side::Radiant  => rectangle(DARK_GREEN, square, transform, gl),
						Side::Dire => rectangle(DARK_RED, square, transform, gl)
					}

					if !tower.can_action{ellipse(YELLOW, rectangle::centered_square(0.0, 0.0, 2.0),
						 c.transform.trans(tower.position.x as f64, tower.position.y as f64), gl);};
				});
			};

			//Draw Thrones
			self.gl.draw(args.viewport(), |c, gl| {
				let transform = c.transform.trans(team.throne.position.x as f64, team.throne.position.y as f64);
				let square = rectangle::centered_square(0.0, 0.0, 15.0);
				match team.side{
					Side::Radiant  => ellipse(DARK_GREEN, square, transform, gl),
					Side::Dire => ellipse(DARK_RED, square, transform, gl)
				}
			});

			//Draw Fountains
			self.gl.draw(args.viewport(), |c, gl| {
				let fountain = &team.fountain;
				let transform = c.transform.trans(fountain.position.x as f64, fountain.position.y as f64);
				let square = rectangle::centered_square(0.0, 0.0, 5.0);
				match team.side{
					Side::Dire => rectangle(DARK_RED, square, transform, gl),
					Side::Radiant => rectangle(DARK_GREEN, square, transform, gl)
				}
				if !fountain.can_action{ellipse(YELLOW, rectangle::centered_square(0.0, 0.0, 2.0),
							 c.transform.trans(fountain.position.x as f64, fountain.position.y as f64), gl);};
			});

		// Draw Lane Creeps
			for creep in &team.lane_creeps{
				self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(creep.position.x as f64, creep.position.y as f64);

					let square = rectangle::centered_square(0.0, 0.0, 3.5);
					match team.side{
						Side::Radiant => ellipse(GREEN, square, transform, gl),
						Side::Dire => ellipse(RED, square, transform, gl)
					}
				});
			}

			// Draw Heroes
			for hero in &team.heroes{
				self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(hero.position.x.round() as f64, hero.position.y.round() as f64).trans(-16.0, -16.0);
					image(&hero.pic, transform, gl);
				});
			}
		}
	}

    fn win_game(&mut self, side: &Side, args: &RenderArgs){
		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets").unwrap();
		let freetype = freetype::Library::init().unwrap();
		let font = assets.join("FiraSans-Regular.ttf");
		let mut face = freetype.new_face(&font, 0).unwrap();
		face.set_pixel_sizes(0, 48).unwrap();

		self.gl.draw(args.viewport(), |c, gl| {
					let transform = c.transform.trans(0.0, 100.0);

					clear(color::WHITE, gl);
					match *side{
						Side::Dire => render_text(&mut face, gl, transform, "RADIANT VICTORY!"),
						Side::Radiant => render_text(&mut face, gl, transform, "Dire VICTORY!")
					}
		});
	}
}

fn wtf(i: usize) -> usize{
	((i as i32 - 1).abs()) as usize
}

fn main() {
    println!("Hello, world!");

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "ooooh-shit-the-absolute-madman-it's-a-dota-football-manager-ripoff-blobs-everywhere",
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
    let rubick_pic = gTexture::from_path(
					 assets.join("Rubick_icon.png")).unwrap();

	let ta_pic = gTexture::from_path(
					 assets.join("Templar_Assassin_icon.png")).unwrap();
	let enigma_pic = gTexture::from_path(
					 assets.join("Enigma_icon.png")).unwrap();
	let batrider_pic = gTexture::from_path(
					 assets.join("Batrider_icon.png")).unwrap();
	let alchemist_pic = gTexture::from_path(
					 assets.join("Alchemist_icon.png")).unwrap();
	let io_pic = gTexture::from_path(
					 assets.join("Io_icon.png")).unwrap();
	let cm_pic = gTexture::from_path(
					 assets.join("Crystal_Maiden_icon.png")).unwrap();
	let np_pic = gTexture::from_path(
					 assets.join("Natures_Prophet_icon.png")).unwrap();
	let puck_pic = gTexture::from_path(
					 assets.join("Puck_icon.png")).unwrap();
	let ck_pic = gTexture::from_path(
					 assets.join("Chaos_Knight_icon.png")).unwrap();

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
			lane: Lane::Mid,
			tier: 1,
			max_hp: 200,
			hp: 200,
			attack_damage: 30,
			can_action: true,
			attack_cooldown: 3.0,
			attack_rate: 3.0,
			range: 30.,
			position: Position{x: (MAX_COORD/2.),
				y: (MAX_COORD/2.),
				},
		};
	let t2_dire_pos = Position{x: (MAX_COORD/2.) + (MAX_COORD/8.), y: (MAX_COORD/2.) - (MAX_COORD/8.)};
	let t2_dire_tower = Tower{tier: 2, max_hp: 300, hp: 300, position: t2_dire_pos, .. tower};
	let t3_dire_tower = Tower{tier: 3, max_hp: 400, hp: 400,
		 position: Position{x: (MAX_COORD/2.) + (MAX_COORD/4.), y: (MAX_COORD/2.) - (MAX_COORD/4.)}, .. tower};

	let t1_rad_tower = Tower{
		 position: Position{x: (MAX_COORD/2.) - (MAX_COORD/16.), y: (MAX_COORD/2.) + (MAX_COORD/16.)}, .. tower};
	let t2_rad_tower = Tower{
		 position: Position{x: (MAX_COORD/2.) - (MAX_COORD/16.) - (MAX_COORD/8.), y: (MAX_COORD/2.) + (MAX_COORD/16.) + (MAX_COORD/8.)},
			  .. t2_dire_tower};
	let t3_rad_tower = Tower{
		 position: Position{x: (MAX_COORD/2.) - (MAX_COORD/16.) - (MAX_COORD/4.), y: (MAX_COORD/2.) + (MAX_COORD/16.) + (MAX_COORD/4.)},
			  .. t3_dire_tower};

	let t3_rad_top_tower = Tower{lane: Lane::Top, position: Position{x: MAX_COORD/12., y: MAX_COORD *(12.0/16.0)},.. t3_rad_tower};
	let t2_rad_top_tower = Tower{tier: 2, position: t3_rad_top_tower.position.alter_y(-MAX_COORD/8.), .. t3_rad_top_tower};
	let t1_rad_top_tower = Tower{tier: 1, position: t2_rad_top_tower.position.alter_y(-MAX_COORD/8.), .. t2_rad_top_tower};

	let t3_rad_bot_tower = Tower{lane: Lane::Bot, position: Position{x: MAX_COORD/4., y: MAX_COORD * (14.0/16.0)},.. t3_rad_tower};
	let t2_rad_bot_tower = Tower{tier: 2, position: t3_rad_bot_tower.position.alter_x(MAX_COORD/4.), .. t3_rad_bot_tower};
	let t1_rad_bot_tower = Tower{tier: 1, position: t2_rad_bot_tower.position.alter_x(MAX_COORD/4.), .. t2_rad_bot_tower};

	let t3_dire_top_tower = Tower{lane: Lane::Top, position: Position{x: MAX_COORD - MAX_COORD/4., y: MAX_COORD * (2.0/16.0)},.. t3_dire_tower};
	let t2_dire_top_tower = Tower{tier: 2, position: t3_dire_top_tower.position.alter_x(-MAX_COORD/4.), .. t3_dire_top_tower};
	let t1_dire_top_tower = Tower{tier: 1, position: t2_dire_top_tower.position.alter_x(-MAX_COORD/4.), .. t2_dire_top_tower};

	let t3_dire_bot_tower = Tower{lane: Lane::Bot, position: Position{x: MAX_COORD - MAX_COORD/8., y: MAX_COORD*(6.0/16.0)},.. t3_dire_tower};
	let t2_dire_bot_tower = Tower{tier: 2, position: t3_dire_bot_tower.position.alter_y(MAX_COORD/6.), .. t3_dire_bot_tower};
	let t1_dire_bot_tower = Tower{tier: 1, position: t2_dire_bot_tower.position.alter_y(MAX_COORD/6.), .. t2_dire_bot_tower};

	let radiant_fount = Fountain{attack_damage: 300, range: 30., hp: 9999, attack_cooldown: 1.3, attack_rate: 1.3, can_action: true,//test with multiple of so get 0
		 position: Position{x: (MAX_COORD/16.7).round(), y:(MAX_COORD - (MAX_COORD/16.7)).round()}};

	let dire_fount = Fountain{position: radiant_fount.position.swap_x_y(), .. radiant_fount};

	let radiant_throne = Throne{max_hp: 1000, hp: 1000,
		 position: Position{x: 2.3*MAX_COORD/16.7, y:MAX_COORD - (2.3*MAX_COORD/16.7)}};

	let dire_throne = Throne{position: radiant_throne.position.swap_x_y(), .. radiant_throne};

	let rubick = Hero{name: "rubick",
					pic: rubick_pic,
					base_int: 27,
					base_str: 19,
					base_agi: 14,
					int_gain: 2.4,
					str_gain: 1.5,
					agi_gain: 1.6,
					base_attack_damage: 27.0, // 17-27
					move_speed: 290,

					can_action: true,
					attack_damage: 27.0,
					max_hp: 200.,
					max_mana: 50.0,
					gold: 650.0,
					hp: 200.,
					hp_regen: 0.25,
					mana: 50.0,
					mana_regen: 0.01,
					attack_cooldown: -0.0001,
					attack_rate: 1.0,
					position: radiant_fount.position,
					level: 1,
					armour: -1.0,
					velocity: Velocity{x: 0., y: 0.},
					respawn_timer: 0,
					range: 30.,
                    decisions: vec!(),
		};

	let enigma = Hero{pic: enigma_pic, decisions: vec!(), .. rubick};
	let alchemist = Hero{pic: alchemist_pic, decisions: vec!(), .. rubick};
	let batrider = Hero{pic: batrider_pic, decisions: vec!(), .. rubick};
	let ta = Hero{pic: ta_pic, decisions: vec!(), .. rubick};
	let puck = Hero{position: dire_fount.position, pic: puck_pic, decisions: vec!(), .. rubick};
	let io = Hero{position: dire_fount.position, pic: io_pic, decisions: vec!(), .. rubick};
	let cm = Hero{position: dire_fount.position, pic: cm_pic, decisions: vec!(), .. rubick};
	let ck = Hero{position: dire_fount.position, pic: ck_pic, name: "ck", decisions: vec!(), .. rubick};
	let np = Hero{position: dire_fount.position, pic: np_pic, decisions: vec!(), .. rubick};


	let radiant = Team{side: Side::Radiant, towers: vec!(t1_rad_tower, t2_rad_tower, t3_rad_tower, t3_rad_top_tower,
		t2_rad_top_tower, t1_rad_top_tower, t3_rad_bot_tower, t2_rad_bot_tower, t1_rad_bot_tower),
		fountain: radiant_fount, throne: radiant_throne, lane_creeps: vec!(), heroes: [rubick, enigma, alchemist, ta, batrider]};

	let dire = Team{side: Side::Dire, towers: vec!(tower, t2_dire_tower, t3_dire_tower, t3_dire_top_tower, t2_dire_top_tower,
		 t1_dire_top_tower, t3_dire_bot_tower, t2_dire_bot_tower, t1_dire_bot_tower),
		fountain: dire_fount, throne: dire_throne, lane_creeps: vec!(), heroes: [io, cm, ck, np, puck]};

	let mut game = Game{
		game_tick: 0,
		game_time: 0.0,
		teams: [radiant, dire],
	};

	'outer: loop {
		if game.game_tick % 280 == 0 || game.game_tick == 0{
			for _ in 1..5{
				let mut position = Position{
						x: MAX_COORD / 8.,
						y: MAX_COORD*(7.0/8.0)
						};
				let new_radiant_top_creep = Creep{
					lane: Lane::Top,
					hp: 150,
					attack_damage: 5,
					attack_cooldown: 1.6,
					attack_rate: 1.6,
					melee_attack: true,
					can_action: true,
					velocity: Velocity{x: 0., y: -1.},
					range: 12.,
					position: position.small_random_pos_offset(),
				};
				let new_radiant_bot_creep = Creep{lane: Lane::Bot, position: position.small_random_pos_offset(),
					 velocity: Velocity{x: 1., y: 0.}, .. new_radiant_top_creep};
				let new_radiant_mid_creep = Creep{lane: Lane::Mid, position: position.small_random_pos_offset(),
					velocity: Velocity{x: 1., y: -1.}, .. new_radiant_top_creep};
				let new_dire_top_creep = Creep{position: position.swap_x_y().small_random_pos_offset(),
					 attack_damage: 3, velocity: Velocity{x: -1., y: 0.}, .. new_radiant_top_creep};
				let new_dire_bot_creep = Creep{lane: Lane::Bot, position: position.swap_x_y().small_random_pos_offset(),
					velocity: Velocity{x: 0., y: 1.}, .. new_dire_top_creep};
				let new_dire_mid_creep = Creep{lane: Lane::Mid, position: position.swap_x_y().small_random_pos_offset(),
					velocity: Velocity{x: -1., y: 1.}, .. new_dire_top_creep};
				game.teams[0].lane_creeps.push(new_radiant_top_creep);
				game.teams[0].lane_creeps.push(new_radiant_bot_creep);
				game.teams[0].lane_creeps.push(new_radiant_mid_creep);
				game.teams[1].lane_creeps.push(new_dire_top_creep);
				game.teams[1].lane_creeps.push(new_dire_mid_creep);
				game.teams[1].lane_creeps.push(new_dire_bot_creep);
			};
		}
		println!("game time {}", game.game_tick);

		game.reset_all_attack_cooldown();

		for i in 0..2{  // this surely makes game imbalanced.
            let (rad, dire) = game.teams.split_at_mut(1);
			let (mut us, mut them) = match i{
				0 => (&mut rad[0], &mut dire[0]),
				1 => (&mut dire[0], &mut rad[0]),
				_ => panic!("I'm pretty sure this is impossible")
			};

            us.fountain.attack_enemy_creeps(&mut them.lane_creeps);

            for tower in &mut us.towers{
				tower.attack_enemy_creeps(&mut them.lane_creeps);
				tower.attack_closest_hero(&mut them.heroes)
			};

            for creep in &mut us.lane_creeps{
				creep.attack_enemy_creeps(&mut them.lane_creeps);
				if !creep.can_action{continue};
				creep.attack_towers(&mut them.towers);
				if !creep.can_action{continue};
				creep.attack_closest_hero(&mut them.heroes);
				if !creep.can_action{continue};
				creep.attack_throne(&mut them.throne);
			}

            if game.game_tick == 0{
                let my_decision = Decision{action: Action::FARM_TOP_LANE, probability: 1.};
                us.heroes[3].decisions.push(my_decision);
            }

            if i == 0{
            match game.game_tick{
    			x if x < 1300 => us.heroes[0].move_towards_creeps(Lane::Bot, &them.lane_creeps),
    			x if x < 2000 => us.heroes[0].move_towards_creeps(Lane::Mid, &them.lane_creeps),
    			_ => us.heroes[0].move_towards_creeps(Lane::Top, &them.lane_creeps),
    		}}

            if i == 1{
    		us.heroes[0].move_towards_creeps(Lane::Mid, &them.lane_creeps);
    		us.heroes[1].move_towards_creeps(Lane::Top, &them.lane_creeps);
    		us.heroes[2].move_towards_creeps(Lane::Bot, &them.lane_creeps);
            }
            else{us.heroes[1].move_towards_creeps(Lane::Mid, &them.lane_creeps);
    		us.heroes[2].move_towards_creeps(Lane::Top, &them.lane_creeps);
    		//us.heroes[3].move_towards_creeps(Lane::Bot, &them.lane_creeps, &game.TOP_LANE_VERTEX, &game.BOT_LANE_VERTEX);
    		us.heroes[4].move_towards_creeps(Lane::Mid, &us.lane_creeps);
            }
            //for hero in us.heroes.iter_mut(){
            //    if hero.decisions.len() > 0{
                us.heroes[3].make_decision(&mut us.lane_creeps, &mut them.lane_creeps, &mut them.towers);
                println!("GOOLD {}", us.heroes[3].gold);
            //}
            //    println!("hehe");
            //}
        };

		game.kill_off_creeps();
		game.kill_off_heroes();
		if game.game_tick % 2 == 0 {game.teams[0].move_creeps_radiant();
			game.teams[1].move_creeps_dire()};
		while let Some(e) = events.next(&mut app.window) {
			if let Some(r) = e.render_args() {
				app.update_game(&game, &r);
				break;
			}
		}
		for team in game.teams.iter(){
			if !team.throne.hp.is_positive(){
				while let Some(e) = events.next(&mut app.window){
					if let Some(r) = e.render_args() {
						app.win_game(&team.side, &r)
					}
				}
				break 'outer;
			}
		};
		game.game_tick += 1;
		match game.game_tick{
			x if x < 1300 => game.teams[0].heroes[0].move_towards_creeps(Lane::Bot, &game.teams[1].lane_creeps),
			x if x < 2000 => game.teams[0].heroes[0].move_towards_creeps(Lane::Mid, &game.teams[1].lane_creeps),
			_ => game.teams[0].heroes[0].move_towards_creeps(Lane::Top, &game.teams[1].lane_creeps),
		}
		game.teams[1].heroes[0].move_towards_creeps(Lane::Mid, &game.teams[0].lane_creeps);
		game.teams[1].heroes[1].move_towards_creeps(Lane::Top, &game.teams[0].lane_creeps);
		game.teams[1].heroes[2].move_towards_creeps(Lane::Bot, &game.teams[0].lane_creeps);
		game.teams[0].heroes[1].move_towards_creeps(Lane::Mid, &game.teams[1].lane_creeps);
		game.teams[0].heroes[2].move_towards_creeps(Lane::Top, &game.teams[1].lane_creeps);
		//game.teams[0].heroes[3].move_towards_creeps(Lane::Bot, &game.teams[1].lane_creeps, &game.TOP_LANE_VERTEX, &game.BOT_LANE_VERTEX);
		game.teams[0].heroes[4].move_towards_creeps(Lane::Mid, &game.teams[0].lane_creeps);
	}
}
