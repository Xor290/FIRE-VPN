import React from "react";
import {
    View,
    Text,
    TouchableOpacity,
    StyleSheet,
    ScrollView,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { NativeStackNavigationProp } from "@react-navigation/native-stack";
import { Colors, Spacing, BorderRadius } from "../theme";
import { useAuth } from "../contexts/AuthContext";

type Props = {
    navigation: NativeStackNavigationProp<any>;
};

function InfoRow({ label, value }: { label: string; value: string }) {
    return (
        <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>{label}</Text>
            <Text style={styles.infoValue}>{value}</Text>
        </View>
    );
}

export function ProfileScreen({ navigation }: Props) {
    const { user, connectedServer, logout, apiUrl } = useAuth();

    return (
        <SafeAreaView style={styles.container}>
            {/* Accent bar */}
            <View style={styles.accentBar} />

            {/* Header */}
            <View style={styles.header}>
                <TouchableOpacity onPress={() => navigation.goBack()}>
                    <Text style={styles.backText}>Retour</Text>
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Profil</Text>
                <View style={styles.headerSpacer} />
            </View>

            <ScrollView contentContainerStyle={styles.scrollContent}>
                {/* Avatar */}
                <View style={styles.avatarSection}>
                    <View style={styles.avatar}>
                        <Text style={styles.avatarText}>
                            {user?.username?.charAt(0).toUpperCase() ?? "?"}
                        </Text>
                    </View>
                    <Text style={styles.username}>{user?.username ?? "-"}</Text>
                    <Text style={styles.email}>{user?.email ?? "-"}</Text>
                </View>

                {/* Account info */}
                <Text style={styles.sectionHeading}>
                    INFORMATIONS DU COMPTE
                </Text>
                <View style={styles.card}>
                    <InfoRow
                        label="Identifiant"
                        value={`#${user?.id ?? "-"}`}
                    />
                    <InfoRow
                        label="Nom d'utilisateur"
                        value={user?.username ?? "-"}
                    />
                    <InfoRow label="Email" value={user?.email ?? "-"} />
                </View>

                {/* Connection info */}
                <Text style={styles.sectionHeading}>CONNEXION VPN</Text>
                <View style={styles.card}>
                    <InfoRow
                        label="Statut"
                        value={connectedServer ? "Connecte" : "Deconnecte"}
                    />
                    {connectedServer && (
                        <InfoRow
                            label="Serveur actif"
                            value={connectedServer.name}
                        />
                    )}
                    <InfoRow label="Serveur API" value={apiUrl} />
                </View>
            </ScrollView>
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
        paddingVertical: Spacing.lg,
    },
    backText: {
        color: Colors.accent,
        fontSize: 14,
    },
    headerTitle: {
        color: Colors.textPrimary,
        fontSize: 17,
        fontWeight: "600",
    },
    headerSpacer: {
        width: 50,
    },
    scrollContent: {
        padding: Spacing.xl,
    },
    avatarSection: {
        alignItems: "center",
        marginBottom: Spacing.xxl,
    },
    avatar: {
        width: 72,
        height: 72,
        borderRadius: 36,
        backgroundColor: Colors.accent,
        justifyContent: "center",
        alignItems: "center",
        marginBottom: Spacing.md,
    },
    avatarText: {
        color: Colors.background,
        fontSize: 28,
        fontWeight: "700",
    },
    username: {
        color: Colors.textPrimary,
        fontSize: 20,
        fontWeight: "600",
        marginBottom: Spacing.xs,
    },
    email: {
        color: Colors.textSecondary,
        fontSize: 14,
    },
    sectionHeading: {
        color: Colors.textMuted,
        fontSize: 11,
        fontWeight: "600",
        letterSpacing: 1.5,
        marginBottom: Spacing.sm,
        marginTop: Spacing.lg,
    },
    card: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.lg,
        padding: Spacing.xl,
        borderWidth: 1,
        borderColor: Colors.border,
    },
    infoRow: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        paddingVertical: Spacing.sm,
        borderBottomWidth: 1,
        borderBottomColor: Colors.border + "40",
    },
    infoLabel: {
        color: Colors.textMuted,
        fontSize: 13,
    },
    infoValue: {
        color: Colors.textPrimary,
        fontSize: 13,
        fontWeight: "500",
    },
    logoutButton: {
        backgroundColor: Colors.danger,
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
        marginTop: Spacing.xxxl,
    },
    logoutButtonText: {
        color: Colors.textPrimary,
        fontSize: 15,
        fontWeight: "600",
    },
});
