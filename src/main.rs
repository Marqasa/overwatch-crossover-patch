use directories::{BaseDirs, UserDirs};
use downloader::{downloader::Builder, Download};
use flate2::read::GzDecoder;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use tar::Archive;
use xz::read::XzDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if CrossOver is installed
    let crossover_path = Path::new("/Applications/CrossOver.app/Contents/SharedSupport/CrossOver");

    if !crossover_path.exists() {
        println!("CrossOver not found");
        return Ok(());
    }

    // Get bottle name from user
    let mut bottle_name = String::new();

    println!("Enter the name of your Overwatch bottle:");

    std::io::stdin()
        .read_line(&mut bottle_name)
        .expect("Failed to read line");

    // Trim the bottle name
    bottle_name = bottle_name.trim().to_string();

    // Get the bottle folder
    let binding = BaseDirs::new()
        .expect("No base directories found")
        .home_dir()
        .join(format!(
            "Library/Application Support/CrossOver/Bottles/{}",
            bottle_name
        ));
    let bottle_path = binding.as_path();

    // Check if the bottle exists
    if !bottle_path.exists() {
        println!("Bottle not found");
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
        fs::create_dir(dependencies_path)?;
    }

    // Set the dependency file names and URLs
    let moltenvk_file = "macos-1.2.3.tar.xz";
    let moltenvk_url = format!(
        "https://github.com/The-Wineskin-Project/MoltenVK/releases/download/v1.2.3/{}",
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
    downloader.download(&downloads)?;

    // Let the user know that the dependencies are being installed
    println!("Installing dependencies...");

    // Extract the files
    let tar_xz = File::open(dependencies_path.join(moltenvk_file))?;
    let decompressor = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(decompressor);
    archive.unpack(dependencies_path)?;

    let tar_gz = File::open(dependencies_path.join(dxvk_file))?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dependencies_path)?;

    // Copy the files to the correct location
    let dxvk_x32_path = dependencies_path.join("dxvk-macOS-async-v1.10.3-20230402-CrossOver/x32");
    let dxvk_x64_path = dependencies_path.join("dxvk-macOS-async-v1.10.3-20230402-CrossOver/x64");
    let dxvk_x32_crossover_path = crossover_path.join("lib/wine/dxvk");
    let dxvk_x64_crossover_path = crossover_path.join("lib64/wine/dxvk");
    let bottle_x64_path = bottle_path.join("drive_c/windows/system32");

    // Copy the files from the dxvk x32 folder to the crossover x32 folder
    for entry in fs::read_dir(dxvk_x32_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().expect("No file name found");
        fs::copy(&path, dxvk_x32_crossover_path.join(file_name))?;
    }

    // Copy the files from the dxvk x64 folder to the crossover x64 folder and the bottle x64 folder
    for entry in fs::read_dir(dxvk_x64_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().expect("No file name found");
        fs::copy(&path, dxvk_x64_crossover_path.join(file_name))?;
        fs::copy(&path, bottle_x64_path.join(file_name))?;
    }

    // Copy the moltenvk dylib file to the crossover folder
    let moltenvk_path =
        dependencies_path.join("Package/Release/MoltenVK/dylib/macOS/libMoltenVK.dylib");
    let moltenvk_crossover_path = crossover_path.join("lib64/libMoltenVK.dylib");
    fs::copy(moltenvk_path, moltenvk_crossover_path)?;

    // Copy the dxvk cache file to the bottle folder
    fs::copy(
        dependencies_path.join("Overwatch.dxvk-cache"),
        bottle_path.join("drive_c/Program Files (x86)/Overwatch/_retail_/Overwatch.dxvk-cache"),
    )?;

    // Copy the settings file to the documents folder
    let documents_path = user_dirs
        .document_dir()
        .expect("No documents directory found");
    let settings_path = documents_path.join("Overwatch/Settings/Settings_v0.ini");

    // Copy the settings file
    fs::copy(dependencies_path.join("Settings_v0.ini"), settings_path)?;

    // Remove the dependencies folder
    fs::remove_dir_all(dependencies_path)?;

    // Update the bottle config
    update_bottle_config(bottle_path);

    // Let the user know that the dxvk config is being created
    println!("Creating dxvk config...");

    // Creat dxvk.conf file
    let dxvk_conf_path =
        bottle_path.join("drive_c/Program Files (x86)/Overwatch/_retail_/dxvk.conf");
    let mut dxvk_conf = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dxvk_conf_path)
        .unwrap();

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
        println!("Couldn't find bottle config. Please check your CrossOver installation.");

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
                println!("Couldn't write to bottle config: {}", e);
            }
        }
    }
}
