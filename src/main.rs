use win32process::ProcessMonitor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = crossbeam_channel::bounded(10);

    let mut sensor = ProcessMonitor::new(tx);
    std::thread::spawn(move || {
        sensor.run().expect("unable to run sensor");
    });

    loop {
        if let Ok(process) = rx.try_recv() {
            println!("{:#?}", process);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
