// A simple rust console program that takes a physical drive as input and checks if the drive has been wiped either by completely overwriting the drive or by using a secure erase method.
// the program will then output the results to the console with wiped and the possible method used.

use core::fmt;
use itertools::Itertools;
use std::{io::Read, process};

use gptman::GPT;

use sysinfo::{DiskExt, System, SystemExt};
use windows_drives::PhysicalDrive;

fn main() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display the number of the disk, the name and the used space, and the total space, and the filesystem type.
    for (disk_index, disk) in sys.disks().iter().enumerate() {
        let used_space = (disk.total_space() - disk.available_space()) / 1000000000;
        let file_system = std::str::from_utf8(&disk.file_system()).expect("invalid utf-8 sequence");
        println!("");
        println!("");
        println!("____________________________________________________");
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!(
            "Disk #{}:, Name: {:?}, Drive letter: {:?}",
            disk_index,
            disk.name(),
            disk.mount_point()
        );
        println!("total space: {} GB", disk.total_space() / 1000000000);
        println!("used space: {} GB", used_space);
        println!("filesystem: {:?}", file_system);
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("____________________________________________________");
    }
    println!("");
    println!("");
    // Get the user to select the disk they would like to check
    println!("Please select the disk you would like to check");
    println!("Enter the number of the disk you would like to check");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u8 = input.trim().parse().expect("Please type a number!");
    println!("You have selected disk #{}", input);
    println!(
        "Checking disk #{} {:?}",
        input,
        sys.disks().get(input as usize).unwrap().name()
    );
    //Compare the first two bytes of the disk to see if they are 0x00
    let stringinput = input.to_string();
    let physical_disk_name = [r"\\.\PhysicalDrive", &stringinput].iter().join("");
    // get total number of sectors on disk
    let mut f = std::fs::File::open(physical_disk_name).expect("could not open disk");
    let gpt = gptman::GPT::find_from(&mut physical_disk_name).expect("could not find GPT");

    println!("Disk GUID: {:?}", gpt.header.disk_guid);

    for (i, p) in gpt.iter() {
        if p.is_used() {
            println!(
                "Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
                i,
                p.partition_type_guid,
                p.size().unwrap() * gpt.sector_size,
                p.starting_lba
            );
        }
    }

    let total_sectors: u64 = sys.disks().get(input as usize).unwrap().total_space() / 512;
    let mut buffer = vec![0; total_sectors as usize];
    let mut physical_disk = PhysicalDrive::open(input).unwrap();
    physical_disk.read_exact(&mut buffer).unwrap();
    println!("First 512 bytes of disk: {:x?}", buffer.iter());
    // if any bytes are not 0x00 then the disk has not been wiped
    if buffer.iter().any(|&x| x != 0) {
        println!("Disk has not been wiped");
    } else {
        println!("Disk has been wiped");
    }
}
