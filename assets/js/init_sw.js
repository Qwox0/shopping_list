if ("serviceWorker" in navigator) {
    window.addEventListener("load", function() {
        navigator.serviceWorker.register("/js/sw.js").then(registration => {
            console.log("[Service Worker] ", "Registration successful! Scope:", registration.scope);
        }, err => {
            console.log("[Service Worker] ", "Registration failed:", err);
        }
        );
    });
}
