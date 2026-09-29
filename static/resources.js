function createResourceCard(resource) {
  const card = document.createElement("article");
  card.className = "resource-card";

  const kind = document.createElement("span");
  kind.className = "eyebrow";
  kind.textContent = resource.kind;

  const title = document.createElement("h3");
  title.textContent = resource.name;

  const status = document.createElement("p");
  status.className = "muted";
  status.textContent = `${resource.status} · ${resource.visibility}`;

  card.append(kind, title, status);

  if (resource.url) {
    const link = document.createElement("a");
    link.className = "secondary-btn";
    link.href = resource.url;
    link.target = "_blank";
    link.rel = "noopener noreferrer";
    link.textContent = "Open resource ↗";
    card.append(link);
  } else {
    const planned = document.createElement("span");
    planned.className = "muted small";
    planned.textContent = "Resource address not configured";
    card.append(planned);
  }

  return card;
}

async function refreshResources() {
  const grid = document.getElementById("resource-grid");
  const summary = document.getElementById("resource-summary");
  const status = document.getElementById("registry-status");

  try {
    const response = await fetch("/api/resources", {
      credentials: "same-origin",
      cache: "no-store",
      headers: { Accept: "application/json" },
    });

    if (response.status === 401) {
      window.location.assign("/login");
      return;
    }

    if (!response.ok) {
      throw new Error(`resource request failed: ${response.status}`);
    }

    const resources = await response.json();
    grid.replaceChildren(...resources.map(createResourceCard));

    const active = resources.filter((resource) => resource.status === "ACTIVE").length;
    summary.textContent = `${active} active · ${resources.length} registered`;
    status.textContent = "REGISTRY OPERATIONAL";
  } catch (error) {
    status.textContent = "REGISTRY UNAVAILABLE";
    summary.textContent = "unavailable";
    console.error("Resource registry refresh failed", error);
  }
}

refreshResources();
window.setInterval(refreshResources, 30000);
