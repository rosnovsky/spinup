// Function to format bytes to a human-readable format
export function formatBytes(bytes: number, decimals = 2): string {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
}

// Function to format seconds to a human-readable format
export function formatSeconds(seconds: number, decimals = 2): string {
  if (seconds === 0) return "0 Seconds";

  const dm = decimals < 0 ? 0 : decimals;
  const years = Math.floor(seconds / 31536000);
  const months = Math.floor((seconds % 31536000) / 2592000);
  const days = Math.floor((seconds % 2592000) / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secondsLeft = seconds % 60;

  return `${years} year${years > 1 ? "s" : ""} ${months} month${
    months > 1 ? "s" : ""
  } ${days} day${days > 1 ? "s" : ""} ${hours} hour${
    hours > 1 ? "s" : ""
  } ${minutes} minute${minutes > 1 ? "s" : ""} ${secondsLeft.toPrecision(
    dm,
  )} second${secondsLeft > 1 ? "s" : ""}`;
}
