// OpenIn — Modern Desktop Client Engine

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
  onboarding_completed: boolean;
  onboarding_step: number;
  preferred_browser_id: string | null;
  fallback_behavior: string;
}

export interface Configuration {
  preferences: Preferences;
  browsers: BrowserTarget[];
  rulesets: RuleSet[];
}

export interface HandlerStatus {
  is_default: boolean;
  current_handler: string | null;
  os_type: string;
  can_auto_register: boolean;
  details: string;
}

export interface RegistrationResult {
  success: boolean;
  message: string;
  verified: boolean;
}

export interface AppBootstrapData {
  settings: Configuration;
  discovered_browsers: BrowserTarget[];
  incoming_url: string | null;
  handler_status: HandlerStatus;
}

export interface RouteSimulationResult {
  valid: boolean;
  sanitized_url: string;
  matched_rule: string | null;
  target_browser_id: string | null;
  target_browser_name: string | null;
}

// Typed Tauri IPC invocation with standalone browser fallback
async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  const win = window as unknown as {
    __TAURI__?: { core?: { invoke: (cmd: string, args?: Record<string, unknown>) => Promise<T> } };
  };

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
          window_height: 360,
          dismiss_after_launch: true,
          onboarding_completed: false,
          onboarding_step: 1,
          preferred_browser_id: null,
          fallback_behavior: "always_ask",
        },
        browsers: [
          { id: "firefox", name: "Firefox", executable_path: "/usr/bin/firefox", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 1 },
          { id: "chrome", name: "Google Chrome", executable_path: "/usr/bin/google-chrome", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 2 },
          { id: "brave", name: "Brave Browser", executable_path: "/usr/bin/brave-browser", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 3 },
          { id: "edge", name: "Microsoft Edge", executable_path: "/usr/bin/microsoft-edge", arguments: null, icon_path: null, alternate_profiles: [], enabled: true, priority: 4 },
        ],
        rulesets: [],
      },
      discovered_browsers: [],
      incoming_url: "https://github.com/MossRune/OpenIn",
      handler_status: {
        is_default: false,
        current_handler: "firefox.desktop",
        os_type: "linux",
        can_auto_register: true,
        details: "Current default browser handler is 'firefox.desktop'.",
      },
    } as unknown as T;
  }

  if (cmd === "get_default_handler_status") {
    return {
      is_default: false,
      current_handler: "firefox.desktop",
      os_type: "linux",
      can_auto_register: true,
      details: "Current default browser handler is 'firefox.desktop'.",
    } as unknown as T;
  }

  if (cmd === "register_default_handler") {
    return {
      success: true,
      message: "OpenIn successfully registered and verified as the default browser.",
      verified: true,
    } as unknown as T;
  }

  if (cmd === "test_route_simulation") {
    return {
      valid: true,
      sanitized_url: (args.url as string) || "https://github.com/MossRune/OpenIn",
      matched_rule: null,
      target_browser_id: "firefox",
      target_browser_name: "Firefox",
    } as unknown as T;
  }

  return undefined as unknown as T;
}

// Vector SVG logos for browsers
function getBrowserIconSvg(id: string, name: string): string {
  const key = (id + " " + name).toLowerCase();

  if (key.includes("chrome")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="20" fill="#f8fafc" opacity="0.08"/><circle cx="24" cy="24" r="18" fill="none" stroke="#ea4335" stroke-width="6"/><path d="M24 15a9 9 0 1 0 9 9" fill="none" stroke="#4285f4" stroke-width="6"/><circle cx="24" cy="24" r="7" fill="#4285f4"/><circle cx="24" cy="24" r="9" fill="none" stroke="#34a853" stroke-width="4"/><circle cx="24" cy="24" r="5" fill="#fbbc05"/></svg>`;
  }
  if (key.includes("firefox")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="url(#ff-grad)"/><path d="M24 8c-8 0-14 6-14 14 0 7 5 12 11 13 3 .5 6-.5 8-2 3-2 5-6 4-10-1 2-2 3-4 4 1-3 0-7-2-9-1 2-2 3-3 4 0-4-3-7-6-8 2 2 3 5 2 8-2-1-3-3-3-5-2 3-3 7-2 10 1 5 6 9 11 9 7 0 13-5 13-12 0-7-5-13-11-13z" fill="#fff" opacity="0.95"/><defs><linearGradient id="ff-grad" x1="0" y1="0" x2="1" y2="1"><stop offset="0%" stop-color="#ff7139"/><stop offset="100%" stop-color="#902c89"/></linearGradient></defs></svg>`;
  }
  if (key.includes("safari")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="#007aff"/><circle cx="24" cy="24" r="16" fill="none" stroke="#fff" stroke-width="1.5" opacity="0.5"/><polygon points="21,21 31,17 27,27 17,31" fill="#ff3b30"/><polygon points="21,21 17,31 27,27" fill="#f8fafc"/><circle cx="24" cy="24" r="1.5" fill="#fff"/></svg>`;
  }
  if (key.includes("edge")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="url(#edge-grad)"/><path d="M24 12c-6 0-11 4-11 10 0 5 4 8 8 8 5 0 8-3 9-6-1 0-3 1-5 1-4 0-6-3-6-6 0-4 3-7 5-7z" fill="#fff" opacity="0.9"/><defs><linearGradient id="edge-grad" x1="0" y1="0" x2="1" y2="1"><stop offset="0%" stop-color="#0078d7"/><stop offset="100%" stop-color="#00c853"/></linearGradient></defs></svg>`;
  }
  if (key.includes("brave")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><path d="M24 6l13 5v11c0 9-6 17-13 20-7-3-13-11-13-20V11l13-5z" fill="#ff3800"/><path d="M24 10l9 3v8c0 6-4 12-9 14-5-2-9-8-9-14v-8l9-3z" fill="#fb542b"/><circle cx="24" cy="23" r="4" fill="#ffffff"/></svg>`;
  }
  if (key.includes("arc")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="url(#arc-grad)"/><path d="M16 28c0-5.5 3.6-10 8-10s8 4.5 8 10" fill="none" stroke="#ffffff" stroke-width="3" stroke-linecap="round"/><circle cx="24" cy="18" r="3" fill="#ffffff"/><defs><linearGradient id="arc-grad" x1="0" y1="0" x2="1" y2="1"><stop offset="0%" stop-color="#ff5983"/><stop offset="50%" stop-color="#7a5af8"/><stop offset="100%" stop-color="#00d2ff"/></linearGradient></defs></svg>`;
  }
  if (key.includes("opera")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="#ff1b2d"/><ellipse cx="24" cy="24" rx="7" ry="12" fill="#090b10"/></svg>`;
  }
  if (key.includes("vivaldi")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="#ef3939"/><path d="M17 18h4l3 9 3-9h4l-5 13h-4l-5-13z" fill="#ffffff"/></svg>`;
  }
  if (key.includes("zen")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="url(#zen-grad)"/><circle cx="24" cy="24" r="8" fill="none" stroke="#ffffff" stroke-width="2.5"/><circle cx="24" cy="24" r="3" fill="#ffffff"/><defs><linearGradient id="zen-grad" x1="0" y1="0" x2="1" y2="1"><stop offset="0%" stop-color="#06b6d4"/><stop offset="100%" stop-color="#8b5cf6"/></linearGradient></defs></svg>`;
  }
  if (key.includes("tor")) {
    return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="#7d4698"/><path d="M24 10a14 14 0 0 1 0 28" fill="none" stroke="#ffffff" stroke-width="2.5"/><path d="M24 15a9 9 0 0 1 0 18" fill="none" stroke="#ffffff" stroke-width="2.5"/><circle cx="24" cy="24" r="3" fill="#ffffff"/></svg>`;
  }

  // Generic Modern Browser Compass
  return `<svg class="browser-icon-svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="18" fill="url(#gen-grad)"/><circle cx="24" cy="24" r="15" fill="none" stroke="#ffffff" stroke-width="1.5" opacity="0.4"/><polygon points="21,21 31,17 27,27 17,31" fill="#ffffff"/><defs><linearGradient id="gen-grad" x1="0" y1="0" x2="1" y2="1"><stop offset="0%" stop-color="#6366f1"/><stop offset="100%" stop-color="#a855f7"/></linearGradient></defs></svg>`;
}

class OpenInApplication {
  private config: Configuration = {
    preferences: {
      launch_under_mouse: false,
      minimize_on_focus_loss: true,
      theme: "auto",
      rule_routing_enabled: true,
      window_width: 540,
      window_height: 360,
      dismiss_after_launch: true,
      onboarding_completed: false,
      onboarding_step: 1,
      preferred_browser_id: null,
      fallback_behavior: "always_ask",
    },
    browsers: [],
    rulesets: [],
  };

  private handlerStatus: HandlerStatus = {
    is_default: false,
    current_handler: null,
    os_type: "linux",
    can_auto_register: true,
    details: "Checking handler status...",
  };

  private currentUrl = "";
  private selectedTargetIndex = 0;
  private currentOnboardingStep = 1;

  // DOM Elements: Views
  private onboardingView!: HTMLElement;
  private chooserView!: HTMLElement;
  private settingsView!: HTMLElement;
  private ruleModal!: HTMLElement;

  // Chooser view elements
  private urlInput!: HTMLInputElement;
  private browserGrid!: HTMLElement;
  private btnCopyUrl!: HTMLButtonElement;
  private btnOpenSettings!: HTMLButtonElement;
  private statusToast!: HTMLElement;
  private handlerAlertBanner!: HTMLElement;
  private btnFixHandler!: HTMLButtonElement;

  // Onboarding wizard elements
  private btnWizardBack!: HTMLButtonElement;
  private btnWizardNext!: HTMLButtonElement;
  private btnWizardFinish!: HTMLButtonElement;
  private onboardingBrowserCount!: HTMLElement;
  private onboardingBrowserList!: HTMLElement;
  private btnOnboardingRescan!: HTMLButtonElement;
  private handlerStatusBullet!: HTMLElement;
  private handlerStatusTitle!: HTMLElement;
  private handlerStatusBadge!: HTMLElement;
  private handlerStatusDetail!: HTMLElement;
  private btnRegisterHandler!: HTMLButtonElement;
  private btnOpenOsSettings!: HTMLButtonElement;
  private btnReverifyHandler!: HTMLButtonElement;
  private onboardingPreferredSelect!: HTMLSelectElement;
  private summaryHandlerStatus!: HTMLElement;
  private summaryBrowserCount!: HTMLElement;
  private summaryFallbackMode!: HTMLElement;
  private summaryRulesCount!: HTMLElement;
  private simulationTestUrl!: HTMLInputElement;
  private btnRunSimulation!: HTMLButtonElement;
  private simulationResult!: HTMLElement;

  // Settings elements
  private btnCloseSettings!: HTMLButtonElement;
  private settingsBrowserList!: HTMLElement;
  private settingsRulesList!: HTMLElement;
  private prefRuleMatching!: HTMLInputElement;
  private prefCloseAfterLaunch!: HTMLInputElement;
  private prefMinimizeOnBlur!: HTMLInputElement;
  private prefTheme!: HTMLSelectElement;
  private btnRefreshBrowsers!: HTMLButtonElement;
  private settingsHandlerBadge!: HTMLElement;
  private settingsHandlerDesc!: HTMLElement;
  private btnSettingsRegisterHandler!: HTMLButtonElement;
  private btnSettingsOpenOs!: HTMLButtonElement;
  private btnSettingsRefreshHandler!: HTMLButtonElement;
  private btnRerunOnboarding!: HTMLButtonElement;

  // Rule Modal elements
  private btnOpenAddRuleModal!: HTMLButtonElement;
  private btnCloseRuleModal!: HTMLButtonElement;
  private btnCancelRule!: HTMLButtonElement;
  private btnSaveRule!: HTMLButtonElement;
  private ruleNameInput!: HTMLInputElement;
  private rulePatternInput!: HTMLInputElement;
  private ruleTargetSelect!: HTMLSelectElement;

  public async init() {
    this.bindDomElements();
    this.attachEventListeners();

    try {
      const data = await invokeTauri<AppBootstrapData>("get_bootstrap_data");
      this.config = data.settings;
      this.handlerStatus = data.handler_status;
      if (data.incoming_url) {
        this.currentUrl = data.incoming_url;
        this.urlInput.value = data.incoming_url;
      }
    } catch (err) {
      console.warn("Bootstrap IPC notice:", err);
    }

    this.applyTheme(this.config.preferences.theme);

    // First-run onboarding check
    if (!this.config.preferences.onboarding_completed) {
      this.currentOnboardingStep = Math.max(1, Math.min(5, this.config.preferences.onboarding_step || 1));
      this.showView("onboarding");
      this.renderOnboardingStep(this.currentOnboardingStep);
      this.updateHandlerStatusUi();
    } else {
      this.showView("chooser");
      this.renderBrowserGrid();
      this.checkHandlerAlert();
    }
  }

  private bindDomElements() {
    this.onboardingView = document.getElementById("onboarding-view")!;
    this.chooserView = document.getElementById("chooser-view")!;
    this.settingsView = document.getElementById("settings-view")!;
    this.ruleModal = document.getElementById("rule-modal")!;

    // Chooser
    this.urlInput = document.getElementById("url-input") as HTMLInputElement;
    this.browserGrid = document.getElementById("browser-grid")!;
    this.btnCopyUrl = document.getElementById("btn-copy-url") as HTMLButtonElement;
    this.btnOpenSettings = document.getElementById("btn-open-settings") as HTMLButtonElement;
    this.statusToast = document.getElementById("status-toast")!;
    this.handlerAlertBanner = document.getElementById("handler-alert-banner")!;
    this.btnFixHandler = document.getElementById("btn-fix-handler") as HTMLButtonElement;

    // Wizard
    this.btnWizardBack = document.getElementById("btn-wizard-back") as HTMLButtonElement;
    this.btnWizardNext = document.getElementById("btn-wizard-next") as HTMLButtonElement;
    this.btnWizardFinish = document.getElementById("btn-wizard-finish") as HTMLButtonElement;
    this.onboardingBrowserCount = document.getElementById("onboarding-browser-count")!;
    this.onboardingBrowserList = document.getElementById("onboarding-browser-list")!;
    this.btnOnboardingRescan = document.getElementById("btn-onboarding-rescan") as HTMLButtonElement;
    this.handlerStatusBullet = document.getElementById("handler-status-bullet")!;
    this.handlerStatusTitle = document.getElementById("handler-status-title")!;
    this.handlerStatusBadge = document.getElementById("handler-status-badge")!;
    this.handlerStatusDetail = document.getElementById("handler-status-detail")!;
    this.btnRegisterHandler = document.getElementById("btn-register-handler") as HTMLButtonElement;
    this.btnOpenOsSettings = document.getElementById("btn-open-os-settings") as HTMLButtonElement;
    this.btnReverifyHandler = document.getElementById("btn-reverify-handler") as HTMLButtonElement;
    this.onboardingPreferredSelect = document.getElementById("onboarding-preferred-browser") as HTMLSelectElement;
    this.summaryHandlerStatus = document.getElementById("summary-handler-status")!;
    this.summaryBrowserCount = document.getElementById("summary-browser-count")!;
    this.summaryFallbackMode = document.getElementById("summary-fallback-mode")!;
    this.summaryRulesCount = document.getElementById("summary-rules-count")!;
    this.simulationTestUrl = document.getElementById("simulation-test-url") as HTMLInputElement;
    this.btnRunSimulation = document.getElementById("btn-run-simulation") as HTMLButtonElement;
    this.simulationResult = document.getElementById("simulation-result")!;

    // Settings
    this.btnCloseSettings = document.getElementById("btn-close-settings") as HTMLButtonElement;
    this.settingsBrowserList = document.getElementById("settings-browser-list")!;
    this.settingsRulesList = document.getElementById("settings-rules-list")!;
    this.prefRuleMatching = document.getElementById("pref-rule-matching") as HTMLInputElement;
    this.prefCloseAfterLaunch = document.getElementById("pref-close-after-launch") as HTMLInputElement;
    this.prefMinimizeOnBlur = document.getElementById("pref-minimize-on-blur") as HTMLInputElement;
    this.prefTheme = document.getElementById("pref-theme") as HTMLSelectElement;
    this.btnRefreshBrowsers = document.getElementById("btn-refresh-browsers") as HTMLButtonElement;
    this.settingsHandlerBadge = document.getElementById("settings-handler-badge")!;
    this.settingsHandlerDesc = document.getElementById("settings-handler-desc")!;
    this.btnSettingsRegisterHandler = document.getElementById("btn-settings-register-handler") as HTMLButtonElement;
    this.btnSettingsOpenOs = document.getElementById("btn-settings-open-os") as HTMLButtonElement;
    this.btnSettingsRefreshHandler = document.getElementById("btn-settings-refresh-handler") as HTMLButtonElement;
    this.btnRerunOnboarding = document.getElementById("btn-rerun-onboarding") as HTMLButtonElement;

    // Rule modal
    this.btnOpenAddRuleModal = document.getElementById("btn-open-add-rule-modal") as HTMLButtonElement;
    this.btnCloseRuleModal = document.getElementById("btn-close-rule-modal") as HTMLButtonElement;
    this.btnCancelRule = document.getElementById("btn-cancel-rule") as HTMLButtonElement;
    this.btnSaveRule = document.getElementById("btn-save-rule") as HTMLButtonElement;
    this.ruleNameInput = document.getElementById("rule-name-input") as HTMLInputElement;
    this.rulePatternInput = document.getElementById("rule-pattern-input") as HTMLInputElement;
    this.ruleTargetSelect = document.getElementById("rule-target-select") as HTMLSelectElement;
  }

  private attachEventListeners() {
    window.addEventListener("keydown", (e) => this.handleKeyboardShortcut(e));

    // Chooser
    this.urlInput.addEventListener("input", () => {
      this.currentUrl = this.urlInput.value.trim();
    });
    this.btnCopyUrl.addEventListener("click", () => this.copyUrlToClipboard());
    this.btnOpenSettings.addEventListener("click", () => this.toggleSettings(true));
    this.btnFixHandler.addEventListener("click", () => this.toggleSettings(true));

    // Onboarding wizard
    this.btnWizardBack.addEventListener("click", () => this.navigateWizard(-1));
    this.btnWizardNext.addEventListener("click", () => this.navigateWizard(1));
    this.btnWizardFinish.addEventListener("click", () => this.finishOnboarding());
    this.btnOnboardingRescan.addEventListener("click", () => this.refreshDiscoveredBrowsers());
    this.btnRegisterHandler.addEventListener("click", () => this.registerDefaultProtocolHandler());
    this.btnOpenOsSettings.addEventListener("click", () => this.openOsSettings());
    this.btnReverifyHandler.addEventListener("click", () => this.reverifyProtocolHandler());
    this.btnRunSimulation.addEventListener("click", () => this.runSimulationTest());

    // Settings
    this.btnCloseSettings.addEventListener("click", () => this.toggleSettings(false));
    this.btnRefreshBrowsers.addEventListener("click", () => this.refreshDiscoveredBrowsers());
    this.btnSettingsRegisterHandler.addEventListener("click", () => this.registerDefaultProtocolHandler());
    this.btnSettingsOpenOs.addEventListener("click", () => this.openOsSettings());
    this.btnSettingsRefreshHandler.addEventListener("click", () => this.reverifyProtocolHandler());
    this.btnRerunOnboarding.addEventListener("click", () => this.rerunOnboardingFlow());

    document.querySelectorAll(".tab-btn").forEach((btn) => {
      btn.addEventListener("click", (e) => {
        const tab = (e.currentTarget as HTMLElement).dataset.tab;
        if (tab) this.selectTab(tab);
      });
    });

    this.prefRuleMatching.addEventListener("change", () => {
      this.config.preferences.rule_routing_enabled = this.prefRuleMatching.checked;
      this.persistSettings();
    });

    this.prefCloseAfterLaunch.addEventListener("change", () => {
      this.config.preferences.dismiss_after_launch = this.prefCloseAfterLaunch.checked;
      this.persistSettings();
    });

    this.prefMinimizeOnBlur.addEventListener("change", () => {
      this.config.preferences.minimize_on_focus_loss = this.prefMinimizeOnBlur.checked;
      this.persistSettings();
    });

    this.prefTheme.addEventListener("change", () => {
      this.config.preferences.theme = this.prefTheme.value;
      this.applyTheme(this.config.preferences.theme);
      this.persistSettings();
    });

    // Rule modal
    this.btnOpenAddRuleModal.addEventListener("click", () => this.toggleRuleModal(true));
    this.btnCloseRuleModal.addEventListener("click", () => this.toggleRuleModal(false));
    this.btnCancelRule.addEventListener("click", () => this.toggleRuleModal(false));
    this.btnSaveRule.addEventListener("click", () => this.saveNewRule());
  }

  private showView(view: "onboarding" | "chooser" | "settings") {
    this.onboardingView.hidden = view !== "onboarding";
    this.chooserView.hidden = view !== "chooser";
    this.settingsView.hidden = view !== "settings";

    if (view === "chooser") {
      this.renderBrowserGrid();
    } else if (view === "settings") {
      this.renderSettings();
    }
  }

  // --- Onboarding Wizard Logic ---

  private async navigateWizard(delta: number) {
    const nextStep = this.currentOnboardingStep + delta;
    if (nextStep >= 1 && nextStep <= 5) {
      this.currentOnboardingStep = nextStep;
      this.renderOnboardingStep(this.currentOnboardingStep);
      try {
        await invokeTauri("save_onboarding_step", { step: this.currentOnboardingStep });
      } catch (e) {
        console.warn("Failed to persist onboarding step:", e);
      }
    }
  }

  private renderOnboardingStep(step: number) {
    // Update step dots
    document.querySelectorAll(".step-dot").forEach((dot) => {
      const dotStep = parseInt((dot as HTMLElement).dataset.step || "1", 10);
      dot.classList.remove("active", "completed");
      if (dotStep === step) {
        dot.classList.add("active");
      } else if (dotStep < step) {
        dot.classList.add("completed");
      }
    });

    // Toggle step cards
    document.querySelectorAll(".wizard-step").forEach((el) => el.classList.remove("active"));
    const activeCard = document.getElementById(`wizard-step-${step}`);
    if (activeCard) activeCard.classList.add("active");

    // Nav buttons
    this.btnWizardBack.disabled = step === 1;
    if (step === 5) {
      this.btnWizardNext.hidden = true;
      this.btnWizardFinish.hidden = false;
      this.renderStep5Summary();
    } else {
      this.btnWizardNext.hidden = false;
      this.btnWizardFinish.hidden = true;
    }

    if (step === 2) {
      this.renderOnboardingBrowserList();
    } else if (step === 3) {
      this.reverifyProtocolHandler();
    } else if (step === 4) {
      this.renderOnboardingPreferences();
    }
  }

  private renderOnboardingBrowserList() {
    this.onboardingBrowserList.innerHTML = "";
    this.onboardingBrowserCount.textContent = `${this.config.browsers.length} Found`;

    if (this.config.browsers.length === 0) {
      this.onboardingBrowserList.innerHTML = `
        <div class="empty-state">
          <span>No desktop browsers detected. Click "Refresh Scan" once an application is installed.</span>
        </div>
      `;
      return;
    }

    this.config.browsers.forEach((browser) => {
      const item = document.createElement("div");
      item.className = "browser-checklist-item";

      item.innerHTML = `
        <div class="browser-check-left">
          ${getBrowserIconSvg(browser.id, browser.name)}
          <div>
            <div class="browser-check-name">${browser.name}</div>
            <div class="browser-check-path">${browser.executable_path}</div>
          </div>
        </div>
        <div class="switch">
          <input type="checkbox" ${browser.enabled ? "checked" : ""} class="browser-toggle" data-id="${browser.id}" />
          <span class="switch-slider"></span>
        </div>
      `;

      const check = item.querySelector(".browser-toggle") as HTMLInputElement;
      check.addEventListener("change", () => {
        browser.enabled = check.checked;
        this.persistSettings();
      });

      this.onboardingBrowserList.appendChild(item);
    });
  }

  private renderOnboardingPreferences() {
    this.onboardingPreferredSelect.innerHTML = "";
    this.config.browsers.forEach((b) => {
      const opt = document.createElement("option");
      opt.value = b.id;
      opt.textContent = b.name;
      if (this.config.preferences.preferred_browser_id === b.id) {
        opt.selected = true;
      }
      this.onboardingPreferredSelect.appendChild(opt);
    });
  }

  private renderStep5Summary() {
    this.summaryHandlerStatus.textContent = this.handlerStatus.is_default
      ? "Verified (Default)"
      : `Not Default (${this.handlerStatus.current_handler || "None"})`;
    this.summaryHandlerStatus.style.color = this.handlerStatus.is_default
      ? "var(--success)"
      : "var(--warning)";

    this.summaryBrowserCount.textContent = `${this.getEnabledTargets().length} Active`;

    const radioAlways = document.getElementById("opt-always-ask") as HTMLInputElement;
    this.summaryFallbackMode.textContent = radioAlways?.checked ? "Always Prompt (HUD)" : "Direct to Preferred";

    this.summaryRulesCount.textContent = `${this.config.rulesets.length} Rules`;
  }

  private async finishOnboarding() {
    const radioAlways = document.getElementById("opt-always-ask") as HTMLInputElement;
    const fallbackBehavior = radioAlways?.checked ? "always_ask" : "open_preferred";
    const preferredId = this.onboardingPreferredSelect.value || null;

    try {
      await invokeTauri("complete_onboarding", {
        preferredBrowserId: preferredId,
        fallbackBehavior,
      });
      this.config.preferences.onboarding_completed = true;
      this.config.preferences.fallback_behavior = fallbackBehavior;
      this.config.preferences.preferred_browser_id = preferredId;
      this.showToast("Setup completed successfully!");
      this.showView("chooser");
      this.checkHandlerAlert();
    } catch (e) {
      this.showToast(`Error saving configuration: ${e}`);
    }
  }

  private async rerunOnboardingFlow() {
    try {
      await invokeTauri("reset_onboarding");
      this.config.preferences.onboarding_completed = false;
      this.config.preferences.onboarding_step = 1;
      this.currentOnboardingStep = 1;
      this.showView("onboarding");
      this.renderOnboardingStep(1);
    } catch (e) {
      this.showToast(`Could not reset onboarding: ${e}`);
    }
  }

  // --- Protocol Status & Registration Actions ---

  private async reverifyProtocolHandler() {
    try {
      this.handlerStatus = await invokeTauri<HandlerStatus>("get_default_handler_status");
      this.updateHandlerStatusUi();
    } catch (e) {
      console.warn("Handler status query failed:", e);
    }
  }

  private updateHandlerStatusUi() {
    const isDefault = this.handlerStatus.is_default;

    // Wizard Step 3 Card
    if (this.handlerStatusBullet) {
      this.handlerStatusBullet.className = "status-bullet " + (isDefault ? "verified" : "unverified");
      this.handlerStatusBadge.className = "badge " + (isDefault ? "verified" : "unverified");
      this.handlerStatusBadge.textContent = isDefault ? "Verified" : "Not Default";
      this.handlerStatusTitle.textContent = isDefault
        ? "OpenIn is the Default Web Browser"
        : "OpenIn is Not Currently Default";
      this.handlerStatusDetail.textContent = this.handlerStatus.details;
    }

    // Settings General Card
    if (this.settingsHandlerBadge) {
      this.settingsHandlerBadge.className = "badge " + (isDefault ? "verified" : "unverified");
      this.settingsHandlerBadge.textContent = isDefault ? "Active Default" : "Not Registered";
      this.settingsHandlerDesc.textContent = this.handlerStatus.details;
    }

    this.checkHandlerAlert();
  }

  private checkHandlerAlert() {
    if (!this.handlerStatus.is_default && this.config.preferences.onboarding_completed) {
      this.handlerAlertBanner.hidden = false;
    } else {
      this.handlerAlertBanner.hidden = true;
    }
  }

  private async registerDefaultProtocolHandler() {
    this.showToast("Registering with operating system...");
    try {
      const res = await invokeTauri<RegistrationResult>("register_default_handler");
      await this.reverifyProtocolHandler();
      if (res.verified) {
        this.showToast("Verified: OpenIn is now your default browser!");
      } else {
        this.showToast(res.message);
      }
    } catch (e) {
      this.showToast(`Registration failed: ${e}`);
    }
  }

  private async openOsSettings() {
    try {
      await invokeTauri("open_os_protocol_settings");
      this.showToast("Opening operating system settings...");
    } catch (e) {
      this.showToast(`Cannot open settings: ${e}`);
    }
  }

  private async runSimulationTest() {
    const url = this.simulationTestUrl.value.trim() || "https://github.com/MossRune/OpenIn";
    this.simulationResult.textContent = "Simulating routing decision...";

    try {
      const res = await invokeTauri<RouteSimulationResult>("test_route_simulation", { url });
      if (res.matched_rule) {
        this.simulationResult.innerHTML = `<strong>Rule Matched:</strong> "${res.matched_rule}" ➔ <strong>${res.target_browser_name || res.target_browser_id}</strong>`;
      } else {
        this.simulationResult.innerHTML = `<strong>Standard Route:</strong> Opens in <strong>${res.target_browser_name || res.target_browser_id || "Chooser Window"}</strong> (Sanitized: ${res.sanitized_url})`;
      }
    } catch (e) {
      this.simulationResult.textContent = `Simulation error: ${e}`;
    }
  }

  // --- Browser Chooser HUD Logic ---

  private getEnabledTargets(): BrowserTarget[] {
    return this.config.browsers.filter((b) => b.enabled);
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
        ? `<img class="browser-icon-img" src="${target.icon_path}" alt="${target.name}" onerror="this.outerHTML='${getBrowserIconSvg(target.id, target.name)}';" />`
        : getBrowserIconSvg(target.id, target.name);

      card.innerHTML = `
        ${keyBadge}
        <div class="browser-icon-wrapper">
          ${iconElement}
        </div>
        <span class="browser-name" title="${target.name}">${target.name}</span>
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

  private handleKeyboardShortcut(e: KeyboardEvent) {
    const isInputActive = document.activeElement === this.urlInput;
    const isModalOpen = !this.ruleModal.hidden;

    if (isModalOpen) {
      if (e.key === "Escape") {
        e.preventDefault();
        this.toggleRuleModal(false);
      }
      return;
    }

    if (e.key === "Escape") {
      e.preventDefault();
      if (!this.settingsView.hidden) {
        this.toggleSettings(false);
      } else {
        invokeTauri("dismiss_window").catch(() => {});
      }
      return;
    }

    // In Onboarding Wizard
    if (!this.onboardingView.hidden) {
      if (e.key === "Enter") {
        e.preventDefault();
        if (this.currentOnboardingStep === 5) {
          this.finishOnboarding();
        } else {
          this.navigateWizard(1);
        }
      }
      return;
    }

    // In Main Chooser View
    if (!isInputActive && this.settingsView.hidden) {
      if (e.key >= "1" && e.key <= "9") {
        const index = parseInt(e.key, 10) - 1;
        const targets = this.getEnabledTargets();
        if (index < targets.length) {
          e.preventDefault();
          this.launchTarget(targets[index]);
          return;
        }
      }

      if ((e.key === "c" || e.key === "C") && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        this.copyUrlToClipboard();
        return;
      }

      if ((e.key === "e" || e.key === "E") && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        this.urlInput.focus();
        this.urlInput.select();
        return;
      }

      if (e.key === ",") {
        e.preventDefault();
        this.toggleSettings(true);
        return;
      }

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
  }

  private async launchTarget(target: BrowserTarget) {
    const url = this.urlInput.value.trim() || this.currentUrl;
    if (!url) {
      this.showToast("Please enter or paste a web address");
      return;
    }

    try {
      await invokeTauri("launch_browser", {
        browserId: target.id,
        url,
        alternateId: null,
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

  // --- Settings View Logic ---

  private toggleSettings(open: boolean) {
    if (open) {
      this.showView("settings");
      this.renderSettings();
      this.reverifyProtocolHandler();
    } else {
      this.showView("chooser");
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
    this.prefRuleMatching.checked = this.config.preferences.rule_routing_enabled;
    this.prefCloseAfterLaunch.checked = this.config.preferences.dismiss_after_launch;
    this.prefMinimizeOnBlur.checked = this.config.preferences.minimize_on_focus_loss;
    this.prefTheme.value = this.config.preferences.theme;

    // Render registered browsers
    this.settingsBrowserList.innerHTML = "";
    this.config.browsers.forEach((target) => {
      const row = document.createElement("div");
      row.className = "browser-config-item";

      row.innerHTML = `
        <div class="config-item-left">
          ${getBrowserIconSvg(target.id, target.name)}
          <div>
            <div class="config-item-name">${target.name}</div>
            <div class="config-item-path">${target.executable_path}</div>
          </div>
        </div>
        <div class="switch">
          <input type="checkbox" ${target.enabled ? "checked" : ""} class="browser-settings-check" />
          <span class="switch-slider"></span>
        </div>
      `;

      const check = row.querySelector(".browser-settings-check") as HTMLInputElement;
      check.addEventListener("change", async () => {
        target.enabled = check.checked;
        await this.persistSettings();
      });

      this.settingsBrowserList.appendChild(row);
    });

    // Render rulesets
    this.renderRulesList();
  }

  private renderRulesList() {
    this.settingsRulesList.innerHTML = "";
    if (this.config.rulesets.length === 0) {
      this.settingsRulesList.innerHTML = `<div class="empty-state">No routing rules configured. Create a rule to automatically direct URLs to specific browsers.</div>`;
      return;
    }

    this.config.rulesets.forEach((r, idx) => {
      const item = document.createElement("div");
      item.className = "rule-config-item";
      item.innerHTML = `
        <div>
          <div class="config-item-name">${r.name} ➔ ${r.target_browser_id}</div>
          <div class="config-item-path">${r.rules.join(", ")}</div>
        </div>
        <button class="btn-delete-rule" title="Delete rule" data-idx="${idx}">
          <svg class="icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
          </svg>
        </button>
      `;

      const btnDelete = item.querySelector(".btn-delete-rule") as HTMLButtonElement;
      btnDelete.addEventListener("click", () => {
        this.config.rulesets.splice(idx, 1);
        this.persistSettings();
        this.renderRulesList();
        this.showToast("Rule removed");
      });

      this.settingsRulesList.appendChild(item);
    });
  }

  private toggleRuleModal(open: boolean) {
    this.ruleModal.hidden = !open;
    if (open) {
      this.ruleNameInput.value = "";
      this.rulePatternInput.value = "";
      this.ruleTargetSelect.innerHTML = "";
      this.config.browsers.forEach((b) => {
        const opt = document.createElement("option");
        opt.value = b.id;
        opt.textContent = b.name;
        this.ruleTargetSelect.appendChild(opt);
      });
      this.ruleNameInput.focus();
    }
  }

  private async saveNewRule() {
    const name = this.ruleNameInput.value.trim();
    const pattern = this.rulePatternInput.value.trim();
    const target = this.ruleTargetSelect.value;

    if (!name || !pattern) {
      this.showToast("Please provide both rule name and domain pattern");
      return;
    }

    this.config.rulesets.push({
      id: "rule-" + Date.now(),
      name,
      rules: [pattern],
      target_browser_id: target,
      launch_arguments: null,
      enabled: true,
      priority: this.config.rulesets.length + 1,
    });

    await this.persistSettings();
    this.toggleRuleModal(false);
    this.renderRulesList();
    this.showToast("Rule added successfully");
  }

  private async refreshDiscoveredBrowsers() {
    this.showToast("Scanning system for browsers...");
    try {
      const discovered = await invokeTauri<BrowserTarget[]>("refresh_browsers");
      this.config.browsers = discovered;
      await this.persistSettings();
      this.renderBrowserGrid();
      this.renderSettings();
      if (!this.onboardingView.hidden) {
        this.renderOnboardingBrowserList();
      }
      this.showToast(`Found ${discovered.length} browsers`);
    } catch (err) {
      this.showToast("Discovery refresh failed");
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
    }, 2800);
  }
}

document.addEventListener("DOMContentLoaded", () => {
  const app = new OpenInApplication();
  app.init();
});
