import { Application } from "../types";
import { checkApplications } from "../utils/helpers";

// Function to perform all checks
function checkSystem(): Application[] {
  return checkApplications();
}

export { checkSystem };
