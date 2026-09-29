async function refreshState() {
  const seen = document.getElementById("state-seen");
  const api = document.getElementById("api-status");
  const release = document.getElementById("release");
  const data = document.getElementById("data-status");

  try {
    const response = await fetch("/api/state", {
      method: "GET",
      credentials: "same-origin",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (!response.ok) {
      throw new Error(`state request failed: ${response.status}`);
    }

    const state = await response.json();
    api.textContent = "ONLINE";
    release.textContent = state.release ?? "UNKNOWN";
    data.textContent = state.sensitive_data === "disabled" ? "GATED" : "ACTIVE";
    seen.textContent = `observed ${new Date().toLocaleTimeString()}`;
  } catch (error) {
    api.textContent = "DEGRADED";
    seen.textContent = "unavailable";
    console.error("Command Center state refresh failed", error);
  }
}

refreshState();
window.setInterval(refreshState, 15000);
