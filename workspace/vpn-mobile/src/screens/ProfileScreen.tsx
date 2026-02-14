import React, { useState, useEffect } from "react";
import {
    View,
    Text,
    TextInput,
    TouchableOpacity,
    StyleSheet,
    ScrollView,
    Alert,
    ActivityIndicator,
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
    const {
        user,
        connectedServer,
        apiUrl,
        isLoading,
        logout,
        refreshProfile,
        updateProfile,
        deleteAccount,
    } = useAuth();

    const [editing, setEditing] = useState(false);
    const [username, setUsername] = useState(user?.username ?? "");
    const [email, setEmail] = useState(user?.email ?? "");
    const [password, setPassword] = useState("");

    useEffect(() => {
        refreshProfile();
    }, [refreshProfile]);

    useEffect(() => {
        if (user) {
            setUsername(user.username);
            setEmail(user.email);
        }
    }, [user]);

    const handleSave = async () => {
        if (!username.trim() || !email.trim() || !password.trim()) {
            Alert.alert("Erreur", "Tous les champs sont requis.");
            return;
        }
        if (password.length < 8) {
            Alert.alert(
                "Erreur",
                "Le mot de passe doit contenir au moins 8 caracteres.",
            );
            return;
        }
        await updateProfile(username.trim(), email.trim(), password);
        setPassword("");
        setEditing(false);
    };

    const handleDelete = () => {
        Alert.alert(
            "Supprimer le compte",
            "Cette action est irreversible. Toutes vos donnees seront supprimees.",
            [
                { text: "Annuler", style: "cancel" },
                {
                    text: "Supprimer",
                    style: "destructive",
                    onPress: deleteAccount,
                },
            ],
        );
    };

    const handleLogout = () => {
        Alert.alert("Deconnexion", "Voulez-vous vous deconnecter ?", [
            { text: "Annuler", style: "cancel" },
            { text: "Deconnecter", style: "destructive", onPress: logout },
        ]);
    };

    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.accentBar} />

            <View style={styles.header}>
                <TouchableOpacity onPress={() => navigation.goBack()}>
                    <Text style={styles.backText}>Retour</Text>
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Profil</Text>
                <TouchableOpacity
                    onPress={() => {
                        if (editing) {
                            setUsername(user?.username ?? "");
                            setEmail(user?.email ?? "");
                            setPassword("");
                        }
                        setEditing(!editing);
                    }}
                >
                    <Text style={styles.backText}>
                        {editing ? "Annuler" : "Modifier"}
                    </Text>
                </TouchableOpacity>
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

                {editing ? (
                    <>
                        <Text style={styles.sectionHeading}>
                            MODIFIER LE PROFIL
                        </Text>
                        <View style={styles.card}>
                            <Text style={styles.fieldLabel}>
                                Nom d'utilisateur
                            </Text>
                            <TextInput
                                style={styles.input}
                                value={username}
                                onChangeText={setUsername}
                                placeholder="Nom d'utilisateur"
                                placeholderTextColor={Colors.textMuted}
                                autoCapitalize="none"
                            />
                            <Text style={styles.fieldLabel}>Email</Text>
                            <TextInput
                                style={styles.input}
                                value={email}
                                onChangeText={setEmail}
                                placeholder="Email"
                                placeholderTextColor={Colors.textMuted}
                                autoCapitalize="none"
                                keyboardType="email-address"
                            />
                            <Text style={styles.fieldLabel}>
                                Nouveau mot de passe
                            </Text>
                            <TextInput
                                style={styles.input}
                                value={password}
                                onChangeText={setPassword}
                                placeholder="Mot de passe (min. 8 car.)"
                                placeholderTextColor={Colors.textMuted}
                                secureTextEntry
                            />
                            <TouchableOpacity
                                style={styles.saveButton}
                                onPress={handleSave}
                                disabled={isLoading}
                            >
                                {isLoading ? (
                                    <ActivityIndicator
                                        color={Colors.background}
                                    />
                                ) : (
                                    <Text style={styles.saveButtonText}>
                                        Enregistrer
                                    </Text>
                                )}
                            </TouchableOpacity>
                        </View>
                    </>
                ) : (
                    <>
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
                                value={
                                    connectedServer ? "Connecte" : "Deconnecte"
                                }
                            />
                            {connectedServer && (
                                <InfoRow
                                    label="Serveur actif"
                                    value={connectedServer.name}
                                />
                            )}
                            <InfoRow label="Serveur API" value={apiUrl} />
                        </View>
                    </>
                )}

                {/* Actions */}
                <TouchableOpacity
                    style={styles.logoutButton}
                    onPress={handleLogout}
                >
                    <Text style={styles.logoutButtonText}>Se deconnecter</Text>
                </TouchableOpacity>

                <TouchableOpacity
                    style={styles.deleteButton}
                    onPress={handleDelete}
                >
                    <Text style={styles.deleteButtonText}>
                        Supprimer le compte
                    </Text>
                </TouchableOpacity>
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
    scrollContent: {
        padding: Spacing.xl,
        paddingBottom: Spacing.xxxl,
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
    errorBox: {
        backgroundColor: Colors.danger + "20",
        borderRadius: BorderRadius.sm,
        padding: Spacing.md,
        marginBottom: Spacing.md,
    },
    errorText: {
        color: Colors.error,
        fontSize: 13,
        textAlign: "center",
    },
    fieldLabel: {
        color: Colors.textSecondary,
        fontSize: 12,
        marginBottom: Spacing.xs,
        marginTop: Spacing.md,
    },
    input: {
        backgroundColor: Colors.background,
        borderRadius: BorderRadius.sm,
        borderWidth: 1,
        borderColor: Colors.border,
        color: Colors.textPrimary,
        fontSize: 14,
        paddingHorizontal: Spacing.md,
        paddingVertical: Spacing.sm,
    },
    saveButton: {
        backgroundColor: Colors.accent,
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
        marginTop: Spacing.xl,
    },
    saveButtonText: {
        color: Colors.background,
        fontSize: 15,
        fontWeight: "600",
    },
    logoutButton: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.md,
        borderWidth: 1,
        borderColor: Colors.border,
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
    deleteButton: {
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
        marginTop: Spacing.md,
    },
    deleteButtonText: {
        color: Colors.danger,
        fontSize: 14,
        fontWeight: "500",
    },
});
