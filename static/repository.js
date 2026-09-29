function formatDate(value) {
  if (!value) return "unknown";
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString();
}

function formatObserved(unixSeconds) {
  if (!Number.isInteger(unixSeconds)) return "unknown";
  return new Date(unixSeconds * 1000).toLocaleString();
}

async function refreshRepository() {
  const status = document.getElementById("repo-status");
  const visibility = document.getElementById("visibility");
  const branch = document.getElementById("branch");
  const issues = document.getElementById("issues");
  const community = document.getElementById("community");
  const lastPush = document.getElementById("last-push");
  const observed = document.getElementById("observed");

  try {
    const response = await fetch("/api/repository", {
      credentials: "same-origin",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (response.status === 401) {
      window.location.assign("/login");
      return;
    }

    if (!response.ok) {
      throw new Error(`repository request failed: ${response.status}`);
    }

    const data = await response.json();
    visibility.textContent = data.visibility ?? "UNKNOWN";
    branch.textContent = data.default_branch ?? "UNKNOWN";
    issues.textContent = data.open_issues ?? "0";
    community.textContent = `${data.stars ?? 0} / ${data.forks ?? 0}`;
    lastPush.textContent = data.last_push_at
      ? `Last push: ${formatDate(data.last_push_at)}`
      : "Last push: unknown";
    observed.textContent = `Server observation: ${formatObserved(data.observed_at_unix)} · source: GitHub API`;
    status.textContent = "UPSTREAM CONNECTED";
  } catch (error) {
    status.textContent = "UPSTREAM UNAVAILABLE";
    console.error("Repository monitor refresh failed", error);
  }
}

refreshRepository();
window.setInterval(refreshRepository, 30000);
