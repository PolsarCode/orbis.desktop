#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin(tauri_plugin_updater::Builder::new().build())
    .setup(|app| {
      open_main_window(app)?;
      #[cfg(not(debug_assertions))]
      start_on_login_by_default(app);
      #[cfg(not(debug_assertions))]
      install_updates(app);
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}

fn open_main_window(app: &tauri::App) -> tauri::Result<()> {
  use std::{collections::HashMap, path::PathBuf, sync::Mutex};
  use tauri::webview::{DownloadEvent, NewWindowResponse};

  let Some(config) = app.config().app.windows.first() else {
    return Ok(());
  };
  let downloads = Mutex::new(HashMap::<String, PathBuf>::new());
  tauri::WebviewWindowBuilder::from_config(app.handle(), config)?
    .on_new_window(|url, _| {
      if matches!(url.scheme(), "http" | "https" | "mailto") {
        let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
      }
      NewWindowResponse::Deny
    })
    .on_download(move |_, event| {
      match event {
        DownloadEvent::Requested { url, destination } => {
          if let Ok(mut pending) = downloads.lock() {
            pending.insert(url.to_string(), destination.clone());
          }
        }
        DownloadEvent::Finished { url, path, success } => {
          let saved = downloads
            .lock()
            .ok()
            .and_then(|mut pending| pending.remove(url.as_str()));
          if let (true, Some(path)) = (success, path.or(saved)) {
            let _ = tauri_plugin_opener::reveal_item_in_dir(path);
          }
        }
        _ => {}
      }
      true
    })
    .build()?;
  Ok(())
}

#[cfg(not(debug_assertions))]
fn start_on_login_by_default(app: &tauri::App) {
  use tauri::Manager;
  use tauri_plugin_autostart::ManagerExt;

  let Ok(dir) = app.path().app_config_dir() else {
    return;
  };
  let marker = dir.join("start-on-login-set");
  if marker.exists() {
    return;
  }
  if app.autolaunch().enable().is_ok() {
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(marker, "");
  }
}

#[cfg(not(debug_assertions))]
fn install_updates(app: &tauri::App) {
  use tauri_plugin_updater::UpdaterExt;

  let handle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    let Ok(updater) = handle.updater() else {
      return;
    };
    let Ok(Some(update)) = updater.check().await else {
      return;
    };
    if update.download_and_install(|_, _| {}, || {}).await.is_ok() {
      handle.restart();
    }
  });
}
