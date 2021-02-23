extern crate piston_window;
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::*;
use crate::chaos_game::ChaosGame;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;


pub struct App{
    window : PistonWindow,
    chaos_game : ChaosGame
}

impl App{

    pub fn new(starting_vertices : usize, only_one_color : bool)->App{
        App{
            window: WindowSettings::new("Chaos Game", [WIDTH, HEIGHT])
                .exit_on_esc(true).resizable(false).build().unwrap(),
            chaos_game: ChaosGame::new(starting_vertices,only_one_color)
        }
    }

    fn draw(&mut self,event:Event) {

        let points = self.chaos_game.get_points();

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
        events.set_ups(60);
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
                self.chaos_game.add_vertex(mouse_pos)
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

}