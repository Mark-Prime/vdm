use vdm::VDM;

fn main() {
    let vdm = VDM::open("example.vdm").unwrap();

    println!("The vdm you opened is {}.vdm", vdm.name);
}
