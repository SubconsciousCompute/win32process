use crossbeam_channel::Sender;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use wmi::{COMLibrary, FilterValue, WMIConnection};

// see https://docs.rs/wmi/latest/wmi/#subscribing-to-event-notifications for further explanation

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceCreationEvent")]
#[serde(rename_all = "PascalCase")]
struct NewProcessEvent {
    target_instance: Process,
}

/// The Win32_Process WMI class represents a process on an operating system.
/// https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Process")]
#[serde(rename_all = "PascalCase")]
pub struct Process {
    /// Numeric identifier used to distinguish one process from another. ProcessIDs are valid from
    /// process creation time to process termination. Upon termination, that same numeric identifier
    /// can be applied to a new process.
    ///
    /// This means that you cannot use ProcessID alone to monitor a particular process. For example,
    /// an application could have a ProcessID of 7, and then fail. When a new process is started,
    /// the new process could be assigned ProcessID 7. A script that checked only for a specified
    /// ProcessID could thus be "fooled" into thinking that the original application was still
    /// running.
    pub process_id: u32,
    /// Unique identifier of the process that creates a process. Process identifier numbers are
    /// reused, so they only identify a process for the lifetime of that process. It is possible
    /// that the process identified by ParentProcessId is terminated, so ParentProcessId may not
    /// refer to a running process. It is also possible that ParentProcessId incorrectly refers to a
    /// process that reuses a process identifier. You can use the CreationDate property to determine
    /// whether the specified parent was created after the process represented by this Win32_Process
    /// instance was created.
    pub parent_process_id: u32,
    /// Name of the executable file responsible for the process, equivalent to the Image Name
    /// property in Task Manager.
    ///
    /// When inherited by a subclass, the property can be overridden to be a key property. The name
    /// is hard-coded into the application itself and is not affected by changing the file name.
    /// For example, even if you rename Calc.exe, the name Calc.exe will still appear in Task
    /// Manager and in any WMI scripts that retrieve the process name.
    pub name: String,
    /// Path to the executable file of the process.
    ///
    /// Example: "C:\Windows\System\Explorer.Exe"
    pub executable_path: Option<String>,
    /// Command line used to start a specific process, if applicable.
    pub command_line: Option<String>, // Command line used to start a specific process, if applicable
}

/// Process space sensor.
pub struct ProcessMonitor {
    tx: crossbeam_channel::Sender<Process>,
}

impl ProcessMonitor {
    pub fn new(tx: Sender<Process>) -> Self {
        Self { tx }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // Before using WMI, a connection must be created.
        let com_con = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::new(com_con).unwrap();
        let mut filters = HashMap::<String, FilterValue>::new();
        filters.insert("TargetInstance".to_owned(), FilterValue::is_a::<Process>()?);
        tracing::info!("WMI connection created.");
        loop {
            for result in wmi_con
                .filtered_notification::<NewProcessEvent>(&filters, Some(Duration::from_secs(1)))?
            {
                let process = result?.target_instance;
                if let Err(e) = self.tx.send(process) {
                    eprintln!("Error sending {e:?}");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(())
    }
}
