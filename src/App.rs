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

    pub fn new(starting_vertices : usize, jump_distance : f32, only_one_color : bool)->App{
        App{
            window: WindowSettings::new("Chaos Game", [WIDTH, HEIGHT])
                .exit_on_esc(true).resizable(false).build().unwrap(),
            chaos_game: ChaosGame::new(starting_vertices,jump_distance,only_one_color)
        }
    }

    fn draw(&mut self,event:Event) {

        let points = self.chaos_game.get_points();

        self.window.draw_2d(&event, |context, graphics, _device| {

            clear([0.0;4], graphics);

            for point in points {
                rectangle(point.color, [point.x as f64,point.y as f64,1.0,1.0], context.transform, graphics);
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
                    self.chaos_game.start_game();
                }
                else if button == Key::P {
                    self.chaos_game.pause_game();
                }
                else if button == Key::R {
                    self.chaos_game.reset();
                }
            }
            _ => {}
        }
    }

}