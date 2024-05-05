import shell from "shelljs";
import config from "../config.json";
import chalk from "chalk";

interface CheckResult {
  name: string;
  isInstalled: boolean;
}

// Function to check if a specific application is installed
function isInstalled(application: string): CheckResult {
  // Using `which` to check for the application's presence
  const result = shell.which(application);
  return {
    name: application,
    isInstalled: !!result,
  };
}

// Function to perform all checks
async function checkSystem(): Promise<void> {
  const applications = config.applications.map((app) => app.name);
  const results = applications.map(isInstalled);

  return results.forEach((result) => {
    console.log(
      `${result.isInstalled ? chalk.greenBright("✓") : chalk.redBright("✗")} ${
        result.name
      }`,
    );
  });
}

export { checkSystem };
