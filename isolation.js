/*
 * Install as a classic deferred script from /blr/isolation.js on every page
 * that creates a pthread runtime, including its runtime iframe.
 */
(() => {
  "use strict";

  let reloadKey;
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
    reloadKey = `blr-coi-reload:${scopePath}:${location.pathname}${location.search}`;

    if (window.crossOriginIsolated === true) {
      if (typeof SharedArrayBuffer === "undefined") {
        fail("this browser reports isolation but does not expose SharedArrayBuffer; use a current browser with cross-origin isolation support.");
      }
      return Object.freeze({ crossOriginIsolated: true, sharedArrayBuffer: true });
    }
    if (window.crossOriginIsolated === undefined) {
      fail("this browser does not support cross-origin isolation.");
    }
    if (!navigator.serviceWorker) {
      fail("service workers are unavailable; use a current browser in a normal (non-private) window.");
    }

    let reloadAttempted;
    try {
      reloadAttempted = sessionStorage.getItem(reloadKey) === "1";
    } catch {
      fail("session storage is unavailable, so a safe one-time setup reload cannot be tracked.");
    }
    if (reloadAttempted) {
      try {
        sessionStorage.removeItem(reloadKey);
      } catch {
        // The failure is still reported; a fresh tab gives the user a new session.
      }
      fail("isolation is still unavailable after the one setup reload. Check the /blr/ service-worker response and CSP, then reload manually to retry.");
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

    // On a first visit the registration controls future navigations. Reload the
    // current document exactly once so its response receives COOP and COEP.
    try {
      sessionStorage.setItem(reloadKey, "1");
    } catch {
      fail("session storage became unavailable before the setup reload.");
    }
    if (!registration) fail("the /blr/ service worker did not register.");
    location.reload();
    return new Promise((_, reject) => {
      setTimeout(() => reject(new Error("BLR cross-origin isolation: the setup reload did not navigate. Reload this page manually once.")), 10000);
    });
  })();

  window.blrIsolationReady = ready.then((result) => {
    try {
      if (reloadKey) sessionStorage.removeItem(reloadKey);
    } catch {
      // Isolation is already proven; inability to clear the retry marker is safe.
    }
    return result;
  });
  // Mark the public promise handled immediately; consumers still receive the
  // original rejection when they await or catch this promise later.
  window.blrIsolationReady.catch(() => {});
})();
