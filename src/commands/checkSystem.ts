import shell from "shelljs";

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
  const applications = ["git", "node", "npm", "docker"]; // Extend this list as needed
  const results = applications.map(isInstalled);

  results.forEach((result) => {
    console.log(
      `${result.name}: ${result.isInstalled ? "Installed" : "Not Installed"}`,
    );
  });
}

export { checkSystem };
