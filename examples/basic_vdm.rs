use vdm::action::ActionType;
use vdm::VDM;

fn main() {
    let mut vdm = VDM::new();

    // create_action() always create the action at the end, we don't need to save it because it's easy to access later.
    let mut props = vdm.create_action(ActionType::SkipAhead).props_mut();

    // Since we used .props_mut() we can directly edit the Action without needing to set anything after.
    // Set is available if you want to completely replace an Action or its properties with .set_nth_props()
    props.name = "Skip 5 seconds in".to_string();
    props.skip_to_time = Some(5.0);

    vdm.export("example.vdm");
}
