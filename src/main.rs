fn print_mac_specs() {
    print_header("USER");
    print_user_information();
    println!();

    print_header("OS");
    print_operating_system_information();
    println!();

    print_header("CPU");
    print_cpu_information();
    println!();

    print_header("MEMORY");
    print_memory_information();
    println!();

    print_header("DISK");
    print_disk_information();
}

fn print_header(header_text: &str) {
    println!("{}", header_text);
    println!("{}", "-".repeat(header_text.len()));
}

fn print_user_information() {
    use users::{get_current_uid, get_user_by_uid};
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("User: {}", user.name());

    println!("Hostname: {}", sys_info::hostname().unwrap());
}

fn print_operating_system_information() {
    println!(
        "Operating System: {:?}",
        os_type::current_platform().os_type
    );
    println!(
        "Operating System Version: {:?}",
        os_type::current_platform().version
    );
    println!("Operating System Type: {}", sys_info::os_type().unwrap());
}

fn print_cpu_information() {
    println!(
        "CPU Speed: {} GHz",
        sys_info::cpu_speed().unwrap() as f64 / 1000.0
    );
    println!("Number of CPUs: {}", sys_info::cpu_num().unwrap());
}

fn print_memory_information() {
    let free_memory_space = sys_info::mem_info().unwrap().free;
    println!(
        "Memory Free: {:.2} GBs",
        free_memory_space as f64 / (1024.0 * 1024.0)
    );

    let total_memory_space = sys_info::mem_info().unwrap().total;
    println!(
        "Memory Total: {:.2} GBs",
        total_memory_space as f64 / (1024.0 * 1024.0)
    );

    let space_used = (total_memory_space - free_memory_space) as f64 / total_memory_space as f64;
    println!("Memory Used: {:.2}%", space_used * 100.0);
}

fn print_disk_information() {
    let free_disk_space = sys_info::disk_info().unwrap().free;
    println!("Disk Space Free: {}", free_disk_space);

    let total_disk_space = sys_info::disk_info().unwrap().total;
    println!("Disk Space Total: {}", total_disk_space);

    let disk_space_used = (total_disk_space - free_disk_space) as f64 / total_disk_space as f64;
    println!("Disk Space Used: {:.2}%", disk_space_used * 100.0);
}

fn main() {
    println!();
    print_mac_specs();
    println!();
}
