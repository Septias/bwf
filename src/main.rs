use amethyst::{Application, core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle}, input::InputBundle, prelude::*, renderer::{RenderDebugLines, RenderShaded3D, RenderToWindow, RenderingBundle, palette::Srgba, rendy::hal::command::ClearColor, types::DefaultBackend}, utils::application_root_dir};
use std::time::Duration;

mod world;
mod bwf;
mod config;

use crate::bwf::Bwf;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // prepare data for GameDataBuilder
    let app_root = application_root_dir()?;
    let display_config_dir = app_root.join("config/display.ron");
    let input_dir = app_root.join("config").join("input.ron");
    let assets_dir = app_root.join("assets/");
    //let minions_config = MinionsConfig::load("config/config.ron")?;

    // create game_data with GameDatabuilder
    let game_data = DispatcherBuilder::default()
        .add_bundle(InputBundle::new().with_bindings_from_file(&input_dir)?)
        .add_bundle(TransformBundle::default())
        //.with_bundle(MinionsBundle)?
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                //.with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_dir)?
                        .with_clear(ClearColor{float32: [0.1, 0.1, 0.1, 1.0]}),
                )
                .with_plugin(RenderShaded3D::default()),
        );
    let mut game = Application::build(assets_dir, Bwf::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        //.with_resource(minions_config.arena)
        //.with_resource(minions_config.camera)
        .build(game_data)?;
    game.run();
    Ok(())
}