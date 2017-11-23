extern crate ggez;

use ggez::audio;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;
use std::env;
use std::path;

struct MainState {
    sound: audio::Source,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();

        let sound = audio::Source::new(ctx, "/sound.ogg").unwrap();

        let _ = sound.play();

        let s = MainState {
            // BUGGO: We never use sound again,
            // but we have to hang on to it, Or Else!
            // The optimizer will decide we don't need it
            // since play() has "no side effects" and free it.
            // Or something.
            sound: sound,
        };



        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }
}

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
pub fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);
    let ctx = &mut Context::load_from_conf("ggez-soundtest", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding manifest dir {:?}", path);
        ctx.filesystem.mount(&path, true);
    } else {
        panic!("No manifest dir???");
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
