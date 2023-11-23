use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use directories::{BaseDirs, UserDirs};
use downloader::{downloader::Builder, Download};
use flate2::read::GzDecoder;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;

enum Client {
    BattleNet,
    Steam,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the CrossOver folder inside the CrossOver.app bundle
    let inner_crossover_path = "Contents/SharedSupport/CrossOver";

    // Full path to the CrossOver folder
    let crossover_path = Path::new("/Applications/CrossOver.app").join(inner_crossover_path);

    // Check if the crossover path exists
    if !crossover_path.exists() {
        // Prompt the user to enter the crossover location
        println!("CrossOver not found. If you have it installed in a custom location, please enter the path here (e.g. /Applications/CrossOver.app):");

        // Declare the crossover location
        let mut crossover_location = String::new();

        // Get the crossover location from the user
        std::io::stdin()
            .read_line(&mut crossover_location)
            .expect("Failed to read line");

        // Trim the crossover location
        crossover_location = crossover_location.trim().to_string();

        // Create the crossover path
        let crossover_path = Path::new(&crossover_location).join(inner_crossover_path);

        // Check if the crossover path exists
        if !crossover_path.exists() {
            eprintln!("CrossOver not found");
            return Ok(());
        }
    }

    // Get bottle name or path from user
    let mut bottle_location = String::new();

    println!("Enter your Overwatch bottle name (or path with --path /path/to/bottle):");

    std::io::stdin()
        .read_line(&mut bottle_location)
        .expect("Failed to read line");

    // Trim the bottle location
    bottle_location = bottle_location.trim().to_string();

    // Declare the bottle path buffer
    let bottle_path_buf: PathBuf;

    // Check if the bottle is a path
    if bottle_location.starts_with("--path") {
        // Split the bottle location
        let mut parts = bottle_location.splitn(2, ' ');

        // Get the path
        bottle_path_buf = PathBuf::from(
            parts
                .nth(1)
                .expect("No path found")
                .trim()
                .replace("\\", ""),
        );
    } else {
        // Get the path from the bottle name
        bottle_path_buf = BaseDirs::new()
            .expect("No base directories found")
            .home_dir()
            .join(format!(
                "Library/Application Support/CrossOver/Bottles/{}",
                bottle_location
            ));
    }

    // Convert PathBuf to Path
    let bottle_path = bottle_path_buf.as_path();

    // Check if the bottle exists
    if !bottle_path.exists() {
        eprintln!("Bottle not found");
        return Ok(());
    }

    // Prompt the user to select a game client
    let items = vec!["Battle.net", "Steam"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select your game client:")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let client = match selection {
        Some(0) => Client::BattleNet,
        Some(1) => Client::Steam,
        _ => {
            eprintln!("No client selected");
            return Ok(());
        }
    };

    // Check installation exists
    let game_file = "Overwatch.exe";
    let installation_path = match client {
        Client::BattleNet => bottle_path.join("drive_c/Program Files (x86)/Overwatch/_retail_"),
        Client::Steam => {
            bottle_path.join("drive_c/Program Files (x86)/Steam/steamapps/common/Overwatch")
        }
    };

    if !installation_path.join(&game_file).exists() {
        eprintln!("Overwatch installation not found");
        return Ok(());
    }

    // Get the user's home directory
    let user_dirs = UserDirs::new().expect("No user directories found");

    // Let the user know that the dependencies are being downloaded
    println!("Downloading dependencies...");

    // Create the overwatch_dependencies folder
    let binding = user_dirs
        .download_dir()
        .expect("No download directory found")
        .join("overwatch_dependencies");
    let dependencies_path = binding.as_path();

    // Create the directory if it doesn't exist
    if !dependencies_path.exists() {
        fs::create_dir(dependencies_path).map_err(|e| {
            eprintln!(
                "Failed to create directory: {}",
                dependencies_path.display()
            );
            return e;
        })?;
    }

    // Set the dependency file names and URLs
    let moltenvk_file = "MoltenVK-macos.tar";
    let moltenvk_url = format!(
        "https://github.com/KhronosGroup/MoltenVK/releases/download/v1.2.4/{}",
        moltenvk_file
    );
    let dxvk_file = "dxvk-macOS-async-v1.10.3-20230402-CrossOver.tar.gz";
    let dxvk_url = format!(
        "https://github.com/Gcenx/DXVK-macOS/releases/download/v1.10.3-20230402/{}",
        dxvk_file
    );
    let dxvk_cache_file = "Overwatch.dxvk-cache";
    let dxvk_cache_url =
        "https://drive.google.com/uc?id=1bEkruqhvQTwjv5V2ZmmQIWsAwhuIAPT_&export=download"
            .to_owned();
    let settings_file = "Settings_v0.ini";
    let settings_url =
        "https://drive.google.com/uc?id=1xQMN3YFnmIUb5oj15qQdCWetjkM22IyD&export=download"
            .to_owned();

    // Create the downloader
    let mut builder = Builder::default();
    builder.download_folder(dependencies_path);
    let mut downloader = builder.build().expect("Failed to build downloader");

    // Create the downloads
    let mut downloads: Vec<Download> = Vec::new();

    downloads.push(Download::new(&dxvk_url));
    downloads.push(Download::new(&moltenvk_url));

    let mut dxvk_cache = Download::new(&dxvk_cache_url);
    dxvk_cache = dxvk_cache.file_name(Path::new(dxvk_cache_file));
    downloads.push(dxvk_cache);

    let mut settings = Download::new(&settings_url);
    settings = settings.file_name(Path::new(settings_file));
    downloads.push(settings);

    // Download the files
    downloader.download(&downloads).map_err(|e| {
        eprintln!("Failed to download files");
        return e;
    })?;

    // Let the user know that the dependencies are being installed
    println!("Installing dependencies...");

    // Extract the files
    let moltenvk_path = dependencies_path.join(moltenvk_file);
    let moltenvk_tar = File::open(&moltenvk_path).map_err(|e| {
        eprintln!("Failed to open file: {}", moltenvk_path.display());
        return e;
    })?;
    let mut archive = Archive::new(moltenvk_tar);
    archive.unpack(dependencies_path).map_err(|e| {
        eprintln!("Failed to extract file: {}", moltenvk_path.display());
        return e;
    })?;

    let dxvk_path = dependencies_path.join(dxvk_file);
    let tar_gz = File::open(&dxvk_path).map_err(|e| {
        eprintln!("Failed to open file: {}", dxvk_path.display());
        return e;
    })?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dependencies_path).map_err(|e| {
        eprintln!("Failed to extract file: {}", dxvk_path.display());
        return e;
    })?;

    // Copy the files to the correct location
    let dxvk_x32_path = dependencies_path.join("dxvk-macOS-async-v1.10.3-20230402-CrossOver/x32");
    let dxvk_x64_path = dependencies_path.join("dxvk-macOS-async-v1.10.3-20230402-CrossOver/x64");
    let dxvk_x32_crossover_path = crossover_path.join("lib/wine/dxvk");
    let dxvk_x64_crossover_path = crossover_path.join("lib64/wine/dxvk");
    let bottle_x64_path = bottle_path.join("drive_c/windows/system32");

    // Copy the files from the dxvk x32 folder to the crossover x32 folder
    for entry in fs::read_dir(&dxvk_x32_path).map_err(|e| {
        eprintln!("Failed to read directory: {}", dxvk_x32_path.display());
        return e;
    })? {
        let entry = entry.map_err(|e| {
            eprintln!("Failed to read directory: {}", dxvk_x32_path.display());
            return e;
        })?;
        let path = entry.path();
        let file_name = path.file_name().expect("No file name found");
        let file_path = dxvk_x32_crossover_path.join(file_name);
        fs::copy(&path, &file_path).map_err(|e| {
            eprintln!("Failed to copy file: {}", file_path.display());
            return e;
        })?;
    }

    // Copy the files from the dxvk x64 folder to the crossover x64 folder and the bottle x64 folder
    for entry in fs::read_dir(&dxvk_x64_path).map_err(|e| {
        eprintln!("Failed to read directory: {}", dxvk_x64_path.display());
        return e;
    })? {
        let entry = entry.map_err(|e| {
            eprintln!("Failed to read directory: {}", dxvk_x64_path.display());
            return e;
        })?;
        let path = entry.path();
        let file_name = path.file_name().expect("No file name found");
        let file_path = dxvk_x64_crossover_path.join(file_name);
        fs::copy(&path, &file_path).map_err(|e| {
            eprintln!("Failed to copy file: {}", file_path.display());
            return e;
        })?;
        let file_path = bottle_x64_path.join(file_name);
        fs::copy(&path, &file_path).map_err(|e| {
            eprintln!("Failed to copy file: {}", file_path.display());
            return e;
        })?;
    }

    // Copy the moltenvk dylib file to the crossover folder
    let moltenvk_path = dependencies_path.join("MoltenVK/MoltenVK/dylib/macOS/libMoltenVK.dylib");
    let moltenvk_crossover_path = crossover_path.join("lib64/libMoltenVK.dylib");
    fs::copy(moltenvk_path, &moltenvk_crossover_path).map_err(|e| {
        eprintln!("Failed to copy file: {}", moltenvk_crossover_path.display());
        return e;
    })?;

    // Let the user know that the dxvk cache is being copied
    println!("Copying DXVK cache...");

    // Copy the dxvk cache file to the bottle folder
    let dxvk_cache_folder = match client {
        Client::BattleNet => bottle_path.join("drive_c/Program Files (x86)/Overwatch/_retail_"),
        Client::Steam => bottle_path.join(
            "drive_c/Program Files (x86)/Steam/steamapps/shadercache/2357570/DXVK_state_cache",
        ),
    };

    fs::create_dir_all(&dxvk_cache_folder).map_err(|e| {
        eprintln!(
            "Failed to create directory: {}",
            dxvk_cache_folder.display()
        );
        return e;
    })?;

    fs::copy(
        dependencies_path.join(&dxvk_cache_file),
        dxvk_cache_folder.join(&dxvk_cache_file),
    )
    .map_err(|e| {
        eprintln!(
            "Failed to copy file: {}",
            dxvk_cache_folder.join(&dxvk_cache_file).display()
        );
        return e;
    })?;

    // Let the user know that the settings file is being copied
    println!("Copying settings file...");

    // Copy the settings file to the documents folder
    let documents_path = user_dirs
        .document_dir()
        .expect("No documents directory found");
    let settings_folder_path = documents_path.join("Overwatch/Settings");

    // Create the settings folder
    fs::create_dir_all(&settings_folder_path).map_err(|e| {
        eprintln!(
            "Failed to create directory: {}",
            settings_folder_path.display()
        );
        return e;
    })?;

    // Create the settings file path
    let settings_path = settings_folder_path.join("Settings_v0.ini");

    // Copy the settings file
    fs::copy(dependencies_path.join("Settings_v0.ini"), &settings_path).map_err(|e| {
        eprintln!("Failed to copy file: {}", settings_path.display());
        return e;
    })?;

    // Remove the dependencies folder
    fs::remove_dir_all(dependencies_path).map_err(|e| {
        eprintln!(
            "Failed to remove directory: {}",
            dependencies_path.display()
        );
        return e;
    })?;

    // Update the bottle config
    update_bottle_config(bottle_path);

    // Let the user know that the dxvk config is being created
    println!("Creating dxvk config...");

    // Creat dxvk.conf file
    let dxvk_conf_file = installation_path.join("dxvk.conf");
    let mut dxvk_conf = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&dxvk_conf_file)
        .map_err(|e| {
            eprintln!("Failed to create file: {}", dxvk_conf_file.display());
            return e;
        })?;

    if let Err(e) = writeln!(dxvk_conf, "dxvk.hud = compiler") {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Let the user know that the installation is complete
    println!("Installation complete!");

    Ok(())
}

/// Updates the bottle config
fn update_bottle_config(bottle_path: &Path) {
    // Let the user know that the bottle config is being updated
    println!("Updating bottle config...");

    // Get the bottle config path
    let bottle_config_path = bottle_path.join("cxbottle.conf");

    // Check if the bottle config exists
    if !bottle_config_path.exists() {
        // Warn the user that the bottle config couldn't be found
        eprintln!("Couldn't find bottle config. Please check your CrossOver installation.");

        return;
    }

    // Get the bottle config text
    let bottle_config_text = fs::read_to_string(&bottle_config_path).expect("Unable to read file");

    // Create a vector of environment variables
    let environment_variables = [
        "\"MVK_ALLOW_METAL_FENCES\" = \"1\"",
        "\"WINEESYNC\" = \"1\"",
    ];

    // Open the bottle config
    let mut bottle_config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(bottle_config_path)
        .expect("Unable to open file");

    // Loop through the environment variables
    for environment_variable in environment_variables.iter() {
        // Check if the environment variable is already in the bottle config
        if !bottle_config_text.contains(environment_variable) {
            // Write the environment variable to the bottle config
            if let Err(e) = writeln!(bottle_config, "{}", environment_variable) {
                // Warn the user that the environment variable couldn't be written to the bottle config
                eprintln!("Couldn't write to bottle config: {}", e);
            }
        }
    }
}
