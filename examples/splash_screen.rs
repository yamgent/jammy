use bevy::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressSystem};
use jammy::splash_screen::JammySplashScreenPlugin;

const SHORT_TASK_SECS: usize = 1;
const MEDIUM_TASK_SECS: usize = 5;
const LONG_TASK_SECS: usize = 10;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum AppState {
    #[default]
    Loading,
    Done,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(ProgressPlugin::new(AppState::Loading).continue_to(AppState::Done))
        .add_plugin(JammySplashScreenPlugin::new(AppState::Loading))
        .add_system(
            fake_task::<SHORT_TASK_SECS>
                .track_progress()
                .run_if(in_state(AppState::Loading)),
        )
        .add_system(
            fake_task::<MEDIUM_TASK_SECS>
                .track_progress()
                .run_if(in_state(AppState::Loading)),
        )
        .add_system(
            fake_task::<LONG_TASK_SECS>
                .track_progress()
                .run_if(in_state(AppState::Loading)),
        )
        .run();
}

fn fake_task<const TIME: usize>(time: Res<Time>, mut logged: Local<bool>) -> Progress {
    if time.elapsed_seconds() > TIME as f32 {
        if !*logged {
            info!("Task with duration {TIME} complete!");
            *logged = true;
        }
        true.into()
    } else {
        false.into()
    }
}
