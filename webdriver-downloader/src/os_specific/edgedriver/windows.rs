use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "msedgedriver.exe";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    // Check common Edge installation paths on Windows
    let program_files = std::env::var("ProgramFiles")?;
    let program_files_x86 = std::env::var("ProgramFiles(x86)")
        .unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());
    
    eprintln!("Searching for Edge in Program Files: {}", program_files);
    eprintln!("Searching for Edge in Program Files (x86): {}", program_files_x86);

    let possible_paths = [
        // Stable channel paths
        format!("{program_files}\\Microsoft\\Edge\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge\\Application\\msedge.exe"),
        // Beta channel paths
        format!("{program_files}\\Microsoft\\Edge Beta\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge Beta\\Application\\msedge.exe"),
        // Dev channel paths
        format!("{program_files}\\Microsoft\\Edge Dev\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge Dev\\Application\\msedge.exe"),
        // Canary channel paths (per-user installation)
        format!(
            "{}\\Microsoft\\Edge SxS\\Application\\msedge.exe",
            std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string())
        ),
    ];

    for path in &possible_paths {
        let path_buf = PathBuf::from(path);
        eprintln!("Checking Edge path: {}", path);
        if path_buf.exists() {
            eprintln!("Found Edge at: {}", path);
            return Ok(path_buf);
        }
    }

    // Default to most common stable path if none exist (will be caught during verification)
    Ok(PathBuf::from(format!(
        "{program_files}\\Microsoft\\Edge\\Application\\msedge.exe"
    )))
}
