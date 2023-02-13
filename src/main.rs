use win32process::ProcessSensor;

impl Process {
    fn clean(&mut self) {
        if self.command_line.is_some() && self.executable_path.is_some() {
            let mut len = 0;
            if self.command_line.as_ref().unwrap().starts_with('"') {
                len += 3;
            }
            self.command_line
                .as_mut()
                .unwrap()
                .replace_range(0..(self.executable_path.as_ref().unwrap().len() + len), "");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = crossbeam_channel::bounded(10);
    let sensor = ProcessSensor::new(tx);

    for process in rx.recv() {
        println!("{:#?}", process);
    }
    Ok(())

}
