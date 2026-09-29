async function refreshLiveStatus() {
  const status = document.getElementById("live-status");
  const signal = document.getElementById("live-signal");
  if (!status || !signal) return;

  try {
    const response = await fetch("/api/health", {
      method: "GET",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (!response.ok) {
      throw new Error(`health request failed: ${response.status}`);
    }

    const health = await response.json();
    status.textContent = health.status === "ok"
      ? `LIVE STATUS / ${health.version}`
      : "LIVE STATUS / DEGRADED";
    document.body.classList.toggle("live-ok", health.status === "ok");
  } catch (error) {
    status.textContent = "LIVE STATUS / UNAVAILABLE";
    document.body.classList.remove("live-ok");
    console.error("Public live status refresh failed", error);
  }
}

refreshLiveStatus();
window.setInterval(refreshLiveStatus, 30000);
