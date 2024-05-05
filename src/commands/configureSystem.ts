import shell from "shelljs";
import fs from "fs";
import path from "path";

// Function to install a software package
function installPackage(packageName: string): void {
  console.log(`Attempting to install ${packageName}...`);
  if (shell.exec(`sudo dnf install -y ${packageName}`).code !== 0) {
    console.error(`Failed to install ${packageName}`);
  } else {
    console.log(`${packageName} installed successfully.`);
  }
}

// Function to copy a configuration file
function configureFile(sourcePath: string, targetPath: string): void {
  console.log(`Configuring ${path.basename(targetPath)}...`);
  try {
    fs.copyFileSync(sourcePath, targetPath);
    console.log(`${path.basename(targetPath)} configured successfully.`);
  } catch (err) {
    console.error(`Failed to configure ${path.basename(targetPath)}: ${err}`);
  }
}

// Function to configure the system
async function configureSystem(): Promise<void> {
  // const packages = ["curl", "vim"]; // Add more packages as needed
  // const configs = [
  //   {
  //     source: "/path/to/default/.vimrc",
  //     target: "/home/username/.vimrc",
  //   },
  // ];
  // // Install missing packages
  // packages.forEach(installPackage);
  // // Configure necessary files
  // configs.forEach((config) => configureFile(config.source, config.target));
}

export { configureSystem };
