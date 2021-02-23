use rand::Rng;

const RANGE_FOR_POINTS: u32 = 800;

const UPPER_LEFT_MARGIN : i32 = 20;
const LOWER_RIGHT_MARGIN : i32 = 780;

const TOTAL_COLORS : usize = 6;
const DEFAULT_VERTICES : usize = 0;

const COLORS: [[f32;4];TOTAL_COLORS] = [
    [0.048,1.0,0.081,1.0],    //green
    [0.061, 0.088, 1.0,1.0],  //blue
    [1.0, 0.050, 0.048,1.0],  //red
    [0.5,0.0,0.5,1.0],        //purple
    [0.8,1.0,0.02,1.0],       //yellow
    [0.2, 0.9, 0.92,1.0]      //cyan
];

#[derive(Clone,Copy)]
pub struct Point{
    pub x : i32,
    pub y : i32,
    pub color : [f32;4]
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
        pos = rand::thread_rng().gen::<i32>()% RANGE_FOR_POINTS as i32;
    }
    pos
}

fn is_centered(pos : i32) -> bool{
    UPPER_LEFT_MARGIN < pos && pos < LOWER_RIGHT_MARGIN
}

pub struct ChaosGame{
    points : Vec<Point>,
    vertices : Vec<Point>,
    point : Point,
    previous_selected : usize,
    pub update_game : bool,
}


impl ChaosGame{

    pub fn new(mut starting_vertices : usize, only_one_color : bool) -> ChaosGame{

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
            color = if only_one_color { color } else { COLORS[i] };
            game.vertices.push(Point::new(color))
        }
        game
    }

    pub fn update(&mut self){
        if !self.update_game || self.vertices.is_empty() {
            return;
        }

        let selected_point = self.get_vertex_to_move();

        self.move_point(selected_point);

    }

    fn move_point(&mut self, selected_point: Point) {
        self.point.x = (self.point.x + selected_point.x) / 2;
        self.point.y = (self.point.y + selected_point.y) / 2;
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
            let color_id = self.vertices.len();
            self.vertices.push(Point {
                x: *x as i32,
                y: *y as i32,
                color: COLORS[color_id]
            });
        }
    }
}