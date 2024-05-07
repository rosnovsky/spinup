import { checkSystem } from "./commands/checkSystem";
import {
  clearConsole,
  displaySystemInfo,
  installApplications,
  printBanner,
} from "./utils/helpers";

async function mainMenu() {
  clearConsole();
  printBanner();
  await displaySystemInfo();
  const missingApps = checkSystem();
  await installApplications(missingApps);
  // TODO: Add font installation
  // await installFonts();
}

mainMenu().catch((err) => {
  console.error("An error occurred:", err);
  process.exit(1);
});
