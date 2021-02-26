use rand::Rng;

const RANGE_FOR_POINTS: u32 = 800;

const UPPER_LEFT_MARGIN : i32 = 20;
const LOWER_RIGHT_MARGIN : i32 = 780;

const TOTAL_COLORS : usize = 6;
const DEFAULT_COLOR_ID: usize = 1;

const MAX_VERTICES : usize = TOTAL_COLORS;
const DEFAULT_VERTICES : usize = 0;

const DEFAULT_JUMP : f32 = 0.5;
const MAX_JUMP_DISTANCE : f32 = 1.0;
const MIN_JUMP_DISTANCE : f32 = 0.0;

const ITERATIONS_PER_UPDATE : usize = 400;

const COLORS: [[f32;4];TOTAL_COLORS] = [
    [0.048,1.0,0.081,0.5],    //green
    [0.061, 0.088, 1.0,0.5],  //blue
    [1.0, 0.050, 0.048,0.5],  //red
    [0.5,0.0,0.5,0.5],        //purple
    [0.8,1.0,0.02,0.5],       //yellow
    [0.2, 0.9, 0.92,0.5]      //cyan
];

#[derive(Clone,Copy)]
pub struct Point{
    pub x : i32,
    pub y : i32,
    pub color : [f32;4]
}

impl Point{
    pub fn new(color : [f32;4])->Point{
        let x = get_position();
        let y = get_position();
        Point{ x, y ,color}
    }
}

fn get_position() -> i32{
    let mut pos : i32 = 0;
    while !is_centered(pos) {
        pos = rand::thread_rng().gen::<i32>()% RANGE_FOR_POINTS as i32;
    }
    pos
}

fn is_centered(pos : i32) -> bool{
    UPPER_LEFT_MARGIN < pos && pos < LOWER_RIGHT_MARGIN
}

fn valid_jump_distance(jump_distance : f32) -> bool{
    MIN_JUMP_DISTANCE < jump_distance && jump_distance < MAX_JUMP_DISTANCE
}

fn get_random_color() -> [f32;4]{
    let color_id = rand::thread_rng().gen::<usize>()%TOTAL_COLORS;
    COLORS[color_id]
}


pub struct ChaosGame{
    points : Vec<Point>,
    vertices : Vec<Point>,
    point : Point,
    previous_selected : usize,
    update_game : bool,
    starting_vertices : usize,
    jump_distance : f32,
    only_one_color : bool,
}


impl ChaosGame{

    pub fn new(mut starting_vertices : usize, mut jump_distance: f32, only_one_color : bool) -> ChaosGame{

        if starting_vertices > MAX_VERTICES{
            starting_vertices = DEFAULT_VERTICES;
        }

        if !valid_jump_distance(jump_distance){
            jump_distance = DEFAULT_JUMP;
        }

        let color = get_random_color();

        let mut game = ChaosGame{
            points : Vec::new(),
            vertices : Vec::new(),
            point : Point::new(color),
            update_game : false,
            previous_selected : 0,
            starting_vertices,
            jump_distance,
            only_one_color
        };

        game.initialize_starting_vertices(color);
        game
    }

    fn initialize_starting_vertices(&mut self, color : [f32;4]) {
        for i in 0..self.starting_vertices {
            let color = if self.only_one_color { color } else { COLORS[i] };
            self.vertices.push(Point::new(color))
        }
    }

    pub fn update(&mut self){
        if !self.can_update() {
            return;
        }
        for _i in 0..ITERATIONS_PER_UPDATE {
            let selected_point = self.get_vertex_to_move();
            self.move_point(selected_point);
        }

    }

    fn can_update(&self) -> bool{
        self.update_game && !self.vertices.is_empty()
    }

    fn move_point(&mut self, selected_point: Point) {
        self.point.x = (((self.point.x + selected_point.x) as f32) * self.jump_distance) as i32;
        self.point.y = (((self.point.y + selected_point.y) as f32) * self.jump_distance) as i32;
        self.point.color = selected_point.color;
        self.points.push(self.point);
    }

    fn get_vertex_to_move(&mut self) -> Point {
        let mut point_to_select = self.get_vertex();

        while !self.can_select_previous_vertex(point_to_select) {
            point_to_select = self.get_vertex();
        }

        let selected_point = self.vertices[point_to_select];

        self.previous_selected = point_to_select;

        selected_point
    }

    fn get_vertex(&self) -> usize{
        rand::thread_rng().gen::<usize>() % self.vertices.len()
    }

    fn can_select_previous_vertex(&self, point_to_select : usize) -> bool{
        self.vertices.len() <= 3 || self.previous_selected != point_to_select
    }

    pub fn get_points(&self) -> Vec<Point>{
        let mut points = self.points.clone();
        let mut vertices = self.vertices.clone();
        points.append(&mut vertices);
        points
    }

    pub fn add_vertex(&mut self, mouse_pos: &mut (f64, f64)) {
        if self.vertices.len() < TOTAL_COLORS {
            let (x, y) = mouse_pos;
            let color_id = if self.only_one_color { DEFAULT_COLOR_ID } else { self.vertices.len() };
            self.vertices.push(Point {
                x: *x as i32,
                y: *y as i32,
                color: COLORS[color_id]
            });
        }
    }

    pub fn pause_game(&mut self) {
        self.update_game = false;
    }

    pub fn start_game(&mut self) {
        self.update_game = true;
    }

    pub fn reset(&mut self){
        self.points = Vec::new();
        self.vertices = Vec::new();

        let color = get_random_color();
        self.point = Point::new(color);
        self.initialize_starting_vertices(color);
    }

}