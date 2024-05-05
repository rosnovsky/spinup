import si from "systeminformation";
import shell from "shelljs";
import chalk from "chalk";
import Table from "cli-table3";
import figlet from "figlet";
import fs from "node:fs";
import config from "../config.json";
import { Application, CheckResult } from "../types";

export function formatBytes(bytes: number, decimals = 2): string {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
}

export function formatSeconds(seconds: number, decimals = 2): string {
  if (seconds === 0) return "0 Seconds";

  const dm = decimals < 0 ? 0 : decimals;
  const months = Math.floor((seconds % 31536000) / 2592000);
  const days = Math.floor((seconds % 2592000) / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secondsLeft = seconds % 60;

  return `${months} month${months > 1 ? "s" : ""} ${days} day${
    days > 1 ? "s" : ""
  } ${hours} hour${hours > 1 ? "s" : ""} ${minutes} minute${
    minutes > 1 ? "s" : ""
  } ${secondsLeft.toPrecision(dm)} second${secondsLeft > 1 ? "s" : ""}`;
}

export async function displaySystemInfo() {
  const osInfo = await si.osInfo();
  const cpuInfo = await si.cpu();

  // Create a new table with customized styling
  const table = new Table({
    colWidths: [25, 55], // Set custom column widths
    style: {
      head: [], // Removes default coloring
      border: ["grey"], // Optional: you can specify border colors if you like
      "padding-left": 1,
      "padding-right": 1,
    },
    chars: {
      top: "═",
      "top-mid": "╤",
      "top-left": "╔",
      "top-right": "╗",
      bottom: "═",
      "bottom-mid": "╧",
      "bottom-left": "╚",
      "bottom-right": "╝",
      left: "║",
      "left-mid": "╟",
      mid: "─",
      "mid-mid": "┼",
      right: "║",
      "right-mid": "╢",
      middle: "│",
    },
  });

  // Add rows to the table
  table.push(
    ["Operating System", `${osInfo.distro} ${osInfo.release}`],
    [
      "Processor",
      `${cpuInfo.manufacturer} ${cpuInfo.brand} at ${cpuInfo.speed} GHz (${osInfo.arch})`,
    ],
  );

  console.log(table.toString());
}

export const clearConsole = () => {
  process.stdout.write(
    process.platform === "win32" ? "\x1B[2J\x1B[0f" : "\x1B[2J\x1B[3J\x1B[H",
  );
};

export const printBanner = () => {
  const packagePath = process.argv[1].split("/dist")[0] + "/package.json";

  let pkgJSON: Record<string, any>;

  try {
    pkgJSON = JSON.parse(fs.readFileSync(packagePath, "utf8"));
  } catch (err) {
    console.error("Failed to parse package.json", err);
    process.exit(1);
  }

  console.log(
    chalk.green(
      figlet.textSync("SpinUp", {
        font: "Univers",
        horizontalLayout: "default",
        verticalLayout: "fitted",
        width: 80,
        whitespaceBreak: true,
      }),
    ),
  );

  console.log(
    `\n${chalk.dim("Version:")} ${pkgJSON.version} by ${chalk.italic(
      pkgJSON.author.name,
    )} <${chalk.blue.underline(pkgJSON.author.email)}>`,
  );
};

// Function to check if a specific application is installed
export function isInstalled(application: string): CheckResult {
  // Using `which` to check for the application's presence
  const result = shell.which(application);
  return {
    name: application,
    isInstalled: !!result,
  };
}

export function checkApplications() {
  const applications = config.applications.map((app) => app.name);
  const installed: string[] = [];
  const missing: string[] = [];

  applications.forEach((app) => {
    const check = isInstalled(app);
    if (check.isInstalled) {
      installed.push(`${chalk.greenBright("✓")} ${check.name}`);
    } else {
      missing.push(`${chalk.redBright("✗")} ${check.name}`);
    }
  });

  displayInstallationStatus(installed, missing);

  const missingApps = config.applications.filter((app) => {
    const check = isInstalled(app.name);
    return !check.isInstalled;
  });

  return missingApps;
}

function displayInstallationStatus(
  installed: string | any[],
  missing: string | any[],
) {
  const maxLength = Math.max(installed.length, missing.length);
  const table = new Table({
    head: ["Installed", "Missing"],
    colWidths: [20, 20],
  });

  for (let i = 0; i < maxLength; i++) {
    table.push([
      installed[i] || "", // Fallback to an empty string if undefined
      missing[i] || "", // Fallback to an empty string if undefined
    ]);
  }

  console.log(table.toString());
}

export function installApplications(missingApps: Application[]) {
  try {
    if (missingApps.length > 0) {
      console.log("Installing missing applications...");
      missingApps.forEach((app) => {
        if (app.install) {
          console.log(`Installing ${app.name}...`);
          app.install.forEach((distro) => {
            shell.exec(distro.command);
          });
        }
      });
      process.exit(0);
    } else {
      clearConsole();
      console.log(chalk.green("All applications are installed."));
      process.exit(0);
    }
  } catch (err) {
    console.error("Failed to install applications:", err);
    process.exit(1);
  }
}
