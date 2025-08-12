use crate::action::{Action, ActionType, Properties};
use crate::VDM;

#[test]
fn parse_open() {
    let vdm = VDM::open("src/tests/test.vdm").unwrap();
    assert_eq!(vdm.name, "test");
    assert_eq!(vdm.actions.len(), 2);
    assert_eq!(vdm.actions[1].props().name, "testrate");
}

fn parse() -> VDM {
    VDM::from(include_str!("test.vdm"))
}

#[test]
fn roundtrip() {
    let vdm = parse();

    let as_string = vdm.to_string();
    let back_to_vdm = VDM::from(&as_string);
    let back_to_string = back_to_vdm.to_string();

    assert_eq!(as_string, back_to_string);
}
