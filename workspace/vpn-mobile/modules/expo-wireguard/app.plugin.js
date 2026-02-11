const { withAndroidManifest } = require("@expo/config-plugins");

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
                    "android:permission":
                        "android.permission.BIND_VPN_SERVICE",
                    "android:exported": "false",
                },
                "intent-filter": [
                    {
                        action: [
                            {
                                $: {
                                    "android:name":
                                        "android.net.VpnService",
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

module.exports = addVpnService;
