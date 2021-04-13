use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::ImageError;
use tinybit::events::{events, Event, KeyCode, KeyEvent, EventModel};
use tinybit::{term_size, Pixel, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, Color};

struct Block {
    left: Pixel,
    right: Pixel,
}

impl Block {
    pub fn new(pos: ScreenPos, color: Option<Color>) -> Self {
        let left = Pixel::new(' ', pos, None, color);
        let right = Pixel {
            pos: ScreenPos::new(left.pos.x + 1, left.pos.y),
            ..left
        };

        Self { left, right }
    }
}

fn load_image(path: &str) -> Result<Vec<Block>, ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    let mut blocks = Vec::new();
    for (x, y, pixel) in img.pixels() {
        if pixel[3] == 0 {
            continue
        }
        let color = Color::Rgb {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        };
        blocks.push(
            Block::new(ScreenPos::new(x as u16 * 2, y as u16), Some(color))
        );
    }
    Ok(blocks)
}

fn main() {
    let filepath = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("No file selected");
            std::process::exit(1);
        }
    };

    let (width, height) = term_size().unwrap();

    let mut viewport = Viewport::new(ScreenPos::new(1, 1), ScreenSize::new(width, height));
    let stdout = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(stdout);

    let blocks = load_image(&filepath).unwrap();

    for event in events(EventModel::Fps(1)) {
        match event {
            Event::Tick => {
                blocks.iter().for_each(|block| {
                    viewport.draw_pixel(block.left);
                    viewport.draw_pixel(block.right);
                });
                renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                return;
            }
            _ => {}
        }
    }
}
