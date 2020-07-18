use cgmath;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use std::env;
use std::path;

type Point2 = cgmath::Point2<f32>;
type Vector2 = cgmath::Vector2<f32>;

struct IZState {
    image1: graphics::Image,
    text: String,
    cnt: usize,
}

impl IZState {
    fn new(ctx: &mut Context) -> GameResult<IZState> {
        let image1 = graphics::Image::new(ctx, "/z.png")?;
        Ok(IZState {
            image1,
            text: "Rust".to_string(),
            cnt: 0,
        })
    }
}

// いそいで合体
mod tools {
    pub fn get_len(st: &str) -> usize {
        st.len()
    }

    pub fn get_char(st: &str, pos: usize) -> String {
        let mut str1 = "".to_string();
        for (i, c) in st.chars().enumerate() {
            if i == pos {
                str1.push(c);
            }
        }
        println!("{}", str1);
        str1
    }
}

impl event::EventHandler for IZState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 6;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.cnt = self.cnt + 1;
            if (self.cnt > self.text.len()) {
                self.cnt = 0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw an image.
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        // Finished drawing, show it all on the screen!

        graphics::draw(
            ctx,
            &self.image1,
            graphics::DrawParam::new()
                //.dest(Point2::new(10.0, 10.0))
                //.rotation(self.rotation), //.offset(Point2::new(0.5, 0.5)),
                .scale(Vector2::new(0.65, 0.65)),
        )?;

        // text:
        // let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        // let text = graphics::Text::new(("Hello Neko World!", font, 48.0));
        // let dest_point = cgmath::Point2::new(1.0, 1.0);
        // //        graphics::draw(ctx, &text, (dest_point,))?;
        // graphics::draw(ctx, &text, (dest_point, 0.0, graphics::BLACK))?;

        // let text = self.text.to_string();       // NG！！！
        let text = "Rust";
        //let size = tools::get_len(text);
        let mut y = 160.0;
        for i in 0..(self.cnt) {
            let mut c = tools::get_char(text, i);
            //println!("pos= {}", c);
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
            let text = graphics::Text::new((c, font, 48.0));
            let dest_point = cgmath::Point2::new(122.0, y);
            //        graphics::draw(ctx, &text, (dest_point,))?;
            graphics::draw(ctx, &text, (dest_point, 0.0, graphics::BLACK))?;

            y = y + 40.0;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut IZState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
