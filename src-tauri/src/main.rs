#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use sysinfo::{Process, ProcessExt, System, SystemExt};
use tauri::regex::Regex;
use tauri::Manager;

struct State {
  steam: Steam,
}

fn main() {
  let state = State {
    steam: Steam::new(),
  };

  let s = System::new_all();
  if s.processes_by_exact_name("steam").next().is_none() {
    state.steam.launch();
  }

  tauri::Builder::default()
    .manage(state)
    .setup(|app| {
      let state: tauri::State<State> = app.state();
      let main_window = app.get_window("main").unwrap();
      let games: Vec<Game> = state.steam.find_games();

      app.emit_all("games", games).expect("tf");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![launch_game, find_games])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Game {
  pub appid: String,
  pub name: String,
}
#[tauri::command]
fn find_games(state: tauri::State<State>) -> Result<Vec<Game>, String> {
  Ok(state.steam.find_games())
}
#[tauri::command]
fn launch_game(state: tauri::State<State>, appid: String) -> Result<(), String> {
  dbg!("asdasdasdas");
  state.steam.exec(format!("run/{}", appid));
  dbg!("running appid {}", appid);
  Ok(())
}

struct Steam {
  root_path: PathBuf,
  executable_path: PathBuf,
}
impl Steam {
  pub fn new() -> Steam {
    let executable_bytes = Command::new("which").arg("steam").output().unwrap().stdout;
    let executable_path = PathBuf::from(
      String::from_utf8(executable_bytes)
        .unwrap()
        .replace('\n', ""),
    );
    Steam {
      root_path: Path::new("/home/ce/.local/share/Steam/").to_path_buf(),
      executable_path,
    }
  }
  pub fn launch(&self) {
    Command::new(self.executable_path.as_os_str().to_str().unwrap())
      .arg("-silent")
      .spawn()
      .expect("Couldn't start steam!");
  }
  pub fn find_games(&self) -> Vec<Game> {
    let mut games = vec![];
    for entry in fs::read_dir(self.root_path.join("steamapps")).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      let stem = path.file_stem().unwrap().to_str().unwrap();
      if !path.is_file() {
        continue;
      }
      if !Regex::new("appmanifest_.*").unwrap().is_match(stem) {
        continue;
      }
      let mut file = File::open(&path).unwrap();
      let mut manifest = String::default();
      file.read_to_string(&mut manifest).unwrap();

      manifest = manifest.replace('\t', "");
      let lines: Vec<&str> = manifest.lines().collect();
      let appid = lines[2].split('"').collect::<Vec<_>>()[3].to_string();
      let name = lines[4].split('"').collect::<Vec<_>>()[3].to_string();

      // don't add if it doesn't have a userconfig
      let game = Game { appid, name };
      games.push(game);
    }

    games
  }
  pub fn exec(&self, command: String) {
    Command::new(self.executable_path.as_os_str().to_str().unwrap())
      .arg(format!("steam://{}", command))
      .spawn()
      .expect("Couldn't start steam!");
  }
}
