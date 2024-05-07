import { prompt } from "enquirer";
import { checkSystem } from "./commands/checkSystem";
import { configureSystem } from "./commands/configureSystem";
import {
  clearConsole,
  displaySystemInfo,
  installApplications,
  printBanner,
} from "./utils/helpers";

interface PromptResponse {
  action: string;
}

async function mainMenu() {
  clearConsole();
  printBanner();
  await displaySystemInfo();
  const missingApps = checkSystem();
  await installApplications(missingApps);

  // const response = await prompt<PromptResponse>({
  //   type: "select",
  //   name: "action",
  //   message: "What would you like to do?",
  //   choices: [
  //     { name: "Configure system", value: "configure" },
  //     { name: "Exit", value: "exit" },
  //   ],
  // });

  // switch (response.action) {
  //   case "Configure system":
  //     break;
  //   case "Exit":
  //     console.log("Exiting...");
  //     process.exit();
  // }

  // Optionally loop back to the main menu
  // await mainMenu();
}

mainMenu().catch((err) => {
  console.error("An error occurred:", err);
  process.exit(1);
});
