extern crate sdl2;
extern crate cgmath;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use cgmath::*;

use std::time::Duration;

fn perpendicular<T : std::ops::Neg<Output = T>>(vec : Vector2<T>) -> Vector2<T> {
    Vector2::<T::Output>::new(-vec.y, vec.x)
}

enum Axis {
    X, // EW
    Y, // NS
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (width, height) = (1200u32, 1000u32);

    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let grid_size = 1.0;

    #[derive(Copy, Clone)]
    enum LevelTile {
        Color(u8, u8, u8), // RGB
        Empty,
    }

    const RED_TILE : LevelTile = LevelTile::Color(255,0,0);
    const GRE_TILE : LevelTile = LevelTile::Color(0,255,0);
    const BLU_TILE : LevelTile = LevelTile::Color(0,0,255);
    const EMPTY    : LevelTile = LevelTile::Empty;

    const LEVEL_WIDTH  : usize = 16;
    const LEVEL_HEIGHT : usize = 32;
    let level : [[LevelTile ; LEVEL_HEIGHT] ; LEVEL_WIDTH] = [
        [RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE],
        [RED_TILE, RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
        [RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE],
    ];

    // const LEVEL_WIDTH  : usize = 8;
    // const LEVEL_HEIGHT : usize = 8;
    // let level : [[bool ; 8] ; 8] = [
    //     [true,  true,  false, true,  false, false, false, false],
    //     [true,  false, false, false, false, false, false, false],    
    //     [false, false, false, false, false, false, false, false],    
    //     [true,  false, false, false, false, false, false, false],    
    //     [false, false, false, false, false, false, false, false],    
    //     [true,  false, false, false, false, false, false, false],    
    //     [false, false, false, false, false, false, false, true ],    
    //     [true,  false, false, false, false, false, true,  true ],    
    // ];

    fn get_tile(ref level : &[[LevelTile ; LEVEL_HEIGHT] ; LEVEL_WIDTH], coord : Point2<i32>) -> LevelTile {
        if coord.x >= 0 && coord.x < (LEVEL_WIDTH as i32) && coord.y >= 0 && coord.y < (LEVEL_HEIGHT as i32) {
            level[coord.x as usize][coord.y as usize]
        } else {
            EMPTY
        }
    }

    let mut debug_view = false;


    // Camera position and direction vector.
    let mut cam_pos = Point2::<f32>::new(6.0, 6.0);
    let mut cam_dir = Vector2::<f32>::new(-1.0, -1.0).normalize();

    // Screen is perpendicular to and centered on cam_dir vector. We just need a size.
    // This is 1/2 screen size when one unit away from camera.
    let fov = 1.0; // 90 degrees

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Create a set of pressed Keys.
        let keys : HashSet<Keycode> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let rot : Basis2<f32> = Rotation2::from_angle(Rad::full_turn() / 180f32);

        for key in &keys {
            match key {
                &Keycode::M => { debug_view = !debug_view; },

                &Keycode::W => { cam_pos += cam_dir * 0.05; },
                &Keycode::S => { cam_pos -= cam_dir * 0.05; },
                &Keycode::A => { cam_pos -= perpendicular(cam_dir) * 0.05; },
                &Keycode::D => { cam_pos += perpendicular(cam_dir) * 0.05; },

                &Keycode::Up    => { cam_pos += cam_dir * 0.05; },
                &Keycode::Down  => { cam_pos -= cam_dir * 0.05; },
                &Keycode::Right => { cam_dir = rot.rotate_vector(cam_dir); },
                &Keycode::Left  => { cam_dir = rot.invert().rotate_vector(cam_dir); },
                _ => {}
            }
        }

        // Start rendering
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Some((0, 0, width, height/2).into())).unwrap();

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(Some((0, (height/2) as i32, width, height/2).into())).unwrap();

        let cam_coord : Point2<i32> = (cam_pos / grid_size).cast().unwrap();
        
        let plane_vec = Vector2::new(-cam_dir.y, cam_dir.x);

        for x in 0i32..(width as i32) {
            let w = width as f32;
            let ray_dir = cam_dir + plane_vec * (fov * (2.0*x as f32 - w)/w)  ;

            let delta_dist_x = 1.0/ray_dir.x.abs();
            let delta_dist_y = 1.0/ray_dir.y.abs();

            let step = Vector2::<i32>::new(
                if ray_dir.x > 0.0 {1} else {-1},
                if ray_dir.y > 0.0 {1} else {-1}
            );

            let mut side_dist = Vector2::new(
                if ray_dir.x > 0.0 {1.0 - cam_pos.x.fract()} else {cam_pos.x.fract()} * delta_dist_x,
                if ray_dir.y > 0.0 {1.0 - cam_pos.y.fract()} else {cam_pos.y.fract()} * delta_dist_y,
            );

            let mut current_coord = cam_coord.clone();
            let mut hit : Option<(Axis, LevelTile)> = None;
            for _iter in 0..(LEVEL_HEIGHT+LEVEL_WIDTH) {
                let pot_hit : Axis;
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist_x;
                    current_coord.x += step.x;
                    pot_hit = Axis::X
                } else {
                    side_dist.y += delta_dist_y;
                    current_coord.y += step.y;
                    pot_hit = Axis::Y
                }

                if let tile @ LevelTile::Color(_, _, _) = get_tile(&level, current_coord.cast().unwrap()) {
                    hit = Some((pot_hit, tile));
                    break;
                }
            }

            let perp_wall_dist = match hit {
                Some((Axis::X, tile)) => Some(((0.5 + (current_coord.x as f32) - cam_pos.x - (step.x as f32)/2.0) / ray_dir.x, tile)),
                Some((Axis::Y, tile)) => Some(((0.5 + (current_coord.y as f32) - cam_pos.y - (step.y as f32)/2.0) / ray_dir.y, tile)),
                _ => None
            };

            if let Some((d, tile)) = perp_wall_dist {
                let h_mid = (height / 2) as i32;
                let line_height = if d > 0f32 {
                    ((height as f32) / (4f32*d)) as i32
                } else {
                    height as i32
                };
                match tile {
                    LevelTile::Color(r,g,b) => {
                        canvas.set_draw_color(Color::RGB(r,g,b));
                        canvas.draw_line((x, h_mid + line_height), (x, h_mid - line_height)).unwrap();
                    },
                    _ => {}, 
                }
            }
        }

        if debug_view {
            canvas.set_draw_color(Color::RGB(50, 50, 50));
            canvas.fill_rect(Some((0,0,(LEVEL_WIDTH as u32)*8,(LEVEL_HEIGHT as u32)*8).into())).unwrap();
            for x in 0..(LEVEL_WIDTH as i32) {
                for y in 0..(LEVEL_HEIGHT as i32) {
                    if let LevelTile::Color(r,g,b) = get_tile(&level, Point2::new(x,y)) {
                        canvas.set_draw_color(Color::RGB(r, g, b));
                        canvas.fill_rect(Some((x*8,y*8,8,8).into())).unwrap();
                    }
                }
            }
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            let debug_cam_pos = sdl2::rect::Point::new((cam_pos.x * 8.0) as i32, (cam_pos.y * 8.0) as i32);
            let debug_cam_rect = (debug_cam_pos.x - 2, debug_cam_pos.y - 2, 4, 4).into();
            canvas.fill_rect(Some(debug_cam_rect)).unwrap();
            let debug_cam_dir_v = cam_dir * 8.0;  
            let debug_cam_dir = sdl2::rect::Point::new(debug_cam_dir_v.x as i32, debug_cam_dir_v.y as i32);
            canvas.draw_line(debug_cam_pos, debug_cam_pos + debug_cam_dir).unwrap();
        }

        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }
}