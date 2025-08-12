use vdm::action::{Action, ActionType};
use vdm::VDM;

fn construct() -> VDM {
    let mut vdm = VDM::new();

    let mut pause = Action::new(ActionType::Pause);
    pause.props_mut().duration = 5.0;
    pause.props_mut().start_tick = Some(1200);
    vdm.add(pause);

    let mut change_playback_rate = Action::new(ActionType::ChangePlaybackRate);
    change_playback_rate.props_mut().name = String::from("testrate");
    change_playback_rate.props_mut().start_tick = Some(6400);
    change_playback_rate.props_mut().stop_tick = Some(12800);
    change_playback_rate.props_mut().playback_rate = 2.0;
    vdm.add(change_playback_rate);

    vdm
}

fn main() {
    let vdm = construct();
    vdm.export("src/tests/test.vdm");
}
