use std::collections::HashMap;
use std::time::Duration;
use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection};

// see https://docs.rs/wmi/latest/wmi/#subscribing-to-event-notifications for further explanation

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceCreationEvent")]
#[serde(rename_all = "PascalCase")]
struct NewProcessEvent {
    target_instance: Process
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Process")]
#[serde(rename_all = "PascalCase")]
struct Process {
    process_id: u32,
    parent_process_id: u32,
    name: String,
    executable_path: Option<String>,
    command_line: Option<String> // Command line used to start a specific process, if applicable
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;

    let mut filters = HashMap::<String, FilterValue>::new();

    filters.insert("TargetInstance".to_owned(), FilterValue::is_a::<Process>()?);

    let iterator = wmi_con.filtered_notification::<NewProcessEvent>(&filters, Some(Duration::from_secs(1)))?;

    for result in iterator {
        let process = result?.target_instance;
        println!("{:#?}", process);
    } // Loop will end only on error

    Ok(())
}
