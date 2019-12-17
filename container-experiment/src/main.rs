use num_cpus;
#[cfg(target_os = "linux")]
use procinfo;

fn main() {
    println!("Num logical cpus: {}", num_cpus::get());
    println!("Num physical cpus: {}", num_cpus::get_physical());

    #[cfg(target_os = "linux")]
    match procinfo::pid::statm_self() {
        Ok(memstats) => {
            println!("Total virtual memory size: {}", memstats.size);
            println!("Resident non-swapped memory: {}", memstats.resident);
            println!("Shared memory: {}", memstats.share);
            println!("Resident executable memory: {}", memstats.text);
            println!("Resident data and stack memory: {}", memstats.data);
        }
        Err(err) => eprintln!("Error fetching memory stats: {:?}", err),
    };
}
