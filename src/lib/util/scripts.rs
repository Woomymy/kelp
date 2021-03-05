use crate::lib::structs::script::Script;
use crate::lib::util::exec::get_root_exec_program;
use kelpdot_macros::debug_print;
use std::process::Command;
pub fn run_script(root: String, script: Script) -> anyhow::Result<()> {
    if let Some(run) = script.elevated {
        if run == true {
            debug_print!("Getting elevator for script {}", script);
            let elevator = get_root_exec_program()?;
            Command::new(elevator) // Use SH because some systems symlinks it to bash / zsh / ash
                .arg("sh")
                .arg(&format!("{}/{}", root, script.path))
                .arg(root)
                .status()?;
        } else {
            run_script(
                root,
                Script {
                    path: script.path,
                    elevated: None,
                },
            )?;
        }
    } else {
        Command::new("sh") // Use SH because some systems symlinks it to bash / zsh / ash
            .arg(&format!("{}/{}", root, script.path))
            .arg(root)
            .status()?;
    }
    Ok(())
}
