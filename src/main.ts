// OpenIn Modern Desktop Client Engine

export interface AlternateProfile {
  id: string;
  name: string;
  arguments: string;
  icon_path: string | null;
}

export interface BrowserTarget {
  id: string;
  name: string;
  executable_path: string;
  arguments: string | null;
  icon_path: string | null;
  alternate_profiles: AlternateProfile[];
  enabled: boolean;
  priority: number;
}

export interface RuleSet {
  id: string;
  name: string;
  rules: string[];
  target_browser_id: string;
  launch_arguments: string | null;
  enabled: boolean;
  priority: number;
}

export interface Preferences {
  launch_under_mouse: boolean;
  minimize_on_focus_loss: boolean;
  theme: string;
  rule_routing_enabled: boolean;
  window_width: number;
  window_height: number;
  dismiss_after_launch: boolean;
}

export interface Configuration {
  preferences: Preferences;
  browsers: BrowserTarget[];
  rulesets: RuleSet[];
}

export interface AppBootstrapData {
  settings: Configuration;
  discovered_browsers: BrowserTarget[];
  incoming_url: string | null;
}

export interface ValidatedUrl {
  raw: string;
  canonical: string;
  scheme: string;
  host: string | null;
}

export interface RouteDecision {
  ruleset_id: string;
  ruleset_name: string;
  target_browser_id: string;
  launch_arguments: string | null;
  matched_rule: string;
}

// Typed Tauri IPC invocation with standalone browser fallback
async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  const win = window as unknown as { __TAURI__?: { core?: { invoke: (cmd: string, args?: Record<string, unknown>) => Promise<T> } } };
  if (win.__TAURI__?.core?.invoke) {
    return win.__TAURI__.core.invoke(cmd, args);
  }

  console.info(`[IPC Fallback] ${cmd}`, args);
  if (cmd === "get_bootstrap_data") {
    return {
      settings: {
        preferences: {
          launch_under_mouse: false,
          minimize_on_focus_loss: true,
          theme: "auto",
          rule_routing_enabled: true,
          window_width: 540,
          window_height: 320,
          dismiss_after_launch: true,
        },
        browsers: [
          { id: "firefox", name: "Firefox", executable_path: "/usr/bin/firefox", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 1 },
          { id: "chrome", name: "Google Chrome", executable_path: "/usr/bin/google-chrome", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 2 },
          { id: "chromium", name: "Chromium", executable_path: "/usr/bin/chromium", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 3 },
          { id: "brave", name: "Brave", executable_path: "/usr/bin/brave-browser", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 4 },
        ],
        rulesets: [],
      },
      discovered_browsers: [],
      incoming_url: "https://crates.io/crates/tauri",
    } as unknown as T;
  }
  if (cmd === "sanitize_url") {
    const raw = (args.url as string) || "";
    return {
      raw,
      canonical: raw.startsWith("http") ? raw : `https://${raw}`,
      scheme: "https",
      host: "example.com",
    } as unknown as T;
  }
  return undefined as unknown as T;
}

class OpenInApplication {
  private config: Configuration = {
    preferences: {
      launch_under_mouse: false,
      minimize_on_focus_loss: true,
      theme: "auto",
      rule_routing_enabled: true,
      window_width: 540,
      window_height: 320,
      dismiss_after_launch: true,
    },
    browsers: [],
    rulesets: [],
  };

  private currentUrl = "";
  private selectedTargetIndex = 0;
  private isSettingsModalOpen = false;

  // DOM Elements
  private chooserView = document.getElementById("chooser-view") as HTMLElement;
  private settingsView = document.getElementById("settings-view") as HTMLElement;
  private urlInput = document.getElementById("url-input") as HTMLInputElement;
  private browserGrid = document.getElementById("browser-grid") as HTMLElement;
  private statusToast = document.getElementById("status-toast") as HTMLElement;
  private btnCopyUrl = document.getElementById("btn-copy-url") as HTMLButtonElement;
  private btnOpenSettings = document.getElementById("btn-open-settings") as HTMLButtonElement;
  private btnCloseSettings = document.getElementById("btn-close-settings") as HTMLButtonElement;
  private btnRefreshBrowsers = document.getElementById("btn-refresh-browsers") as HTMLButtonElement;
  private settingsBrowserList = document.getElementById("settings-browser-list") as HTMLElement;
  private settingsRulesList = document.getElementById("settings-rules-list") as HTMLElement;
  private prefTheme = document.getElementById("pref-theme") as HTMLSelectElement;

  async init() {
    this.registerEventHandlers();

    try {
      const data = await invokeTauri<AppBootstrapData>("get_bootstrap_data");
      this.config = data.settings;
      this.currentUrl = data.incoming_url || "";
      this.urlInput.value = this.currentUrl;

      this.applyTheme(this.config.preferences.theme);
      this.renderBrowserGrid();
      this.renderSettings();

      // Check if automatic routing rule applies immediately
      if (this.currentUrl && this.config.preferences.rule_routing_enabled) {
        try {
          const decision = await invokeTauri<RouteDecision | null>("evaluate_and_route", {
            url: this.currentUrl,
          });
          if (decision) {
            this.showToast(`Auto-routed to ${decision.target_browser_id} (${decision.ruleset_name})`);
          }
        } catch (err) {
          console.warn("Auto routing evaluation notice:", err);
        }
      }
    } catch (e) {
      console.error("Initialization failure:", e);
      this.showToast("Failed to load application settings");
    }
  }

  private registerEventHandlers() {
    window.addEventListener("keydown", (e) => this.handleKeyboardShortcut(e));

    this.urlInput.addEventListener("input", () => {
      this.currentUrl = this.urlInput.value.trim();
    });

    this.btnCopyUrl.addEventListener("click", () => this.copyUrlToClipboard());
    this.btnOpenSettings.addEventListener("click", () => this.toggleSettings(true));
    this.btnCloseSettings.addEventListener("click", () => this.toggleSettings(false));

    document.querySelectorAll(".tab-btn").forEach((btn) => {
      btn.addEventListener("click", (e) => {
        const tab = (e.currentTarget as HTMLElement).dataset.tab;
        if (tab) this.selectTab(tab);
      });
    });

    this.btnRefreshBrowsers.addEventListener("click", async () => {
      try {
        const discovered = await invokeTauri<BrowserTarget[]>("refresh_browsers");
        this.config.browsers = discovered;
        await this.persistSettings();
        this.renderBrowserGrid();
        this.renderSettings();
        this.showToast("System browsers refreshed");
      } catch (err) {
        this.showToast("Discovery refresh error");
      }
    });

    this.prefTheme.addEventListener("change", () => {
      this.config.preferences.theme = this.prefTheme.value;
      this.applyTheme(this.config.preferences.theme);
      this.persistSettings();
    });
  }

  private handleKeyboardShortcut(e: KeyboardEvent) {
    const isInputActive = document.activeElement === this.urlInput;

    if (e.key === "Escape") {
      e.preventDefault();
      if (this.isSettingsModalOpen) {
        this.toggleSettings(false);
      } else {
        invokeTauri("dismiss_window").catch(() => {});
      }
      return;
    }

    if (!isInputActive && !this.isSettingsModalOpen) {
      // 1-9 direct hotkey launch
      if (e.key >= "1" && e.key <= "9") {
        const index = parseInt(e.key, 10) - 1;
        const enabledTargets = this.getEnabledTargets();
        if (index < enabledTargets.length) {
          e.preventDefault();
          this.launchTarget(enabledTargets[index]);
          return;
        }
      }

      // C: Copy clean URL
      if ((e.key === "c" || e.key === "C") && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        this.copyUrlToClipboard();
        return;
      }

      // E: Focus URL field
      if ((e.key === "e" || e.key === "E") && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        this.urlInput.focus();
        this.urlInput.select();
        return;
      }

      // , : Toggle Settings
      if (e.key === ",") {
        e.preventDefault();
        this.toggleSettings(true);
        return;
      }

      // Arrow navigation
      const targets = this.getEnabledTargets();
      if (targets.length > 0) {
        if (e.key === "ArrowRight" || e.key === "ArrowDown") {
          e.preventDefault();
          this.selectedTargetIndex = (this.selectedTargetIndex + 1) % targets.length;
          this.updateTargetFocus();
          return;
        }
        if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
          e.preventDefault();
          this.selectedTargetIndex = (this.selectedTargetIndex - 1 + targets.length) % targets.length;
          this.updateTargetFocus();
          return;
        }
        if (e.key === "Enter") {
          e.preventDefault();
          this.launchTarget(targets[this.selectedTargetIndex]);
          return;
        }
      }
    }

    if (isInputActive && e.key === "Enter") {
      e.preventDefault();
      const targets = this.getEnabledTargets();
      if (targets.length > 0) {
        this.launchTarget(targets[this.selectedTargetIndex]);
      }
    }
  }

  private getEnabledTargets(): BrowserTarget[] {
    return this.config.browsers
      .filter((b) => b.enabled)
      .sort((a, b) => a.priority - b.priority);
  }

  private renderBrowserGrid() {
    this.browserGrid.innerHTML = "";
    const targets = this.getEnabledTargets();

    if (targets.length === 0) {
      this.browserGrid.innerHTML = `<div class="empty-state">No browsers registered. Open settings to scan for applications.</div>`;
      return;
    }

    targets.forEach((target, index) => {
      const card = document.createElement("div");
      card.className = "browser-card" + (index === this.selectedTargetIndex ? " selected" : "");
      card.setAttribute("role", "option");
      card.setAttribute("tabindex", "0");
      card.dataset.id = target.id;

      const keyBadge = index < 9 ? `<span class="browser-key-badge">${index + 1}</span>` : "";

      const iconElement = target.icon_path
        ? `<img class="browser-icon-img" src="${target.icon_path}" alt="${target.name}" onerror="this.style.display='none'; this.nextElementSibling.style.display='flex';" /><div class="browser-icon-fallback" style="display:none;">${target.name.charAt(0)}</div>`
        : `<div class="browser-icon-fallback">${target.name.charAt(0)}</div>`;

      card.innerHTML = `
        ${keyBadge}
        <div class="browser-icon-wrapper">
          ${iconElement}
        </div>
        <span class="browser-name">${target.name}</span>
      `;

      card.addEventListener("click", () => {
        this.selectedTargetIndex = index;
        this.updateTargetFocus();
        this.launchTarget(target);
      });

      this.browserGrid.appendChild(card);
    });
  }

  private updateTargetFocus() {
    const cards = this.browserGrid.querySelectorAll(".browser-card");
    cards.forEach((card, idx) => {
      if (idx === this.selectedTargetIndex) {
        card.classList.add("selected");
        (card as HTMLElement).focus();
      } else {
        card.classList.remove("selected");
      }
    });
  }

  private async launchTarget(target: BrowserTarget, profileId?: string) {
    const url = this.urlInput.value.trim() || this.currentUrl;
    if (!url) {
      this.showToast("Please enter a valid link address");
      return;
    }

    try {
      await invokeTauri("launch_browser", {
        browserId: target.id,
        url,
        alternateId: profileId || null,
      });
      this.showToast(`Launched ${target.name}`);
    } catch (err) {
      this.showToast(`Launch failed: ${err}`);
    }
  }

  private async copyUrlToClipboard() {
    const url = this.urlInput.value.trim() || this.currentUrl;
    if (url) {
      await navigator.clipboard.writeText(url);
      this.showToast("Address copied to clipboard");
    }
  }

  private toggleSettings(open: boolean) {
    this.isSettingsModalOpen = open;
    if (open) {
      this.chooserView.classList.remove("active");
      this.chooserView.hidden = true;
      this.settingsView.classList.add("active");
      this.settingsView.hidden = false;
      this.renderSettings();
    } else {
      this.settingsView.classList.remove("active");
      this.settingsView.hidden = true;
      this.chooserView.classList.add("active");
      this.chooserView.hidden = false;
      this.renderBrowserGrid();
    }
  }

  private selectTab(tabId: string) {
    document.querySelectorAll(".tab-btn").forEach((b) => b.classList.remove("active"));
    document.querySelectorAll(".tab-content").forEach((c) => c.classList.remove("active"));

    const btn = document.querySelector(`[data-tab="${tabId}"]`);
    const content = document.getElementById(tabId);
    if (btn) btn.classList.add("active");
    if (content) content.classList.add("active");
  }

  private renderSettings() {
    this.prefTheme.value = this.config.preferences.theme;

    // Render registered browsers
    this.settingsBrowserList.innerHTML = "";
    this.config.browsers.forEach((target) => {
      const row = document.createElement("div");
      row.className = "browser-config-item";
      row.style.display = "flex";
      row.style.alignItems = "center";
      row.style.justifyContent = "space-between";
      row.style.padding = "8px 0";
      row.style.borderBottom = "1px solid var(--border-color)";

      row.innerHTML = `
        <div style="display:flex; align-items:center; gap:8px;">
          <input type="checkbox" ${target.enabled ? "checked" : ""} class="browser-enable-check" />
          <span>${target.name}</span>
          <span style="font-size:11px; color:var(--text-muted);">${target.executable_path}</span>
        </div>
      `;

      const check = row.querySelector(".browser-enable-check") as HTMLInputElement;
      check.addEventListener("change", async () => {
        target.enabled = check.checked;
        await this.persistSettings();
      });

      this.settingsBrowserList.appendChild(row);
    });

    // Render rulesets
    this.settingsRulesList.innerHTML = "";
    if (this.config.rulesets.length === 0) {
      this.settingsRulesList.innerHTML = `<div style="font-size:12px; color:var(--text-muted); padding:8px 0;">No routing rules configured. Create a rule to automatically direct URLs to specific browsers.</div>`;
    } else {
      this.config.rulesets.forEach((r) => {
        const item = document.createElement("div");
        item.style.padding = "8px 0";
        item.style.borderBottom = "1px solid var(--border-color)";
        item.innerHTML = `
          <div style="font-weight:500; font-size:13px;">${r.name} ➔ ${r.target_browser_id}</div>
          <div style="font-size:11px; color:var(--text-secondary);">${r.rules.join(", ")}</div>
        `;
        this.settingsRulesList.appendChild(item);
      });
    }
  }

  private async persistSettings() {
    try {
      await invokeTauri("save_settings", { settings: this.config });
    } catch (e) {
      console.error("Settings persistence error:", e);
    }
  }

  private applyTheme(theme: string) {
    document.documentElement.setAttribute("data-theme", theme);
  }

  private showToast(msg: string) {
    this.statusToast.textContent = msg;
    this.statusToast.style.opacity = "1";
    setTimeout(() => {
      this.statusToast.style.opacity = "0";
    }, 2500);
  }
}

document.addEventListener("DOMContentLoaded", () => {
  const app = new OpenInApplication();
  app.init();
});
