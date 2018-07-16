extern crate os_info;
extern crate os_type;
extern crate sys_info;
extern crate users;

fn print_mac_specs() {
    print_user_information();
    println!();
    print_operating_system_information();
    println!();
    print_disk_information();
    println!();
    print_memory_information();
}

fn print_user_information() {
    use users::{get_user_by_uid, get_current_uid};
    let user = get_user_by_uid (get_current_uid()).unwrap();
    println!("User: {}", user.name());

    println!("Hostname: {}", sys_info::hostname().unwrap());
}

fn print_operating_system_information() {
    println!("Operating System: {:?}", os_type::current_platform().os_type);
    println!("Operating System Version: {:?}", os_type::current_platform().version);
    println!("Operating System Type: {}", sys_info::os_type().unwrap());
}

fn print_disk_information() {
    let free_disk_space = sys_info::disk_info().unwrap().free;
    let total_disk_space = sys_info::disk_info().unwrap().total;

    //println!("Disk Space Free: {}", bytes_to_gigabytes (free_disk_space));
    //println!("Disk Space Total: {}", bytes_to_gigabytes (total_disk_space));

    let disk_space_used = (total_disk_space - free_disk_space) as f64 / total_disk_space as f64;
    println!("Disk Space Used: {:.2}%", disk_space_used * 100 as f64);
}

fn print_memory_information() {
    let free_memory_space = sys_info::mem_info().unwrap().free;
    let total_memory_space = sys_info::mem_info().unwrap().total;

    println!("Memory Free: {}", free_memory_space);
    println!("Memory Total: {}", total_memory_space);

    let space_used = (total_memory_space - free_memory_space) as f64 / total_memory_space as f64;
    println!("Memory Used: {:.2}%", space_used * 100 as f64);
}

fn bytes_to_gigabytes (bytes: u64) -> u64 {
    return bytes / (1024 * 1024 * 8);
}

fn main() {
    println!();
    print_mac_specs();
    println!();
}
