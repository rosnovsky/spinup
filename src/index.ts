import { prompt } from "enquirer";
import { checkSystem } from "./commands/checkSystem";
import { configureSystem } from "./commands/configureSystem";

interface PromptResponse {
  action: string;
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
    case "check":
      await checkSystem();
      break;
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
