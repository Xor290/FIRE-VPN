const {
    withAndroidManifest,
    withAppBuildGradle,
} = require("@expo/config-plugins");

function enableCoreLibraryDesugaring(config) {
    return withAppBuildGradle(config, (config) => {
        let buildGradle = config.modResults.contents;

        // Add coreLibraryDesugaring dependency if not present
        if (!buildGradle.includes("coreLibraryDesugaring")) {
            buildGradle = buildGradle.replace(
                /dependencies\s*\{/,
                `dependencies {\n    coreLibraryDesugaring 'com.android.tools:desugar_jdk_libs:2.0.3'`,
            );
        }

        // Enable coreLibraryDesugaringEnabled in compileOptions
        if (!buildGradle.includes("coreLibraryDesugaringEnabled")) {
            if (buildGradle.match(/compileOptions\s*\{/)) {
                // compileOptions block exists, inject into it
                buildGradle = buildGradle.replace(
                    /compileOptions\s*\{/,
                    `compileOptions {\n        coreLibraryDesugaringEnabled true`,
                );
            } else {
                // No compileOptions block, add one inside the android block
                buildGradle = buildGradle.replace(
                    /android\s*\{/,
                    `android {\n    compileOptions {\n        coreLibraryDesugaringEnabled true\n    }`,
                );
            }
        }

        config.modResults.contents = buildGradle;
        return config;
    });
}

function enableCleartextTraffic(config) {
    return withAndroidManifest(config, async (config) => {
        const manifest = config.modResults;
        const app = manifest.manifest.application?.[0];
        if (!app) return config;

        app.$["android:usesCleartextTraffic"] = "true";

        return config;
    });
}

function addVpnService(config) {
    return withAndroidManifest(config, async (config) => {
        const manifest = config.modResults;
        const app = manifest.manifest.application?.[0];
        if (!app) return config;

        // Add VpnService declaration if not already present
        if (!app.service) {
            app.service = [];
        }

        const vpnServiceExists = app.service.some(
            (s) =>
                s.$?.["android:name"] ===
                "com.wireguard.android.backend.GoBackend$VpnService",
        );

        if (!vpnServiceExists) {
            app.service.push({
                $: {
                    "android:name":
                        "com.wireguard.android.backend.GoBackend$VpnService",
                    "android:permission": "android.permission.BIND_VPN_SERVICE",
                    "android:exported": "false",
                },
                "intent-filter": [
                    {
                        action: [
                            {
                                $: {
                                    "android:name": "android.net.VpnService",
                                },
                            },
                        ],
                    },
                ],
            });
        }

        return config;
    });
}

module.exports = (config) => {
    config = enableCoreLibraryDesugaring(config);
    config = enableCleartextTraffic(config);
    config = addVpnService(config);
    return config;
};
