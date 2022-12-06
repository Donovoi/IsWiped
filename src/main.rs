// A simple rust console program that takes a physical drive as input and checks if the drive has been wiped either by completely overwriting the drive or by using a secure erase method.
// the program will then output the results to the console with wiped and the possible method used.

use sysinfo::{System, SystemExt, DiskExt};

fn main() {

let mut sys = System::new_all();

// First we update all information of our `System` struct.
sys.refresh_all();

// We display the number of the disk, the name and the used space, and the total space, and the filesystem type.
for (disk_index, disk) in sys.disks().iter().enumerate() {
    let used_space = (disk.total_space() - disk.available_space()) / 1000000000;
    let file_system = std::str::from_utf8(&disk.file_system()).expect("invalid utf-8 sequence");
    println!("Disk #{}:, Name: {:?}, Drive letter: {:?}", disk_index, disk.name(), disk.mount_point());
    println!("total space: {} GB",  disk.total_space()/1000000000);
    println!("used space: {} GB", used_space);
    println!("filesystem: {:?}", file_system);
    println!("");
    println!("");
    // Get the user to select the disk they would like to check
    println!("Please select the disk you would like to check");
    println!("Enter the number of the disk you would like to check");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    println!("You have selected disk #{}", input);
    println!("Checking disk #{}", input);
return;
}

}