import { prompt } from "enquirer";
import { checkSystem } from "./commands/checkSystem";
import { configureSystem } from "./commands/configureSystem";
import si from "systeminformation";
import { formatBytes, formatSeconds } from "./utils/helpers";
import chalk from "chalk";
import Table from "cli-table3";

interface PromptResponse {
  action: string;
}

async function displaySystemInfo() {
  const osInfo = await si.osInfo();
  const cpuInfo = await si.cpu();
  const memInfo = await si.mem();
  const diskInfo = (await si.diskLayout())[0];
  const shellInfo = await si.shell();
  const userInfo = (await si.users())[0];
  const uptimeInfo = si.time().uptime;

  // Create a new table with customized styling
  const table = new Table({
    head: [chalk.green("System"), chalk.green("Information")],
    colWidths: [25, 50], // Set custom column widths
    style: {
      head: [], // Removes default coloring
      border: [], // Optional: you can specify border colors if you like
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
      `${cpuInfo.manufacturer} ${cpuInfo.brand} at ${cpuInfo.speed} GHz`,
    ],
    ["Memory", formatBytes(memInfo.total)],
    ["Disk", formatBytes(diskInfo.size)],
    ["Shell", shellInfo],
    ["User", userInfo.user],
    ["Uptime", formatSeconds(uptimeInfo)],
  );

  // Log the table to the console
  console.log(table.toString());
}

async function mainMenu() {
  const response = await prompt<PromptResponse>({
    type: "select",
    name: "action",
    message: "What would you like to do?",
    choices: [
      { name: "Check system", value: "check" },
      { name: "Configure system", value: "configure" },
      { name: "Exit", value: "exit" },
    ],
  });

  switch (response.action) {
    case "Check system":
      await displaySystemInfo();
      await checkSystem();
    case "configure":
      await configureSystem();
      break;
    case "exit":
      console.log("Exiting...");
      process.exit();
  }

  // Optionally loop back to the main menu
  await mainMenu();
}

mainMenu().catch((err) => {
  console.error("An error occurred:", err);
  process.exit(1);
});
