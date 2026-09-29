async function refreshModules() {
  const grid = document.getElementById("module-grid");
  const summary = document.getElementById("module-summary");
  if (!grid || !summary) return;

  try {
    const response = await fetch("/api/modules", {
      method: "GET",
      credentials: "same-origin",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (!response.ok) {
      throw new Error(`module request failed: ${response.status}`);
    }

    const modules = await response.json();
    grid.replaceChildren();

    for (const [index, module] of modules.entries()) {
      const card = document.createElement(module.status === "ACTIVE" ? "a" : "article");
      card.className = module.status === "ACTIVE" ? "module" : "module locked";

      if (module.status === "ACTIVE") {
        card.href = module.path;
      }

      const ordinal = document.createElement("span");
      ordinal.textContent = String(index + 1).padStart(2, "0");

      const title = document.createElement("h3");
      title.textContent = module.name;

      const description = document.createElement("p");
      description.textContent = module.status === "ACTIVE"
        ? `Live module · ${module.classification}`
        : `Reserved · ${module.classification}`;

      card.append(ordinal, title, description);
      grid.append(card);
    }

    const active = modules.filter((module) => module.status === "ACTIVE").length;
    summary.textContent = `${active} active · ${modules.length} registered`;
  } catch (error) {
    summary.textContent = "registry unavailable";
    console.error("Command Center module refresh failed", error);
  }
}

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
refreshModules();
window.setInterval(refreshState, 15000);
window.setInterval(refreshModules, 30000);
