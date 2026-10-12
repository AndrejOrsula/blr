/*
 * Install as a classic deferred script from /blr/isolation.js on every page
 * that creates a pthread runtime, including its runtime iframe.
 */
(() => {
  "use strict";

  const setupMarker = "__blr_coi_setup=1";
  const removeSetupMarker = (href) => {
    const hashIndex = href.indexOf("#");
    const base = hashIndex < 0 ? href : href.slice(0, hashIndex);
    const hash = hashIndex < 0 ? "" : href.slice(hashIndex);
    const queryIndex = base.indexOf("?");
    if (queryIndex < 0) return { attempted: false, href };
    const lastAmp = base.lastIndexOf("&");
    const componentStart = lastAmp > queryIndex ? lastAmp + 1 : queryIndex + 1;
    if (base.slice(componentStart) !== setupMarker) return { attempted: false, href };
    const cleanBase = lastAmp > queryIndex ? base.slice(0, lastAmp) : base.slice(0, queryIndex);
    return { attempted: true, href: `${cleanBase}${hash}` };
  };

  const addSetupMarker = (href) => {
    const hashIndex = href.indexOf("#");
    const base = hashIndex < 0 ? href : href.slice(0, hashIndex);
    const hash = hashIndex < 0 ? "" : href.slice(hashIndex);
    const separator = base.includes("?") ? "&" : "?";
    return `${base}${separator}${setupMarker}${hash}`;
  };

  const ready = (async () => {
    const fail = (message) => {
      throw new Error(`BLR cross-origin isolation: ${message}`);
    };

    if (!window.isSecureContext) {
      fail("use HTTPS or localhost, then reload this page.");
    }
    const scriptUrl = document.currentScript?.src || document.querySelector('script[src$="/isolation.js"]')?.src;
    if (!scriptUrl) {
      fail("load /blr/isolation.js as a classic script before starting the runtime.");
    }
    const script = new URL(scriptUrl, location.href);
    const scopeUrl = new URL("./", script);
    const scopePath = scopeUrl.pathname;
    const setup = removeSetupMarker(location.href);
    const originalUrl = new URL(setup.href);
    const legacyReloadKey = `blr-coi-reload:${scopePath}:${originalUrl.pathname}${originalUrl.search}`;
    if (setup.attempted) {
      try {
        window.history.replaceState(window.history.state, "", setup.href);
      } catch (error) {
        fail(`the one-time setup marker could not be removed from this URL (${error instanceof Error ? error.message : String(error)}). Open the demo directly in a new tab and reload normally.`);
      }
    }
    // Clear the exact legacy key left by an older bootstrap. The URL marker
    // now bounds the automatic setup attempt.
    try {
      sessionStorage.removeItem(legacyReloadKey);
    } catch {
      // URL state bounds the setup attempt without depending on session storage.
    }

    if (window.crossOriginIsolated === true) {
      if (typeof SharedArrayBuffer === "undefined") {
        fail("this browser reports isolation but does not expose SharedArrayBuffer; use a current browser with cross-origin isolation support.");
      }
      return Object.freeze({ crossOriginIsolated: true, sharedArrayBuffer: true });
    }
    if (window.crossOriginIsolated === undefined) {
      fail("this browser does not support cross-origin isolation.");
    }
    if (setup.attempted) {
      if (!navigator.serviceWorker?.controller) {
        fail(`the browser did not route the setup navigation through the BLR service worker. Use a normal reload, check that service workers are allowed for this site, then open ${scopeUrl.href} directly in a new tab if isolation remains unavailable.`);
      }
      fail(`the setup navigation was controlled, but browser isolation remains disabled. Open ${scopeUrl.href} directly in a new tab, or check whether the embedding page or browser policy allows cross-origin isolation.`);
    }
    if (window.top !== window) {
      let topLevelIsolated;
      try {
        topLevelIsolated = window.top.crossOriginIsolated;
      } catch {
        // A cross-origin ancestor may hide its isolation state; try one setup.
      }
      if (topLevelIsolated === false) {
        fail(`the top-level page is not cross-origin isolated. Open ${scopeUrl.href} directly in a new tab, or allow cross-origin isolation in the embedding page.`);
      }
    }
    if (!navigator.serviceWorker) {
      fail("service workers are unavailable; use a current browser in a normal (non-private) window.");
    }

    let registration;
    try {
      const workerUrl = new URL("./coi-serviceworker.js", script);
      const existing = await navigator.serviceWorker.getRegistration(scopePath);
      if (existing) {
        const workers = [existing.active, existing.waiting, existing.installing].filter(Boolean);
        const ours = workerUrl.href;
        if (workers.some((worker) => new URL(worker.scriptURL).href !== ours)) {
          fail(`another service worker already owns ${scopePath}; remove that registration through its own application before using this demo.`);
        }
      }
      registration = await navigator.serviceWorker.register(workerUrl.href, {
        scope: scopePath,
        updateViaCache: "none",
      });
      await new Promise((resolve, reject) => {
          const timer = setTimeout(() => reject(new Error("service worker activation timed out")), 10000);
          const check = () => {
            if (registration.active?.state === "activated") {
              clearTimeout(timer);
              resolve();
              return true;
            }
            return false;
          };
          if (check()) return;
          for (const worker of [registration.installing, registration.waiting, registration.active]) {
            worker?.addEventListener("statechange", check);
          }
        });
      if (registration.active?.state !== "activated") {
        fail("the /blr/ service worker did not activate. Check browser service-worker settings and retry.");
      }
    } catch (error) {
      if (error instanceof Error && error.message.startsWith("BLR cross-origin isolation:")) throw error;
      fail(`could not install the /blr/ service worker (${error instanceof Error ? error.message : String(error)}). Check HTTPS, browser service-worker settings, and the /blr/ scope.`);
    }

    // A distinct URL forces a service-worker navigation. Its marker survives a
    // COOP browsing-group switch and is removed by the next document.
    if (!registration) fail("the /blr/ service worker did not register.");
    location.replace(addSetupMarker(location.href));
    return new Promise((_, reject) => {
      setTimeout(() => reject(new Error("BLR cross-origin isolation: the one-time setup navigation did not complete. Use a normal reload, then open the demo directly in a new tab if isolation remains unavailable.")), 10000);
    });
  })();

  window.blrIsolationReady = ready;
  // Mark the public promise handled immediately; consumers still receive the
  // original rejection when they await or catch this promise later.
  window.blrIsolationReady.catch(() => {});
})();
