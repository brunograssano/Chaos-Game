extern crate piston_window;
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::*;
use rand::Rng;
use std::f64::consts::PI;
use self::piston_window::ellipse::circle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const UPPER_LEFT_MARGIN : i32 = 20;
const LOWER_RIGHT_MARGIN : i32 = 780;

const TOTAL_COLORS : usize = 5;
const DEFAULT_VERTICES : usize = 3;

const COLORS: [[f32;4];TOTAL_COLORS] = [
    [0.048,1.0,0.081,1.0],    //green
    [0.061, 0.088, 1.0,1.0],  //blue
    [1.0, 0.050, 0.048,1.0],  //red
    [0.5,0.0,0.5,1.0],        //purple
    [0.8,1.0,0.02,1.0]        //yellow
];

#[derive(Clone,Copy)]
struct Point{
    x : i32,
    y : i32,
    color : [f32;4]
}

impl Point{
    pub fn new(color : [f32;4])->Point{
        let x = get_number();
        let y = get_number();
        Point{ x, y ,color}
    }
}

fn get_number() -> i32{
    let mut pos : i32 = 0;
    while !is_centered(pos) {
        pos = rand::thread_rng().gen::<i32>()% WIDTH as i32;
    }
    pos
}
fn is_centered(pos : i32) -> bool{
    UPPER_LEFT_MARGIN < pos && pos < LOWER_RIGHT_MARGIN
}
struct ChaosGame{
    points : Vec<Point>,
    vertices : Vec<Point>,
    point : Point,
    update_game : bool,
    previous_selected : usize,
}


impl ChaosGame{
    pub fn new(mut starting_vertices : usize) -> ChaosGame{
        if starting_vertices > TOTAL_COLORS{
            starting_vertices = DEFAULT_VERTICES;
        }
        let mut color = COLORS[rand::thread_rng().gen::<usize>()%TOTAL_COLORS];
        let mut game = ChaosGame{
            points : Vec::new(),
            vertices : Vec::new(),
            point : Point::new(color),
            update_game : false,
            previous_selected : 0
        };
        for i in 0..starting_vertices {
            color = COLORS[i];
            game.vertices.push(Point::new(color))
        }
        game
    }

    pub fn update(&mut self){
        if !self.update_game {
            return;
        }
        let mut point_to_select = self.get_vertex();
        while self.vertices.len() > DEFAULT_VERTICES && self.previous_selected == point_to_select{
            point_to_select = self.get_vertex();
        }

        let selected_point = self.vertices[point_to_select];

        self.point.x = (self.point.x + selected_point.x) / 2;
        self.point.y = (self.point.y + selected_point.y) / 2;
        self.point.color = selected_point.color;
        self.points.push(self.point);

        self.previous_selected = point_to_select;
    }

    fn get_vertex(&self) -> usize{
        rand::thread_rng().gen::<usize>() % self.vertices.len()
    }
}

pub struct App{
    window : PistonWindow,
    chaos_game : ChaosGame
}

impl App{

    pub fn new(starting_vertices : usize)->App{
        App{
            window: WindowSettings::new("Chaos Game", [WIDTH, HEIGHT])
                .exit_on_esc(true).resizable(false).build().unwrap(),
            chaos_game: ChaosGame::new(starting_vertices)
        }
    }

    fn draw(&mut self,event:Event) {

        let mut points = self.chaos_game.points.clone();
        let mut vertices = self.chaos_game.vertices.clone();
        points.append(&mut vertices);
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0;4], graphics);
            for point in points {
                rectangle(point.color, [point.x as f64,point.y as f64,2.0,2.0], context.transform, graphics);
            }

        });
    }

    pub fn game_loop(&mut self) {
        let mut mouse_pos = (0.0, 0.0);
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(_r) = e.render_args() {
                self.draw(e);
            } else if let Some(_u) = e.update_args() {
                self.chaos_game.update();
            } else if let Some(button) = e.press_args() {
                self.manage_buttons(&mut mouse_pos, button);
            } else if let Some(cursor) = e.mouse_cursor_args() {
                mouse_pos = (cursor[0], cursor[1]);
            }
        }
    }

    fn manage_buttons(&mut self, mouse_pos: &mut (f64, f64), button: Button) {
        match button {
            Button::Mouse(_button) => {
                self.add_vertex(mouse_pos)
            }
            Button::Keyboard(button)=> {
                if button == Key::S {
                    self.start_game();
                }
                else if button == Key::P {
                    self.pause_game();
                }
            }
            _ => {}
        }
    }

    fn pause_game(&mut self) {
        self.chaos_game.update_game = false;
    }

    fn start_game(&mut self) {
        self.chaos_game.update_game = true;
    }

    fn add_vertex(&mut self, mouse_pos: &mut (f64, f64)) {
        if self.chaos_game.vertices.len() < TOTAL_COLORS {
            let (x, y) = mouse_pos;
            let color_id = self.chaos_game.vertices.len();
            self.chaos_game.vertices.push(Point {
                x: *x as i32,
                y: *y as i32,
                color: COLORS[color_id]
            });
        }
    }
}