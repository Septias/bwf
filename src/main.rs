#![feature(drain_filter)]

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{
        palette::Srgba, rendy::hal::command::ClearColor, types::DefaultBackend, RenderDebugLines,
        RenderShaded3D, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    Application,
};
use config::BwfConfig;
use std::time::Duration;
use world::FwdWorld;

mod bwf;
mod config;
mod world;
mod tiles;

use crate::bwf::Bwf;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // prepare data for GameDataBuilder
    let app_root = application_root_dir()?;
    let display_config_dir = app_root.join("config/display.ron");
    let input_dir = app_root.join("config").join("input.ron");
    let assets_dir = app_root.join("assets/");
    let bwf_config = BwfConfig::load("config/config.ron")?;

    // create game_data with GameDatabuilder
    let mut game_data = DispatcherBuilder::default();
    game_data.add_bundle(InputBundle::new().with_bindings_from_file(&input_dir)?)
        .add_bundle(TransformBundle::default())
        //.with_bundle(MinionsBundle)?
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                //.with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_dir)?.with_clear(ClearColor {
                        float32: [0.1, 0.1, 0.1, 1.0],
                    }),
                )
                .with_plugin(RenderShaded3D::default()),
        );
    let game = Application::build(assets_dir, Bwf::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .with_resource(bwf_config.arena)
        .with_resource(bwf_config.camera)
        .build(game_data)?;

    game.run();
    Ok(())
}
