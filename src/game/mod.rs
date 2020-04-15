mod input;
use input as gameInput;

use coffee::graphics::{Color, Frame, Image, Point, Rectangle, Sprite, Window, WindowSettings};
use coffee::input::keyboard;
use coffee::load::Task;
use coffee::{Game, Result, Timer};

struct DeckStats {
    width: u16,
    height: u16,
    x: f32,
    y: f32,
    acceleration: f32,
}

pub struct GameStore {
    palette: Image,
    deck: DeckStats,
}

impl Game for GameStore {
    type Input = gameInput::InputListener;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<GameStore> {
        return Task::using_gpu(|gpu| Image::from_colors(gpu, &[Color::from_rgb(150, 50, 50)]))
            .map(|palette| GameStore {
                palette,
                deck: DeckStats {
                    height: 100,
                    width: 10,
                    x: 0.0,
                    y: 0.0,
                    acceleration: 0.0,
                },
            });
    }

    fn interact(&mut self, input: &mut input::InputListener, window: &mut Window) {
        let pressed = input.pressed_keys.len();
        if pressed == 0 || pressed > 1 {
            return self.deck.acceleration = 0.0;
        } else if input.pressed_keys.contains(&keyboard::KeyCode::Up) {
            let next = self.deck.y - 2.0 - self.deck.acceleration;
            self.deck.y = if next < 0.0 { 0.0 } else { next };
        } else if input.pressed_keys.contains(&keyboard::KeyCode::Down) {
            let max = window.height() - self.deck.height as f32;
            let next = self.deck.y + 2.0 + self.deck.acceleration;
            self.deck.y = if next > max { max } else { next };
        }
        self.deck.acceleration += 0.5;
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::from_rgb(150, 150, 150));

        self.palette.draw(
            Sprite {
                source: Rectangle {
                    x: 0,
                    y: 0,
                    width: self.deck.width,
                    height: self.deck.height,
                },
                position: Point::new(self.deck.x, self.deck.y),
                scale: (1.0, 1.0),
            },
            &mut frame.as_target(),
        );
    }
}

pub fn init() -> Result<()> {
    GameStore::run(WindowSettings {
        title: String::from("Ping Pong"),
        size: (640, 480),
        resizable: true,
        fullscreen: false,
    })
}
