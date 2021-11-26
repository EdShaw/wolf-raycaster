extern crate sdl2;
extern crate cgmath;
extern crate lodepng;
extern crate rgb;
extern crate freetype;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashSet;
use cgmath::*;
use rgb::*;
use std::time::Instant;

fn perpendicular<T : std::ops::Neg<Output = T>>(vec : Vector2<T>) -> Vector2<T> {
    Vector2::<T::Output>::new(-vec.y, vec.x)
}

const GRID_SIZE : f32 = 1.0;

enum Axis {
    X, // EW
    Y, // NS
}

#[derive(Copy, Clone)]
enum LevelTile {
    SolidTile(SolidTile),
    Empty,
}

#[derive(Copy, Clone)]
enum SolidTile {
    Color(u8, u8, u8), // RGB
    Textured(usize), // Texture index
}

const BST_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Textured(0));
const RBR_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Textured(1));
const EAG_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Textured(2));

const CEIL : [u8; 3] = [50, 50, 50];
const FLOOR : [u8; 3] = [100, 100, 100];

const RED_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Color(255, 0, 0));
const GRE_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Color(0, 255, 0));
const BLU_TILE : LevelTile = LevelTile::SolidTile(SolidTile::Color(0, 0, 255));
const EMPTY    : LevelTile = LevelTile::Empty;

const LEVEL_WIDTH  : usize = 16;
const LEVEL_HEIGHT : usize = 32;
static LEVEL : [[LevelTile ; LEVEL_HEIGHT] ; LEVEL_WIDTH] = [
    [RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE],
    [RBR_TILE, EAG_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EAG_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, BST_TILE, BST_TILE, BST_TILE, BST_TILE, BST_TILE, GRE_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, BST_TILE, BST_TILE, BST_TILE, BST_TILE, BST_TILE, GRE_TILE, GRE_TILE, EMPTY   , EMPTY   , EMPTY   , BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, BLU_TILE, EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EAG_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , BLU_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, EAG_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , GRE_TILE, EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , EMPTY   , RED_TILE],
    [RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RBR_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE, RED_TILE],
];

fn get_tile(ref level : &[[LevelTile ; LEVEL_HEIGHT] ; LEVEL_WIDTH], coord : Point2<i32>) -> LevelTile {
    if coord.x >= 0 && coord.x < (LEVEL_WIDTH as i32) && coord.y >= 0 && coord.y < (LEVEL_HEIGHT as i32) {
        level[coord.x as usize][coord.y as usize]
    } else {
        EMPTY
    }
}

const TEX_WIDTH : usize = 128;
const TEX_HEIGHT : usize = TEX_WIDTH;

type Tex = [u8 ; TEX_WIDTH*TEX_HEIGHT*3];
fn load_textures() -> Vec<Tex> {
    let bluestone : lodepng::Bitmap<lodepng::RGB<u8>> = lodepng::decode24_file("assets/freedoom/patches/brick.png").unwrap();
    let redbrick : lodepng::Bitmap<lodepng::RGB<u8>> = lodepng::decode24_file("assets/freedoom/patches/brick1.png").unwrap();
    let eagle : lodepng::Bitmap<lodepng::RGB<u8>> = lodepng::decode24_file("assets/freedoom/patches/brick2.png").unwrap();

    let mut bluestone_buf : Tex = [0; TEX_WIDTH*TEX_HEIGHT*3];
    let mut redbrick_buf : Tex = [0; TEX_WIDTH*TEX_HEIGHT*3];
    let mut eagle_buf : Tex = [0; TEX_WIDTH*TEX_HEIGHT*3];

    bluestone_buf.copy_from_slice(bluestone.buffer.as_bytes());
    redbrick_buf.copy_from_slice(redbrick.buffer.as_bytes());
    eagle_buf.copy_from_slice(eagle.buffer.as_bytes());

    return vec![
        bluestone_buf,
        redbrick_buf,
        eagle_buf,
    ];
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (width, height) = (1000u32, 800u32);

    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    let (width, height) = window.drawable_size();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Rotated 90 degrees, allowing us to access columns.
    let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, height, width).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut debug_view = false;

    let texture_manager = load_textures();

    // Text rendering
    // Init freetype
    let lib = freetype::Library::init().expect("Failed to load freetype");
    // Load a font face
    let face = lib.new_face("../common/Roboto_Mono/RobotoMono-Regular.ttf", 0).expect("Failed to load RobotoMono-Regular");

    // Camera position and direction vector.
    let mut cam_pos = Point2::<f32>::new(6.0, 6.0);
    let mut cam_dir = Vector2::<f32>::new(-1.0, -1.0).normalize();

    // Screen is perpendicular to and centered on cam_dir vector. We just need a size.
    // This is 1/2 screen size when one unit away from camera.
    let fov = 1.0; // 90 degrees

    let mut instant = Instant::now();

    'running: loop {
        let next_instant = Instant::now();
        let duration = next_instant.duration_since(instant);
        instant = next_instant;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Toggle debug
                Event::KeyDown { keycode: Some(Keycode::M), ..} => {
                    debug_view = !debug_view
                },
                _ => {}
            }
        }

        // Create a set of pressed Keys.
        let keys : HashSet<Keycode> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let rot : Basis2<f32> = Rotation2::from_angle(Rad::full_turn() / 180f32);

        for key in &keys {
            match key {
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

        let cam_coord : Point2<i32> = (cam_pos / GRID_SIZE).cast().unwrap();
        
        let plane_vec = Vector2::new(-cam_dir.y, cam_dir.x);

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // Gives us `width` chunks, each of size height (pitch).
            for (x, column) in buffer.chunks_mut(pitch).enumerate() {
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
                let mut hit : Option<(Axis, SolidTile)> = None;
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

                    if let LevelTile::SolidTile(solid_tile) = get_tile(&LEVEL, current_coord.cast().unwrap()) {
                        hit = Some((pot_hit, solid_tile));
                        break;
                    }
                }

                let perp_wall_dist = match hit {
                    Some((side@Axis::X, tile)) => Some(((0.5 + (current_coord.x as f32) - cam_pos.x - (step.x as f32)/2.0) / ray_dir.x, side, tile)),
                    Some((side@Axis::Y, tile)) => Some(((0.5 + (current_coord.y as f32) - cam_pos.y - (step.y as f32)/2.0) / ray_dir.y, side, tile)),
                    _ => None
                };

                if let Some((d, dir, solid_tile)) = perp_wall_dist {
                    // Line height assumed to be the "full" level height. Will need to change for shorter walls.
                    let line_height = 
                        ((height as f32) / (2.0_f32.sqrt() * d)) as i32;

                    // gap_top == gap_bottom due to symmetry. May be revisted if we shear for vertical look.
                    let gap_top = (height as i32 - line_height)/2;
                    let gap_top_u = gap_top.max(0) as usize;

                    let (top, rem) = column.split_at_mut(gap_top_u*3);
                    let (middle, bottom) = if line_height < (rem.len()/3) as i32 {
                        rem.split_at_mut(3*line_height as usize)
                    } else {
                        (rem, &mut [][..])
                    };   



                    match solid_tile {
                        SolidTile::Color(r,g,b) => {
                            for rgb in top.chunks_exact_mut(3) {
                                rgb.copy_from_slice(&CEIL);
                            }
                            for rgb in middle.chunks_exact_mut(3) {
                                rgb.copy_from_slice(&[r, g, b]);
                            }
                            for rgb in bottom.chunks_exact_mut(3) {
                                rgb.copy_from_slice(&FLOOR);
                            }
                        },
                        SolidTile::Textured(index) => {
                            // TODO: No-Float
                            let wall_x = match dir {
                                Axis::X => {cam_pos.y + d * ray_dir.y},
                                Axis::Y => {cam_pos.x + d * ray_dir.x},
                            }.fract().max(0.0).min(1.0);
                            
                            let x_offset = (wall_x * (TEX_WIDTH as f32)) as usize;
                            let tex_column = &texture_manager[index][x_offset*3*TEX_WIDTH..(x_offset*3*TEX_WIDTH + TEX_WIDTH*3)];

                            for rgb in top.chunks_exact_mut(3) {
                                rgb.copy_from_slice(&CEIL);
                            }
                            for (y, rgb) in middle.chunks_exact_mut(3).enumerate() {
                                // TODO: Calculating Y is too expensive. We can step y up based on line_height instead
                                let y = if gap_top < 0 {
                                    y as i32 - gap_top
                                } else {
                                    y as i32
                                };
                                let y_offset = (TEX_HEIGHT - 1) - ((TEX_HEIGHT as i32) * y / (line_height)).max(0).min((TEX_HEIGHT-1) as i32) as usize;

                                rgb.copy_from_slice(&tex_column[y_offset*3..(y_offset*3+3)]);
                            }
                            for rgb in bottom.chunks_exact_mut(3) {
                                rgb.copy_from_slice(&FLOOR);
                            }
                        },
                    }
                }
            }
        }).unwrap();

        canvas.set_draw_color(Color::RGB(100, 50, 50));
        canvas.clear();
        canvas.copy_ex(&texture,
                Some((0,0,width,height).into()),
                Some((0,0,height,width).into()),
                -90.0,
                Some(((height/2) as i32, (height/2) as i32).into()),
                false, false).unwrap();

        if debug_view {
            canvas.set_draw_color(Color::RGB(50, 50, 50));
            canvas.fill_rect(Some((0,0,(LEVEL_WIDTH as u32)*8,(LEVEL_HEIGHT as u32)*8).into())).unwrap();
            for x in 0..(LEVEL_WIDTH as i32) {
                for y in 0..(LEVEL_HEIGHT as i32) {
                    match get_tile(&LEVEL, Point2::new(x,y)) {
                        LevelTile::SolidTile(SolidTile::Color(r,g,b)) => {
                            canvas.set_draw_color(Color::RGB(r, g, b));
                            canvas.fill_rect(Some((x*8,y*8,8,8).into())).unwrap();
                        },
                        LevelTile::SolidTile(SolidTile::Textured(index)) => {
                            let texture = &texture_manager[index];
                            canvas.set_draw_color(Color::RGB(texture[0], texture[1], texture[2]));
                            canvas.fill_rect(Some((x*8,y*8,8,8).into())).unwrap();
                        },
                        LevelTile::Empty => {},
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
            
            // Frame time
            let duration = duration.as_secs() * 1_000 + (duration.subsec_millis() as u64);
            let duration = duration.to_string();
            let mut p = ((width-20) as i32, 10, 24, 24).into();
            for ch in duration.chars().rev() {
                draw_glyph(&mut canvas, &texture_creator, &face, ch, p);
                p.offset(-24, 0)
            }
        }

        canvas.present();
    }
}


fn draw_glyph<T : sdl2::render::RenderTarget, U>(
        canvas: &mut sdl2::render::Canvas<T>,
        texture_creator: &sdl2::render::TextureCreator<U>,
        face: &freetype::Face,
        ch: char,
        pos: sdl2::rect::Rect,
) {
    // Set the font size
    let size = 24 as usize;
    face.set_pixel_sizes(size as u32, size as u32).unwrap();
    face.load_char(ch as usize, freetype::face::LoadFlag::RENDER).unwrap();
    // Get the glyph instance
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();
    // TODO:
    let _pix_mode = bitmap.pixel_mode().unwrap();

    let b_width = bitmap.width() as usize;
    let b_height = bitmap.rows() as usize;
    let b_pitch = bitmap.pitch() as usize;
    let b_buf = bitmap.buffer();

    // Also TODO:
    let _x = glyph.bitmap_left() as usize;
    let _y = b_height - glyph.bitmap_top() as usize;

    let mut glyph_tex = texture_creator.create_texture_streaming(PixelFormatEnum::ARGB8888, size as u32, size as u32).unwrap();
    glyph_tex.with_lock(None, |buf: &mut [u8], pitch: usize|{
        for j in 0..size {
            for i in 0..size {
                buf[(4*i + j*pitch) + 0] = 0;
                buf[(4*i + j*pitch) + 1] = 0;
                buf[(4*i + j*pitch) + 2] = 0;
                if i < b_width && j < b_height {
                    buf[(4*i + j*pitch) + 3] = b_buf[i + j*b_pitch];
                } else {
                    buf[(4*i + j*pitch) + 3] = 0;
                }
            }
        }
    }).unwrap();

    glyph_tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.copy(&glyph_tex, None, Some(pos)).unwrap();
}
