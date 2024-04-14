use raylib::{misc::AsF32, prelude::*};
use std::fmt::Error;
use std::collections::HashMap;

const WINDOWWIDTH: i32 = 800;
const WINDOWHEIGHT: i32 =800;
const INFINITY: i32 = 1_000_000_000;

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct GridWorld {
    width: i32,
    height: i32,

    start: Point,
    end: Point,
    walls: Vec<bool>,
    current: Point,

    unvisited: HashMap<i32,Point>,
    distances: Vec<f32>,

    path: Vec<Point>,
}

impl GridWorld {
    fn wall_at(&self, x: i32, y: i32) -> bool{
        return self.walls[(y*self.width+x) as usize ]

    }

    fn set_wall_at(&mut self, x: i32, y: i32, value: bool) {
        self.walls[(y*self.width+x) as usize] = value;
    }
}
fn center_rect(rect: Rectangle, relative_width: f32, relative_height: f32) -> Result<Rectangle, Error> {
    return Ok(Rectangle::new(rect.x + rect.width*(1.0-relative_width)/2.0, 
                             rect.y  + rect.height*(1.0-relative_height)/2.0, 
                             rect.width * relative_width, 
                             rect.height * relative_height)
            );
}

fn draw_grid(drawing:  &mut RaylibDrawHandle,location: Rectangle, world: &mut GridWorld) -> Result<(), Error>{

    let cell_width: f32 = location.width / world.width as f32 ;
    let cell_height: f32 = location.height / world.height as f32 ;
    let mouse_position: Vector2 = drawing.get_mouse_position();

    for i in 0..world.height {
        for j in 0..world.width {

            let mut cell = Rectangle{
                x: location.x,
                y: location.y,
                width: cell_width,
                height: cell_height,
            };

            cell.x += j as f32 * cell.width;
            cell.y += i as f32 * cell.height;

            let mut  color = Color::ORANGE;

            if world.wall_at(j as i32, i as i32) {
                color = Color::DARKPURPLE;
            }
            let point= Point{x: j as i32 , y: i as i32};
			if !point_is_unvisited(world, point) {
				color = Color::LIGHTGRAY;

			}
            if j == world.start.x && i == world.start.y {
                color = Color::GREEN;
            }

            if j == world.end.x && i == world.end.y {
                color = Color::RED;
            }
            drawing.draw_rectangle_rec(cell, color);
            drawing.draw_rectangle_lines_ex(cell, 2, Color::DARKPURPLE);
			

            if !point_is_unvisited(world, point) {
                let text = format!("{}", distance_at(world, point));
                drawing.draw_text(text.as_str(), cell.x as i32+ 5 , cell.y as i32 + 5, 25, Color::BLACK)
            }
            
            if cell.check_collision_point_rec(mouse_position) {
                if drawing.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
                world.set_wall_at(j as i32, i as i32, !world.wall_at(j as i32, i as i32));
                }
                if drawing.is_key_released(KeyboardKey::KEY_ONE) {
                    world.start = point;
                    reset_world(world)
                }
                // Change the position of the end point
                if drawing.is_key_released(KeyboardKey::KEY_TWO)  {
                    world.end = point;
                    reset_world(world);
                }    
            }


            if point_in_path(world, point) {
                drawing.draw_rectangle_lines_ex(cell, 5, Color::GOLD);
                let text = format!("{}", distance_at(world, point));
                drawing.draw_text(&text, cell.x  as i32 + 5 , cell.y as i32 + 5 , 25, Color::BEIGE);

            }

        }
    }
    Ok(())

}


fn step_dijkstra( world: &mut GridWorld) {

	if !point_is_unvisited(world, world.end) {
		return
	}

    for dx in -1..=1 {
        for dy in -1..=1{
            if dx == 0 && dy == 0 {
                continue;
            }
            if dx != 0 && dy != 0 {
                continue;
            }

            let neighbour= Point{
                x: world.current.x + dx as i32,
                y: world.current.y + dy as i32,
            };
            if !point_in_bounds(world, neighbour) {
				continue
			}
			if !point_is_unvisited(world, neighbour) {
				continue
			}

			if world.wall_at(neighbour.x, neighbour.y) { 
				continue
			}

            let dist_current_to_neighbour: f32 = 1.0;
			let distance = distance_at(world, world.current) + dist_current_to_neighbour;


			if distance < distance_at(world, neighbour) {
				set_distance_at(world, neighbour, distance)
			}

        }
    }
    match world.unvisited.get(&(world.current.y*world.width + world.current.x)) {
        Some(_) =>  {world.unvisited.remove(&(world.current.y*world.width + world.current.x));},
        None =>  {{};},
    }

    let mut min_index: i32 = -1;
    let mut min_distance: f32 = INFINITY as f32;

    let _vec_test: Vec<()> = world.unvisited.iter().map(|(key, point)|{ 
        let dist = distance_at(world, *point);
        if dist < min_distance {
            min_distance = dist;
            min_index = *key;
        }
    }).collect();
    if min_index != -1 {
        world.current = world.unvisited[&min_index]
    }

}

fn point_in_bounds(world: &GridWorld, point: Point) -> bool {
    point.x >= 0 && point.x < world.width && point.y >= 0 && point.y < world.height
}

fn point_is_unvisited(world: &GridWorld, point: Point) -> bool {
    if world.unvisited.len() == 0 {
        return false
    }

    match world.unvisited.get(&(point.y*world.width + point.x)) {
        Some(_) => return true,
        None => return false,
    }
}

fn distance_at(world: &GridWorld, point: Point) -> f32 {
        world.distances[(point.y*world.width + point.x) as usize]
        
}

fn set_distance_at(world: &mut GridWorld, point: Point, value: f32)  {
    world.distances[(point.y*world.width + point.x) as usize] = value
}


fn reset_world(world: &mut GridWorld){

    for i in 0..world.width*world.height {
        world.distances[i as usize] = INFINITY as f32
    }
    world.current = world.start;
    set_distance_at(world, world.start, 0.0);

    for i in 0..world.height {
        for j in 0..world.width {
            world.unvisited.insert(i*world.width+j, Point{x: j as i32, y: i as i32});
        }
    }

    world.path = vec![];


}

fn reconstruct_path(world: &mut GridWorld){
	if point_is_unvisited(world, world.end) {
		return
	}

    world.current = world.end;
    world.path = vec![];
    world.path.push(world.current);

    while world.current.x != world.start.x || world.current.y != world.start.y {
        let mut next = world.current;
        let mut min_distance = INFINITY as f32;
        for dx in -1..=1 {
            for dy in -1..=1{
                if dx ==0 && dy == 0 {
                    continue;
                }
                if dx != 0 && dy !=0 {
                    continue;
                }
				let neighbour = Point{
					x: world.current.x + dx as i32,
					y: world.current.y + dy as i32,                
                };
				if !point_in_bounds(world, neighbour) { 
					continue
				}
				if world.wall_at(neighbour.x, neighbour.y) { 
					continue
				}
                let dist = distance_at(world, neighbour);
				if dist < min_distance {
					min_distance = dist;
					next = neighbour ;
				}            
            }
        }

        world.current = next; 
        world.path.push(world.current);

    }
}

fn point_in_path(world: &GridWorld, point: Point) -> bool {

    if world.path.len() == 0 {
        return false
    }
    world.path.contains(&point)
}
fn main() {
    let (mut raylb, thread) = raylib::init()
        .size(WINDOWWIDTH, WINDOWHEIGHT)
        .title("PathFinder")
        .build();

    raylb.set_target_fps(60);

    let window = Rectangle::new(0.as_f32(), 0.as_f32(), WINDOWWIDTH.as_f32(), WINDOWHEIGHT.as_f32());

    let margin: f32 = 0.90;
    let grid_rect = center_rect(window, margin, margin).unwrap();

    // Create grid world 
    let size = 20;
    let walls: Vec<bool> = vec![false; size*size];


    let startpoint = Point {x: 0, y:0};
    let mut world = GridWorld {
        width: size as i32,
        height: size as i32,
        start: startpoint,
        end: Point{x: (size-1) as i32, y: (size-1) as i32},
        walls: walls,
        current: startpoint,
        unvisited: HashMap::new(),
        distances: vec![INFINITY as f32; size*size],
        path: vec![],
        
    };

    reset_world(&mut world);

    while !raylb.window_should_close() {
        let mut drawing = raylb.begin_drawing(&thread);
         
        drawing.clear_background(Color::LIGHTGRAY);

        draw_grid(&mut drawing, grid_rect, &mut world).unwrap();

        if drawing.is_key_released(KeyboardKey::KEY_S) ||  drawing.is_key_down(KeyboardKey::KEY_F) {
            step_dijkstra(&mut world);
        }

        if drawing.is_key_released(KeyboardKey::KEY_R) {
            reset_world(&mut world)
        }
        if drawing.is_key_released(KeyboardKey::KEY_P) {
            reconstruct_path(&mut world)
        }
        drawing.draw_fps(5, 5)

        
    }
}