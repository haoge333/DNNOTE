// Release 模式下使用 Windows 窗口子系统，避免启动时出现黑色控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_data_file,
            write_data_file,
            get_data_path,
            read_fixed_file,
            write_fixed_file,
            get_fixed_path,
            set_autostart,
            get_autostart
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 获取数据文件路径（exe 同目录下的 data.json）
#[tauri::command]
fn get_data_path() -> String {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let data_path = dir.join("龙之谷数据.json");
    data_path.to_string_lossy().to_string()
}

// 读取数据文件
#[tauri::command]
fn read_data_file() -> Result<String, String> {
    let path = get_data_path();
    if !std::path::Path::new(&path).exists() {
        return Err("文件不存在".to_string());
    }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

// 写入数据文件
#[tauri::command]
fn write_data_file(content: String) -> Result<(), String> {
    let path = get_data_path();
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

// 获取固定数据文件路径（exe 同目录下的 龙之谷固定数据.json）
#[tauri::command]
fn get_fixed_path() -> String {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
    dir.join("龙之谷固定数据.json")
        .to_string_lossy()
        .to_string()
}

// 读取固定数据文件（职业表等游戏通用配置）
#[tauri::command]
fn read_fixed_file() -> Result<String, String> {
    let path = get_fixed_path();
    if !std::path::Path::new(&path).exists() {
        return Err("文件不存在".to_string());
    }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

// 写入固定数据文件
#[tauri::command]
fn write_fixed_file(content: String) -> Result<(), String> {
    let path = get_fixed_path();
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

// 设置/取消开机自启动（写入当前用户的 Run 注册表项）
#[tauri::command]
fn set_autostart(enabled: bool) -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = exe_path.to_string_lossy().to_string();
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    let name = "龙之谷小助手";
    let mut cmd = std::process::Command::new("reg");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW，避免弹出控制台窗口
    }
    if enabled {
        let value = format!("\"{}\"", exe);
        let out = cmd
            .args(["add", key, "/v", name, "/t", "REG_SZ", "/d", &value, "/f"])
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }
    } else {
        // 删除不存在的值时 reg 返回非 0，这里视为成功
        let _ = cmd.args(["delete", key, "/v", name, "/f"]).output();
    }
    Ok(())
}

// 查询当前程序是否已注册为开机自启动（路径需与当前 exe 一致）
#[tauri::command]
fn get_autostart() -> Result<bool, String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = exe_path.to_string_lossy().to_string();
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    let name = "龙之谷小助手";
    let mut cmd = std::process::Command::new("reg");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    let out = cmd
        .args(["query", key, "/v", name])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Ok(false);
    }
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    Ok(text.contains(&format!("\"{}\"", exe)))
}
