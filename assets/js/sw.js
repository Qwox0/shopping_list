const cacheName = "shoppingListPWA-v1";
const cacheFirst = [
    "/",
    "/en",
    "/de",
    "/favicon.ico",
    "/img/bin.webp",
    "/img/pen.webp",
    "/js/init_sw.js",
    "/js/sw.js",
    "/language/en.toml",
    "/language/de.toml",
    "/pkg/shopping_list.css",
    "/pkg/shopping_list.js",
    "/pkg/shopping_list.wasm",
    "/pwa.webmanifest",
];
const cacheAfter = [
    //"/icons/icon-1024.png",
    "/img/icon-120.png",
    "/img/icon-128.png",
    "/img/icon-144.png",
    "/img/icon-152.png",
    "/img/icon-180.png",
    "/img/icon-192.png",
    "/img/icon-256.png",
    "/img/icon-384.png",
    "/img/icon-512.png",
    "/img/icon-72.png",
    "/img/icon-96.png",
];

// // // Utils
const log = (...args) => {
    console.log("[Service Worker] ", ...args)
};
const err = (...args) => {
    console.error("[Service Worker] ", ...args)
};

const addAll = async (cache, requests) => {
    for (req of requests) {
        try { await cache.add(req); }
        catch (e) {
            err(`Failed to fetch "${req}"!`);
        }
    }
};

const cache_fetch = async (request) => {
    const response = await caches.match(request);
    if (!response) return undefined;
    log(`Fetched from Cache: ${request.url}`);
    return { response, source: "cache" }
};

/** must clone response! */
const cache_add = async (request, response) => {
    const cache = await caches.open(cacheName);
    log(`Caching new resource: ${request.url}`);
    cache.put(request, response);
};

const network_fetch = async (request) => {
    try {
        const response = await fetch(request);
        log(`Fetched from Server: ${request.url}`);
        return { response, source: "network" }
    } catch (err) {
        err(`Failed to fetch from Server: ${err}`);
        return undefined;
    }
};

// // // Main
// Install
self.addEventListener("install", event => {
    log("Install");
    event.waitUntil((async () => {
        const cache = await caches.open(cacheName);
        log("slowly Caching: ", cacheAfter);
        addAll(cache, cacheAfter);
        log("Caching immediately: ", cacheFirst);
        await addAll(cache, cacheFirst);
    })());
});

// Activate
self.addEventListener("activate", event => {
    log("Activate and remove old files");
    event.waitUntil(
        caches.keys().then((keys) => {
            return Promise.all(keys
                .filter((key) => key !== cacheName)
                .map(caches.delete)
                //.map((key) => caches.delete(key)),
            );
        })
    );
});

// fetch data
const cache_over_network = async (event) => {
    const response = await cache_fetch(event.request) || await network_fetch(event.request);
    if (!response) return; // todo: case: no cache + no connection
    if (response.source !== "cache") cache_add(event.request, response.response.clone());
    return response.response;
};
const network_only = async (event) => (await network_fetch(event.request))?.response;
const network_over_cache = async (event) => {
    const response = await network_fetch(event.request) || await cache_fetch(event.request);
    log("response:", response)
    if (!response) return; // todo: case: no cache + no connection
    if (response.source !== "cache") cache_add(event.request, response.response.clone());
    return response.response;
};
self.addEventListener("fetch", event => {
    //event.respondWith(cache_over_network(event)); // for release
    event.respondWith(network_over_cache(event)); // for dev
});
/*
self.addEventListener("fetch", event => {
    event.respondWith((async () => {
        const r = await caches.match(event.request);
        if (r) {
            log(`Fetched resource ${event.request.url}`);
            return r;
        }
        const response = await fetch(event.request);
        const cache = await caches.open(cacheName);
        log(`Caching new resource: ${event.request.url}`);
        cache.put(event.request, response.clone());
        return response;
    })());
});
*/

// Notification
/*
self.addEventListener("push", (event) => {
    if (event.data.text() != "new-email") return;
    event.waitUntil((async () => {
        const cache = await caches.open("mysite-dynamic");
        const response = await fetch("/inbox.json");
        await cache.put("/inbox.json", response.clone());
        const emails = await response.json();
        registration.showNotification("New email", {
            body: "From " + emails[0].from.name,
            tag: "new-email"
        });
    })());
});

self.addEventListener("notificationclick", function(event) {
    if (event.notification.tag == "new-email") {
        // Assume that all of the resources needed to render
        // /inbox/ have previously been cached, e.g. as part
        // of the install handler.
        new WindowClient("/inbox/");
    }
});
*/
