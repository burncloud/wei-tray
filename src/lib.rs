use systray::Application;
use std::io::Write;

fn create_default_ico(ico_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // 确保目录存在
    if let Some(parent) = ico_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 使用一个极简但有效的ICO文件格式
    // 这是一个16x16像素的单色ICO文件
    let ico_data = vec![
        // ICO Header (6 bytes)
        0x00, 0x00,       // Reserved
        0x01, 0x00,       // Type (1 = ICO)
        0x01, 0x00,       // Count (1 image)

        // Image Directory Entry (16 bytes)
        0x10,             // Width (16 pixels)
        0x10,             // Height (16 pixels)
        0x02,             // Color count (2 colors)
        0x00,             // Reserved
        0x01, 0x00,       // Color planes
        0x01, 0x00,       // Bits per pixel (1 bit)
        0x30, 0x00, 0x00, 0x00,  // Size of bitmap data (48 bytes)
        0x16, 0x00, 0x00, 0x00,  // Offset to bitmap data (22 bytes)

        // DIB Header (40 bytes)
        0x28, 0x00, 0x00, 0x00,  // DIB header size (40 bytes)
        0x10, 0x00, 0x00, 0x00,  // Width (16 pixels)
        0x20, 0x00, 0x00, 0x00,  // Height (32 pixels, includes AND mask)
        0x01, 0x00,              // Planes (1)
        0x01, 0x00,              // Bits per pixel (1)
        0x00, 0x00, 0x00, 0x00,  // Compression (0 = none)
        0x00, 0x00, 0x00, 0x00,  // Image size (0 = auto)
        0x00, 0x00, 0x00, 0x00,  // X pixels per meter
        0x00, 0x00, 0x00, 0x00,  // Y pixels per meter
        0x02, 0x00, 0x00, 0x00,  // Colors in palette (2)
        0x00, 0x00, 0x00, 0x00,  // Important colors (0 = all)

        // Color Palette (8 bytes = 2 colors * 4 bytes each)
        0x00, 0x00, 0x00, 0x00,  // Color 0: Black (BGRA)
        0xFF, 0xFF, 0xFF, 0x00,  // Color 1: White (BGRA)

        // XOR Bitmap (32 bytes = 16 lines * 2 bytes per line)
        0xFF, 0xFF, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01,
        0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01,
        0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01,
        0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0xFF, 0xFF,

        // AND Bitmap (32 bytes = 16 lines * 2 bytes per line)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut file = std::fs::File::create(ico_path)?;
    file.write_all(&ico_data)?;

    Ok(())
}

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new().unwrap();

    let mut ico = "./wei.ico";
    let mut path = std::env::current_dir()?;
    path.push("./src/main.rs");
    if path.exists() {
        // 确保res目录存在
        let res_dir = std::path::Path::new("./res");
        if !res_dir.exists() {
            std::fs::create_dir_all(res_dir)?;
        }
        ico = "./res/wei.ico";
    }

    let ico_path = std::path::Path::new(ico);

    // 如果ico文件不存在，创建一个默认的ico文件
    if !ico_path.exists() {
        create_default_ico(ico_path)?;
    }

    // 尝试设置图标，如果失败则使用默认方式
    match app.set_icon_from_file(&ico_path.to_string_lossy()) {
        Ok(_) => {},
        Err(_) => {
            // 如果设置图标失败，尝试不设置图标或使用系统默认图标
            println!("Warning: Failed to set custom icon, using default");
        }
    }
    app.add_menu_item(&"启动界面".to_string(), |_| {
      match webbrowser::open("http://127.0.0.1:1115") {
        Ok(_) => {}
        Err(_) => {}
      }
      Ok::<_, systray::Error>(())
    }).unwrap();
    app.add_menu_item(&"退出".to_string(), |window| {
        wei_env::stop();
        window.quit();
        Ok::<_, systray::Error>(())
    }).unwrap();
    app.wait_for_message().unwrap();

    wei_run::kill("wei")?;
    
    Ok(())
}

// use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
// use tauri::Manager;

// pub fn start() -> Result<(), Box<dyn std::error::Error>> {
//   let quit = CustomMenuItem::new("exit".to_string(), "退出");
//   let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
//   let tray_menu = SystemTrayMenu::new()
//     .add_item(quit)
//     .add_native_item(SystemTrayMenuItem::Separator)
//     .add_item(hide);

//   tauri::Builder::default()
//     .system_tray(SystemTray::new().with_menu(tray_menu))
//     .on_system_tray_event(|app, event| match event {
//       SystemTrayEvent::LeftClick {
//         position: _,
//         size: _,
//         ..
//       } => {
//         let window = app.get_window("main").unwrap();
//         window.show().unwrap();
//       }
//       SystemTrayEvent::RightClick {
//         position: _,
//         size: _,
//         ..
//       } => {
//         println!("system tray received a right click");
//       }
//       SystemTrayEvent::DoubleClick {
//         position: _,
//         size: _,
//         ..
//       } => {
//         let window = app.get_window("main").unwrap();
//         window.show().unwrap();
//       }
//       SystemTrayEvent::MenuItemClick { id, .. } => {
//         match id.as_str() {
//           "exit" => {
//             wei_env::stop();
//             std::process::exit(0);
//           }
//           "hide" => {
//             let window = app.get_window("main").unwrap();
//             window.hide().unwrap();
//           }
//           _ => {}
//         }
//       }
//       _ => {}
//     })
//     .run(tauri::generate_context!("./tauri.conf.json"))
//     .expect("error while running tauri application");

//     Ok(())
// }
