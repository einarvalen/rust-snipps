trait Computer {
    fn isGraphicsCardEnabled(&self) -> bool { false }
    fn isBluetoothEnabled(&self) -> bool { false }
    fn hdd(&self) -> String;
    fn ram(&self) -> String;
}

#[derive(Debug)]
struct SmallVm {}
#[derive(Debug)]
struct MediumVm {}
#[derive(Debug)]
struct LargeVm {}

impl Computer for SmallVm {
    fn isGraphicsCardEnabled(&self) -> bool { false }
    fn isBluetoothEnabled(&self) -> bool { false }
    fn hdd(&self) -> String  { String::from("100 GB") }
    fn ram(&self) -> String  { String::from("1 GB") }
}

impl Computer for MediumVm {
    fn isGraphicsCardEnabled(&self) -> bool { false }
    fn isBluetoothEnabled(&self) -> bool { false }
    fn hdd(&self) -> String  {  String::from("500 GB") }
    fn ram(&self) -> String  {  String::from("5 GB") }
}

impl Computer for LargeVm {
    fn isGraphicsCardEnabled(&self) -> bool { false }
    fn isBluetoothEnabled(&self) -> bool { false }
    fn hdd(&self) -> String  {String::from("1000 GB") }
    fn ram(&self) -> String  {String::from("10 GB") }
}

#[derive(Debug)]
struct CustomVm {}
impl Computer for CustomVm {
    fn isGraphicsCardEnabled(&self) -> bool { true }
    fn isBluetoothEnabled(&self) -> bool { true }
    fn hdd(&self) -> String  { String::from("1000 GB") }
    fn ram(&self) -> String  { String::from("10 GB") }
}

fn main() {
	let smallVm: Box<dyn Computer> = Box::new(SmallVm{});
	let mediumVm: Box<dyn Computer> = Box::new(MediumVm{});
	let largeVm: Box<dyn Computer> = Box::new(LargeVm{});
	let customVm: Box<dyn Computer> = Box::new(CustomVm{});
    println!("smallVm: {}", smallVm);
    println!("mediumVm: {}", mediumVm);
    println!("largeVm: {}", largeVm);
    println!("customVm: {}", customVm);
}
