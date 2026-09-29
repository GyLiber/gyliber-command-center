function formatObservation(unixSeconds) {
  if (!Number.isInteger(unixSeconds)) return "unknown";
  return new Date(unixSeconds * 1000).toLocaleString();
}

async function refreshState() {
  const status = document.getElementById("state-status");
  const release = document.getElementById("release");
  const schema = document.getElementById("schema");
  const sensitive = document.getElementById("sensitive");
  const freshness = document.getElementById("freshness");
  const observed = document.getElementById("observed");
  const clock = document.getElementById("clock");

  try {
    const response = await fetch("/api/state", {
      credentials: "same-origin",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (response.status === 401) {
      window.location.assign("/login");
      return;
    }

    if (!response.ok) {
      throw new Error(`state request failed: ${response.status}`);
    }

    const state = await response.json();
    release.textContent = state.release ?? "UNKNOWN";
    schema.textContent = state.schema_version ?? "UNKNOWN";
    sensitive.textContent = state.sensitive_data ?? "UNKNOWN";
    freshness.textContent = state.data_freshness ?? "UNKNOWN";
    observed.textContent = formatObservation(state.observed_at_unix);
    clock.textContent = `Browser observation: ${new Date().toLocaleString()}`;
    status.textContent = "SYSTEM OPERATIONAL";
  } catch (error) {
    status.textContent = "STATE UNAVAILABLE";
    console.error("Live state monitor refresh failed", error);
  }
}

refreshState();
window.setInterval(refreshState, 5000);
