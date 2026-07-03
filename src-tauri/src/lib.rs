use tauri::Emitter;
use tauri::Manager;
use tauri::Listener;
use std::io::Write;
use std::sync::Mutex;

#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
use tauri::menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu};

static APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);

fn set_app_handle(handle: tauri::AppHandle) {
    *APP_HANDLE.lock().unwrap() = Some(handle);
}

#[allow(dead_code)]
fn get_app_handle() -> Option<tauri::AppHandle> {
    APP_HANDLE.lock().unwrap().clone()
}

static OPEN_FILE_CACHE: Mutex<Option<serde_json::Value>> = Mutex::new(None);

/// macOS application:openFiles: 委托方法（Tao 未实现，需自行注册）
#[cfg(target_os = "macos")]
extern "C" fn application_open_files(
    _self: &objc::runtime::Object,
    _cmd: objc::runtime::Sel,
    _app: *mut objc::runtime::Object,
    files: *mut objc::runtime::Object,
) {
    unsafe {
        let count: usize = msg_send![files, count];
        log(&format!("[application:openFiles:] received {} file(s)", count));
        for i in 0..count {
            let file: *mut objc::runtime::Object = msg_send![files, objectAtIndex: i];
            let utf8: *const i8 = msg_send![file, UTF8String];
            if utf8.is_null() {
                continue;
            }
            let path = std::ffi::CStr::from_ptr(utf8)
                .to_string_lossy()
                .to_string();
            log(&format!("[application:openFiles:] path={}", path));
            if let Some(handle) = get_app_handle() {
                if let Some(window) = handle.get_webview_window("main") {
                    let path_lower = path.to_lowercase();
                    if path_lower.ends_with(".md")
                        || path_lower.ends_with(".markdown")
                        || path_lower.ends_with(".txt")
                    {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            let file_name = std::path::Path::new(&path)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("未命名.md")
                                .to_string();
                            log(&format!(
                                "[application:openFiles:] file read OK: {} ({} chars)",
                                file_name,
                                content.len()
                            ));
                            window
                                .emit(
                                    "file-open",
                                    serde_json::json!({
                                        "path": path,
                                        "name": file_name,
                                        "content": content
                                    }),
                                )
                                .ok();
                        } else {
                            log(&format!("[application:openFiles:] read failed: {}", path));
                            window
                                .emit(
                                    "file-open-error",
                                    serde_json::json!({
                                        "path": path,
                                        "error": "无法读取文件，可能需要授予文件访问权限"
                                    }),
                                )
                                .ok();
                        }
                    }
                }
            }
        }
    }
}

/// macOS application:openFile: 委托方法（单数版本，旧 API）
#[cfg(target_os = "macos")]
extern "C" fn application_open_file(
    _self: &objc::runtime::Object,
    _cmd: objc::runtime::Sel,
    _app: *mut objc::runtime::Object,
    file: *mut objc::runtime::Object,
) {
    unsafe {
        let utf8: *const i8 = msg_send![file, UTF8String];
        if utf8.is_null() {
            log("[application:openFile:] UTF8String is null");
            return;
        }
        let path = std::ffi::CStr::from_ptr(utf8)
            .to_string_lossy()
            .to_string();
        log(&format!("[application:openFile:] path={}", path));

        if let Some(handle) = get_app_handle() {
            if let Some(window) = handle.get_webview_window("main") {
                let path_lower = path.to_lowercase();
                if path_lower.ends_with(".md")
                    || path_lower.ends_with(".markdown")
                    || path_lower.ends_with(".txt")
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let file_name = std::path::Path::new(&path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("未命名.md")
                            .to_string();
                        log(&format!(
                            "[application:openFile:] file read OK: {} ({} chars)",
                            file_name,
                            content.len()
                        ));
                        window
                            .emit(
                                "file-open",
                                serde_json::json!({
                                    "path": path,
                                    "name": file_name,
                                    "content": content
                                }),
                            )
                            .ok();
                    } else {
                        log(&format!("[application:openFile:] read failed: {}", path));
                        window
                            .emit(
                                "file-open-error",
                                serde_json::json!({
                                    "path": path,
                                    "error": "无法读取文件，可能需要授予文件访问权限"
                                }),
                            )
                            .ok();
                    }
                }
            }
        }
    }
}

/// 在 Tao 的 NSApp 委托上注册 application:openFiles: / openFile: 方法
#[cfg(target_os = "macos")]
unsafe fn register_open_files_handler() {
    use std::ffi::CString;
    use libc::{c_void, dlsym, RTLD_DEFAULT};

    type ClassAddMethod =
        unsafe extern "C" fn(*mut c_void, objc::runtime::Sel, *const c_void, *const i8) -> u8;

    let ptr = dlsym(RTLD_DEFAULT, CString::new("class_addMethod").unwrap().as_ptr());
    if ptr.is_null() {
        log("[register_open_files_handler] class_addMethod not found");
        return;
    }
    let class_add_method: ClassAddMethod = std::mem::transmute(ptr);

    let Some(class) = objc::runtime::Class::get("TaoAppDelegate") else {
        log("[register_open_files_handler] TaoAppDelegate class not found");
        return;
    };

    // 注册 application:openFiles:（复数）
    let sel_files = sel!(application:openFiles:);
    let method_files: extern "C" fn(
        &objc::runtime::Object,
        objc::runtime::Sel,
        *mut objc::runtime::Object,
        *mut objc::runtime::Object,
    ) = application_open_files;
    let types_files = CString::new("v@:@@").unwrap();
    let result_files = class_add_method(
        class as *const _ as *mut c_void,
        sel_files,
        method_files as *const extern "C" fn(&objc::runtime::Object, objc::runtime::Sel, *mut objc::runtime::Object, *mut objc::runtime::Object) as *const c_void,
        types_files.as_ptr(),
    );
    log(&format!("[register_open_files_handler] application:openFiles: result={}", result_files));

    // 注册 application:openFile:（单数）
    let sel_file = sel!(application:openFile:);
    let method_file: extern "C" fn(
        &objc::runtime::Object,
        objc::runtime::Sel,
        *mut objc::runtime::Object,
        *mut objc::runtime::Object,
    ) = application_open_file;
    let types_file = CString::new("v@:@@").unwrap();
    let result_file = class_add_method(
        class as *const _ as *mut c_void,
        sel_file,
        method_file as *const extern "C" fn(&objc::runtime::Object, objc::runtime::Sel, *mut objc::runtime::Object, *mut objc::runtime::Object) as *const c_void,
        types_file.as_ptr(),
    );
    log(&format!("[register_open_files_handler] application:openFile: result={}", result_file));
}

/// 获取日志目录：放在 App 内部的 Contents/MacOS/logs/ 下（与二进制同目录，可写）
fn log_dir() -> Option<std::path::PathBuf> {
    std::env::current_exe().ok().and_then(|exe| {
        let macos_dir = exe.parent()?;
        let logs_dir = macos_dir.join("logs");
        let _ = std::fs::create_dir_all(&logs_dir);
        Some(logs_dir)
    })
}

/// 写入日志文件（追加模式），同时输出到 stderr
fn log(msg: &str) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let line = format!("[{}] {}\n", timestamp, msg);
    if let Some(dir) = log_dir() {
        let log_path = dir.join("app.log");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            let _ = file.write_all(line.as_bytes());
            let _ = file.flush();
        }
    }
    eprintln!("{}", msg);
}

/// 统一 emit file-open 事件，失败时缓存到 OPEN_FILE_CACHE
fn emit_file_open(window: &tauri::WebviewWindow, path: &str, name: &str, content: &str) {
    log(&format!("[emit_file_open] {} ({} chars)", name, content.len()));
    if let Err(_) = window.emit("file-open", serde_json::json!({
        "path": path,
        "name": name,
        "content": content
    })) {
        log("[emit_file_open] Window not ready, caching");
        let mut cache = OPEN_FILE_CACHE.lock().unwrap();
        *cache = Some(serde_json::json!({
            "path": path,
            "name": name,
            "content": content
        }));
    }
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    log(&format!("[read_file] path={}", path));
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    log(&format!("[write_file] path={} len={}", path, content.len()));
    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct DirEntry {
    name: String,
    path: String,
    is_directory: bool,
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<DirEntry>, String> {
    log(&format!("[read_dir] path={}", path));
    let mut entries = Vec::new();
    let dir = std::fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in dir {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path().to_string_lossy().to_string();
        let is_directory = entry.file_type().map_err(|e| e.to_string())?.is_dir();
        entries.push(DirEntry {
            name: file_name,
            path: file_path,
            is_directory,
        });
    }
    Ok(entries)
}

/// macOS 系统菜单创建
#[cfg(target_os = "macos")]
fn create_macos_menu<R: tauri::Runtime>(app: &tauri::App<R>) -> tauri::Result<Menu<R>> {
    let menu = Menu::new(app)?;

    let about_metadata = AboutMetadata {
        name: Some("MD Preview".to_string()),
        version: Some("1.0.0".to_string()),
        ..Default::default()
    };

    // App 菜单
    let app_menu = Submenu::with_items(
        app,
        "MD Preview",
        true,
        &[
            &PredefinedMenuItem::about(app, None, Some(about_metadata))?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::services(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )?;
    menu.append(&app_menu)?;

    // 文件菜单
    let new_item = MenuItem::with_id(app, "menu-new", "新建", true, Some("CmdOrCtrl+N"))?;
    let open_item = MenuItem::with_id(app, "menu-open", "打开文件...", true, Some("CmdOrCtrl+O"))?;
    let open_folder_item = MenuItem::with_id(app, "menu-open-folder", "打开文件夹...", true, Some("CmdOrCtrl+Shift+O"))?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let save_item = MenuItem::with_id(app, "menu-save", "保存", true, Some("CmdOrCtrl+S"))?;
    let save_as_item = MenuItem::with_id(app, "menu-save-as", "另存为...", true, Some("CmdOrCtrl+Shift+S"))?;
    let file_menu = Submenu::with_items(
        app,
        "文件",
        true,
        &[&new_item, &open_item, &open_folder_item, &sep1, &save_item, &save_as_item],
    )?;
    menu.append(&file_menu)?;

    // 编辑菜单
    let edit_menu = Submenu::with_items(
        app,
        "编辑",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;
    menu.append(&edit_menu)?;

    // 视图菜单
    let split_item = MenuItem::with_id(app, "menu-view-split", "左右对比", true, Some("CmdOrCtrl+1"))?;
    let editor_item = MenuItem::with_id(app, "menu-view-editor", "仅编辑", true, Some("CmdOrCtrl+2"))?;
    let preview_item = MenuItem::with_id(app, "menu-view-preview", "仅预览", true, Some("CmdOrCtrl+3"))?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sidebar_item = MenuItem::with_id(app, "menu-sidebar", "文件树", true, Some("CmdOrCtrl+B"))?;
    let toc_item = MenuItem::with_id(app, "menu-toc", "目录", true, Some("CmdOrCtrl+T"))?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let zen_item = MenuItem::with_id(app, "menu-zen", "禅模式", true, Some("CmdOrCtrl+Shift+H"))?;
    let sep4 = PredefinedMenuItem::separator(app)?;
    let theme_item = MenuItem::with_id(app, "menu-theme", "切换主题", true, None::<&str>)?;
    let view_menu = Submenu::with_items(
        app,
        "视图",
        true,
        &[
            &split_item, &editor_item, &preview_item, &sep2,
            &sidebar_item, &toc_item, &sep3,
            &zen_item, &sep4, &theme_item,
        ],
    )?;
    menu.append(&view_menu)?;

    // Window 菜单
    let window_menu = Submenu::with_id_and_items(
        app,
        tauri::menu::WINDOW_SUBMENU_ID,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;
    menu.append(&window_menu)?;

    // Help 菜单
    let help_item = MenuItem::with_id(app, "menu-help", "快捷键帮助", true, None::<&str>)?;
    let help_menu = Submenu::with_id_and_items(
        app,
        tauri::menu::HELP_SUBMENU_ID,
        "帮助",
        true,
        &[&help_item],
    )?;
    menu.append(&help_menu)?;

    Ok(menu)
}

/// macOS 菜单事件处理
#[cfg(target_os = "macos")]
fn handle_menu_event(app_handle: &tauri::AppHandle, menu_id: &tauri::menu::MenuId) {
    let id = menu_id.as_ref();
    log(&format!("[MenuEvent] id: {}", id));
    if let Some(window) = app_handle.get_webview_window("main") {
        window.emit("menu-action", id).ok();
    }
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![read_file, write_file, read_dir])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                set_app_handle(app.app_handle().clone());
                window.set_title("MD Preview").unwrap();
                match create_macos_menu(app) {
                    Ok(menu) => {
                        if let Err(e) = window.set_menu(menu) {
                            log(&format!("[setup] Failed to set menu: {}", e));
                        } else {
                            log("[setup] Menu set successfully");
                        }
                    }
                    Err(e) => log(&format!("[setup] Failed to create menu: {}", e)),
                }
            }

            log(&format!("[setup] App started, log_dir={:?}", log_dir()));

            // 处理终端启动时的命令行参数
            let args: Vec<String> = std::env::args().collect();
            log(&format!("[setup] args: {:?}", args));
            if args.len() > 1 {
                let file_path = &args[1];
                let path_lower = file_path.to_lowercase();
                if path_lower.ends_with(".md") || path_lower.ends_with(".markdown") || path_lower.ends_with(".txt") {
                    log(&format!("[setup] Opening file from args: {}", file_path));
                    if let Ok(content) = std::fs::read_to_string(file_path) {
                        let file_name = std::path::Path::new(file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("未命名.md")
                            .to_string();
                        window.emit("file-open", serde_json::json!({
                            "path": file_path,
                            "name": file_name,
                            "content": content
                        })).ok();
                    } else {
                        window.emit("file-open-error", serde_json::json!({
                            "path": file_path,
                            "error": "无法读取文件，可能需要授予文件访问权限"
                        })).ok();
                    }
                }
            }

            // 检查是否有缓存的启动文件（RunEvent::Opened 在 setup 之前触发时缓存）
            // 延迟到前端 listener 注册完成后再 emit
            let window_clone = window.clone();
            let app_handle_clone = app.app_handle().clone();
            app_handle_clone.listen("app-ready", move |_| {
                log("[app-ready] Frontend ready, checking cache");
                let cache = OPEN_FILE_CACHE.lock().unwrap().take();
                if let Some(payload) = cache {
                    log(&format!("[app-ready] Emitting cached file-open"));
                    window_clone.emit("file-open", payload).ok();
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    use std::sync::Once;
    static REGISTER_OPEN_FILES: Once = Once::new();

    app.run(|app_handle, event| {
        #[cfg(target_os = "macos")]
        REGISTER_OPEN_FILES.call_once(|| {
            unsafe { register_open_files_handler(); }
        });

        match event {
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Opened { urls } => {
                log(&format!("[RunEvent::Opened] received {} URL(s)", urls.len()));
                for url in urls {
                    log(&format!("[RunEvent::Opened] URL: {}", url.as_str()));
                    if url.scheme() == "file" {
                        if let Ok(path) = url.to_file_path() {
                            let file_path = path.to_string_lossy();
                            log(&format!("[RunEvent::Opened] decoded path: {}", file_path));
                            let path_lower = file_path.to_lowercase();
                            if path_lower.ends_with(".md") || path_lower.ends_with(".markdown") || path_lower.ends_with(".txt") {
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let file_name = path.file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("未命名.md")
                                        .to_string();
                                    log(&format!("[RunEvent::Opened] file read OK: {} ({} chars)", file_name, content.len()));
                                    if let Some(window) = app_handle.get_webview_window("main") {
                                        emit_file_open(&window, &file_path, &file_name, &content);
                                    } else {
                                        log("[RunEvent::Opened] Window not ready, caching");
                                        let mut cache = OPEN_FILE_CACHE.lock().unwrap();
                                        *cache = Some(serde_json::json!({
                                            "path": file_path.to_string(),
                                            "name": file_name,
                                            "content": content
                                        }));
                                    }
                                } else {
                                    log(&format!("[RunEvent::Opened] read failed: {}", file_path));
                                    if let Some(window) = app_handle.get_webview_window("main") {
                                        window.emit("file-open-error", serde_json::json!({
                                            "path": file_path.to_string(),
                                            "error": "无法读取文件，可能需要授予文件访问权限"
                                        })).ok();
                                    }
                                }
                            } else {
                                log("[RunEvent::Opened] skipped: not a markdown file");
                            }
                        } else {
                            log(&format!("[RunEvent::Opened] to_file_path() failed: {}", url.as_str()));
                        }
                    }
                }
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::MenuEvent(event) => {
                handle_menu_event(app_handle, &event.id);
            }
            _ => {}
        }
    });
}
