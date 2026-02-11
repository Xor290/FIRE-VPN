import React, { useEffect, useRef } from "react";
import {
    View,
    Text,
    ScrollView,
    TouchableOpacity,
    StyleSheet,
    Animated,
    ActivityIndicator,
    Platform,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { Colors, Spacing, BorderRadius, getCountryFlag } from "../theme";
import { useAuth } from "../contexts/AuthContext";
import { ServerCard } from "../components/ServerCard";
import { StatusPill } from "../components/StatusPill";

export function ConnectedScreen() {
    const {
        connectedServer,
        connectionInfo,
        servers,
        isLoading,
        error,
        disconnectFromServer,
        switchServer,
        logout,
        clearError,
        user,
        tunnelStatus,
    } = useAuth();

    const isTunnelUp = tunnelStatus === "UP";

    // Pulsing animation
    const pulseAnim = useRef(new Animated.Value(1)).current;

    useEffect(() => {
        const animation = Animated.loop(
            Animated.sequence([
                Animated.timing(pulseAnim, {
                    toValue: 1.4,
                    duration: 1200,
                    useNativeDriver: true,
                }),
                Animated.timing(pulseAnim, {
                    toValue: 1,
                    duration: 1200,
                    useNativeDriver: true,
                }),
            ]),
        );
        animation.start();
        return () => animation.stop();
    }, [pulseAnim]);

    const pulseOpacity = pulseAnim.interpolate({
        inputRange: [1, 1.4],
        outputRange: [0.6, 0],
    });

    // Parse WireGuard config for details
    const configLines = connectionInfo?.config?.split("\n") ?? [];
    const getConfigValue = (key: string): string => {
        const line = configLines.find((l) => l.startsWith(key));
        return line?.split("=").slice(1).join("=").trim() ?? "-";
    };

    const peerIp = connectionInfo?.peer_ip ?? "-";
    const dns = getConfigValue("DNS");
    const endpoint = getConfigValue("Endpoint");

    return (
        <SafeAreaView style={styles.container}>
            {/* Accent bar */}
            <View style={styles.accentBar} />

            {/* Header */}
            <View style={styles.header}>
                <View>
                    <Text style={styles.title}>FIRE VPN</Text>
                    {user && <Text style={styles.userEmail}>{user.email}</Text>}
                </View>
                <TouchableOpacity onPress={logout}>
                    <Text style={styles.logoutText}>Deconnexion</Text>
                </TouchableOpacity>
            </View>

            {/* Error */}
            {error && (
                <View style={styles.errorBox}>
                    <Text style={styles.errorText}>{error}</Text>
                    <TouchableOpacity onPress={clearError}>
                        <Text style={styles.errorClose}>X</Text>
                    </TouchableOpacity>
                </View>
            )}

            <ScrollView contentContainerStyle={styles.scrollContent}>
                {/* Connection status hero */}
                <View style={styles.heroSection}>
                    <View style={styles.pulseContainer}>
                        {isTunnelUp && (
                            <Animated.View
                                style={[
                                    styles.pulseRing,
                                    {
                                        transform: [{ scale: pulseAnim }],
                                        opacity: pulseOpacity,
                                    },
                                ]}
                            />
                        )}
                        <View
                            style={[
                                styles.pulseDot,
                                !isTunnelUp && {
                                    backgroundColor: Colors.textMuted,
                                },
                            ]}
                        />
                    </View>
                    <Text
                        style={[
                            styles.connectedText,
                            {
                                color: isTunnelUp
                                    ? Colors.success
                                    : Colors.textMuted,
                            },
                        ]}
                    >
                        {isTunnelUp ? "CONNECTE" : "TUNNEL INACTIF"}
                    </Text>
                    {connectedServer && (
                        <Text style={styles.serverInfo}>
                            {getCountryFlag(connectedServer.country)}{" "}
                            {connectedServer.name}
                        </Text>
                    )}
                </View>

                {/* Connection details card */}
                <View style={styles.detailsCard}>
                    <Text style={styles.detailsTitle}>
                        Details de connexion
                    </Text>
                    <View style={styles.detailRow}>
                        <Text style={styles.detailLabel}>IP locale</Text>
                        <Text style={styles.detailValue}>{peerIp}</Text>
                    </View>
                    <View style={styles.detailRow}>
                        <Text style={styles.detailLabel}>DNS</Text>
                        <Text style={styles.detailValue}>{dns}</Text>
                    </View>
                    <View style={styles.detailRow}>
                        <Text style={styles.detailLabel}>Endpoint</Text>
                        <Text style={styles.detailValue}>{endpoint}</Text>
                    </View>
                </View>

                {/* Server switch section */}
                {servers.length > 0 && (
                    <>
                        <Text style={styles.sectionHeading}>SERVEURS</Text>
                        {servers.map((server) => {
                            const isCurrent = server.id === connectedServer?.id;
                            return (
                                <ServerCard
                                    key={server.id}
                                    server={server}
                                    selected={isCurrent}
                                    onPress={() => {
                                        if (!isCurrent && !isLoading) {
                                            switchServer(server);
                                        }
                                    }}
                                    rightElement={
                                        isCurrent ? (
                                            <StatusPill
                                                label="Actif"
                                                color={Colors.success}
                                            />
                                        ) : (
                                            <Text style={styles.switchText}>
                                                Changer
                                            </Text>
                                        )
                                    }
                                />
                            );
                        })}
                    </>
                )}
            </ScrollView>

            {/* Disconnect button */}
            <View style={styles.footer}>
                <TouchableOpacity
                    style={styles.disconnectButton}
                    onPress={disconnectFromServer}
                    disabled={isLoading}
                    activeOpacity={0.8}
                >
                    {isLoading ? (
                        <ActivityIndicator color={Colors.textPrimary} />
                    ) : (
                        <Text style={styles.disconnectButtonText}>
                            Se deconnecter
                        </Text>
                    )}
                </TouchableOpacity>
            </View>
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: Colors.background,
    },
    accentBar: {
        height: 3,
        backgroundColor: Colors.accent,
    },
    header: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        paddingHorizontal: Spacing.xl,
        paddingTop: Spacing.xl,
        paddingBottom: Spacing.md,
    },
    title: {
        fontSize: 20,
        fontWeight: "700",
        color: Colors.accent,
        letterSpacing: 2,
    },
    userEmail: {
        color: Colors.textMuted,
        fontSize: 12,
        marginTop: 2,
    },
    logoutText: {
        color: Colors.textMuted,
        fontSize: 13,
    },
    errorBox: {
        backgroundColor: Colors.error + "15",
        borderWidth: 1,
        borderColor: Colors.error + "40",
        borderRadius: BorderRadius.sm,
        padding: Spacing.md,
        marginHorizontal: Spacing.xl,
        marginBottom: Spacing.md,
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
    },
    errorText: {
        color: Colors.error,
        fontSize: 13,
        flex: 1,
    },
    errorClose: {
        color: Colors.error,
        fontSize: 14,
        fontWeight: "700",
        marginLeft: Spacing.sm,
        padding: Spacing.xs,
    },
    scrollContent: {
        padding: Spacing.xl,
    },
    heroSection: {
        alignItems: "center",
        marginBottom: Spacing.xxl,
        paddingVertical: Spacing.xl,
    },
    pulseContainer: {
        width: 80,
        height: 80,
        justifyContent: "center",
        alignItems: "center",
        marginBottom: Spacing.lg,
    },
    pulseRing: {
        position: "absolute",
        width: 60,
        height: 60,
        borderRadius: 30,
        borderWidth: 2,
        borderColor: Colors.success,
    },
    pulseDot: {
        width: 20,
        height: 20,
        borderRadius: 10,
        backgroundColor: Colors.success,
    },
    connectedText: {
        fontSize: 20,
        fontWeight: "700",
        letterSpacing: 2,
        marginBottom: Spacing.xs,
    },
    serverInfo: {
        color: Colors.textSecondary,
        fontSize: 14,
        marginTop: Spacing.xs,
    },
    detailsCard: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.lg,
        padding: Spacing.xl,
        borderWidth: 1,
        borderColor: Colors.border,
        marginBottom: Spacing.xxl,
    },
    detailsTitle: {
        color: Colors.textSecondary,
        fontSize: 13,
        fontWeight: "600",
        marginBottom: Spacing.lg,
        textTransform: "uppercase",
        letterSpacing: 1,
    },
    detailRow: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        paddingVertical: Spacing.sm,
        borderBottomWidth: 1,
        borderBottomColor: Colors.border + "40",
    },
    detailLabel: {
        color: Colors.textMuted,
        fontSize: 13,
    },
    detailValue: {
        color: Colors.textPrimary,
        fontSize: 13,
        fontFamily: Platform.OS === "ios" ? "Menlo" : "monospace",
    },
    sectionHeading: {
        color: Colors.textMuted,
        fontSize: 11,
        fontWeight: "600",
        letterSpacing: 1.5,
        marginBottom: Spacing.md,
    },
    switchText: {
        color: Colors.accent,
        fontSize: 13,
        fontWeight: "500",
    },
    footer: {
        padding: Spacing.xl,
        borderTopWidth: 1,
        borderTopColor: Colors.border,
    },
    disconnectButton: {
        backgroundColor: Colors.danger,
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
    },
    disconnectButtonText: {
        color: Colors.textPrimary,
        fontSize: 15,
        fontWeight: "600",
    },
});
