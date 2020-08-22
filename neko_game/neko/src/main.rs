use std::path;
use cgmath;
use ggez;
use ggez::event;
use ggez::graphics;
//use ggez::timer;
use ggez::{Context, GameResult};
//use ggez::event::{KeyCode, KeyMods};

type Point2 = cgmath::Point2<f32>;
//type Vector2 = cgmath::Vector2<f32>;

trait UnitActions {

    // OK
    fn draw_unit(&mut self, ctx: &mut Context) -> GameResult;

    // // err:使えていない
    // fn draw_unit_2(&mut self, ctx: &mut Context) -> GameResult;

    // // err:問題が解決できない
    // fn draw(&mut self, ctx: &mut Context, img: &mut graphics::Image, px: u32, py: u32) -> GameResult {
    //     let point2_new = |x, y| {
    //         let cast_u32_f32 = |cp: u32| { cp as f32 };
    //         Point2::new(cast_u32_f32(x), cast_u32_f32(y))
    //     };
    //     graphics::draw(
    //         ctx,
    //         img,
    //         graphics::DrawParam::new()
    //             .dest(point2_new(px, py))
    //     )?;
    //     Ok(())
    // }
}

struct MainUnit {
    image: graphics::Image,
    x: u32,
    y: u32
}

impl UnitActions for MainUnit {
    fn draw_unit(&mut self, ctx: &mut Context) -> GameResult {
        let point2_new = |x, y| {
            let cast_u32_f32 = |cp: u32| { cp as f32 };
            Point2::new(cast_u32_f32(x), cast_u32_f32(y))
        };
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new()
                .dest(point2_new(self.x, self.y))
        )?;
        Ok(())
    }

    // // err:使えていない
    // fn draw_unit_2(&mut self, ctx: &mut Context) -> GameResult {
    //     self.draw(ctx, &mut self.image, self.x, self.y)
    // }
}

struct EnemyUnit {
    image: graphics::Image,
    x: u32,
    y: u32
}

impl UnitActions for EnemyUnit {
    fn draw_unit(&mut self, ctx: &mut Context) -> GameResult {
        let point2_new = |x, y| {
            let cast_u32_f32 = |cp: u32| { cp as f32 };
            Point2::new(cast_u32_f32(x), cast_u32_f32(y))
        };
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new()
                .dest(point2_new(self.x, self.y))
        )?;
        Ok(())
    }

    // // err:使えていない
    // fn draw_unit_2(&mut self, ctx: &mut Context) -> GameResult {
    //     self.draw(ctx, &mut self.image, self.x, self.y)
    // }
}

struct MainState {
    main_unit: MainUnit,
    enemy_units: Vec<EnemyUnit>,
    cnt: u32,
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // MainUnit
        let img = graphics::Image::new(ctx, "/neko.png")?;
        let mu = MainUnit {
            image: img,
            x: 100,
            y: 100,
        };

        // EnemyUnit
        let mut e_units = Vec::new();
        let img1 = graphics::Image::new(ctx, "/unko.png")?;
        let eu1 = EnemyUnit {
            image: img1,
            x: 300,
            y: 300,
        };
        e_units.push(eu1);
        let img2 = graphics::Image::new(ctx, "/unko.png")?;
        let eu2 = EnemyUnit {
            image: img2,
            x: 500,
            y: 400,
        };
        e_units.push(eu2);

        Ok(MainState {
            main_unit: mu,
            enemy_units: e_units,
            cnt: 0,
        })
    }
}

impl event::EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // ネコ描画 MainUnit
        self.main_unit.draw_unit(ctx)?;

        // 💩描画 EnemyUnit
        for u in &mut self.enemy_units {
            u.draw_unit(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    // // key
    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     keycode: KeyCode,
    //     _keymod: KeyMods,
    //     _repeat: bool,
    // ) {
    //     match keycode {
    //         KeyCode::Left => {
    //             //self.vx = -5.0;
    //             //self.vy = 0.0;
    //             self.cnt = self.cnt + 1;
    //         }
    //         KeyCode::Right => {
    //             //self.vx = 5.0;
    //             //self.vy = 0.0;
    //             self.cnt = self.cnt + 2;
    //         }
    //         KeyCode::Space => {
    //             self.cnt = self.cnt + 1;
    //         }
    //     }
    // }

}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
