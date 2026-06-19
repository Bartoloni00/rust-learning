mod system;

fn main() {
    println!("Hostname: {}", system::get_hostname());
    println!("Kernel: {}", system::get_kernel());
    println!("{}", system::get_ram());
}