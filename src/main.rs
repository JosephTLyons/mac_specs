extern crate os_info;
extern crate os_type;
extern crate sys_info;
extern crate users;

fn print_mac_specs() {
    print_user();
    println!("Hostname: {}", sys_info::hostname().unwrap());
    println!();
    print_operating_system_information();
    println!();
    print_disk_information();
}

fn print_user() {
    use users::{get_user_by_uid, get_current_uid};
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("User: {}", user.name());
}

fn print_operating_system_information() {
    println!("Operating System: {:?}", os_type::current_platform().os_type);
    println!("Operating System Version: {:?}", os_type::current_platform().version);
    println!("Operating System Type: {}", sys_info::os_type().unwrap());
}

fn print_disk_information() {
    let free_space = sys_info::disk_info().unwrap().free;
    let total_space = sys_info::disk_info().unwrap().total;

    //println!("Disk Space Free: {}", bytes_to_gigabytes (free_space));
    //println!("Disk Space Total: {}", bytes_to_gigabytes (total_space));

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