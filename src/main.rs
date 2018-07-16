extern crate os_info;
extern crate sys_info;
extern crate users;

fn print_mac_specs() {
    //println!("Username: {}", username::)
    println!("Hostname: {}", sys_info::hostname().unwrap());
    print_operating_system_information();
    print_disk_information();
}

fn print_operating_system_information() {
    println!("Operating System Type: {}", sys_info::os_type().unwrap());
    println!("Operating System: {}", sys_info::os_release().unwrap());
}

fn print_disk_information() {
    let free_space = sys_info::disk_info().unwrap().free;
    let total_space = sys_info::disk_info().unwrap().total;

    // println!("{}", sys_info::disk_info().unwrap().free);
    //
    // println!("Disk Space Free: {}", bytes_to_gigabytes (free_space));
    // println!("Disk Space Total: {}", bytes_to_gigabytes (total_space));

    let space_used = (total_space - free_space) as f64 / total_space as f64;
    println!("Disk Space Used: {:.2}%", space_used * 100 as f64);
}

fn bytes_to_gigabytes (bytes: u64) -> u64 {
    return bytes / (1024 * 1024 * 8);
}

fn main() {
    println!();
    print_mac_specs();
    println!();
}
