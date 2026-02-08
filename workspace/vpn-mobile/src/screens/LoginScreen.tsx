import React, { useState } from "react";
import {
    View,
    Text,
    TextInput,
    TouchableOpacity,
    StyleSheet,
    KeyboardAvoidingView,
    Platform,
    ScrollView,
    ActivityIndicator,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { Colors, Spacing, BorderRadius } from "../theme";
import { useAuth } from "../contexts/AuthContext";

export function LoginScreen() {
    const {
        login,
        register,
        isLoading,
        error,
        clearError,
        savedEmail,
        apiUrl,
        setApiUrl,
    } = useAuth();

    const [isRegister, setIsRegister] = useState(false);
    const [email, setEmail] = useState(savedEmail);
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const [showApiUrl, setShowApiUrl] = useState(false);

    const handleSubmit = () => {
        if (isLoading) return;
        clearError();
        if (isRegister) {
            register(username, email, password);
        } else {
            login(email, password);
        }
    };

    const toggleMode = () => {
        setIsRegister(!isRegister);
        clearError();
    };

    return (
        <SafeAreaView style={styles.container}>
            <KeyboardAvoidingView
                style={styles.flex}
                behavior={Platform.OS === "ios" ? "padding" : "height"}
            >
                <ScrollView
                    contentContainerStyle={styles.scrollContent}
                    keyboardShouldPersistTaps="handled"
                >
                    {/* Accent bar */}
                    <View style={styles.accentBar} />

                    {/* Header */}
                    <View style={styles.header}>
                        <Text style={styles.title}>FIRE VPN</Text>
                        <Text style={styles.subtitle}>Connexion securisee</Text>
                    </View>

                    {/* Card */}
                    <View style={styles.card}>
                        {/* Error */}
                        {error && (
                            <View style={styles.errorBox}>
                                <Text style={styles.errorText}>{error}</Text>
                            </View>
                        )}

                        {/* Username (register only) */}
                        {isRegister && (
                            <View style={styles.field}>
                                <Text style={styles.label}>
                                    Nom d'utilisateur
                                </Text>
                                <TextInput
                                    style={styles.input}
                                    value={username}
                                    onChangeText={setUsername}
                                    placeholder="Nom d'utilisateur"
                                    placeholderTextColor={Colors.textMuted}
                                    autoCapitalize="none"
                                    autoCorrect={false}
                                />
                            </View>
                        )}

                        {/* Email */}
                        <View style={styles.field}>
                            <Text style={styles.label}>Email</Text>
                            <TextInput
                                style={styles.input}
                                value={email}
                                onChangeText={setEmail}
                                placeholder="email@exemple.com"
                                placeholderTextColor={Colors.textMuted}
                                keyboardType="email-address"
                                autoCapitalize="none"
                                autoCorrect={false}
                            />
                        </View>

                        {/* Password */}
                        <View style={styles.field}>
                            <Text style={styles.label}>Mot de passe</Text>
                            <TextInput
                                style={styles.input}
                                value={password}
                                onChangeText={setPassword}
                                placeholder="Mot de passe"
                                placeholderTextColor={Colors.textMuted}
                                secureTextEntry
                                onSubmitEditing={handleSubmit}
                            />
                        </View>

                        {/* Submit button */}
                        <TouchableOpacity
                            style={[
                                styles.button,
                                isLoading && styles.buttonDisabled,
                            ]}
                            onPress={handleSubmit}
                            disabled={isLoading}
                            activeOpacity={0.8}
                        >
                            {isLoading ? (
                                <ActivityIndicator color={Colors.textPrimary} />
                            ) : (
                                <Text style={styles.buttonText}>
                                    {isRegister
                                        ? "Creer un compte"
                                        : "Se connecter"}
                                </Text>
                            )}
                        </TouchableOpacity>

                        {/* Toggle mode */}
                        <TouchableOpacity
                            onPress={toggleMode}
                            style={styles.toggleButton}
                        >
                            <Text style={styles.toggleText}>
                                {isRegister
                                    ? "Deja un compte ? Se connecter"
                                    : "Pas de compte ? S'inscrire"}
                            </Text>
                        </TouchableOpacity>
                    </View>

                    {/* API URL config */}
                    <TouchableOpacity
                        onPress={() => setShowApiUrl(!showApiUrl)}
                        style={styles.apiToggle}
                    >
                        <Text style={styles.apiToggleText}>
                            {showApiUrl ? "Masquer" : "Configurer"} l'URL du
                            serveur
                        </Text>
                    </TouchableOpacity>

                    {showApiUrl && (
                        <View style={styles.apiUrlCard}>
                            <Text style={styles.label}>URL de l'API</Text>
                            <TextInput
                                style={styles.input}
                                value={apiUrl}
                                onChangeText={setApiUrl}
                                placeholder="http://10.0.2.2:8080"
                                placeholderTextColor={Colors.textMuted}
                                autoCapitalize="none"
                                autoCorrect={false}
                                keyboardType="url"
                            />
                        </View>
                    )}
                </ScrollView>
            </KeyboardAvoidingView>
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: Colors.background,
    },
    flex: {
        flex: 1,
        backgroundColor: Colors.background,
    },
    scrollContent: {
        flexGrow: 1,
        justifyContent: "center",
        padding: Spacing.xl,
    },
    accentBar: {
        height: 3,
        backgroundColor: Colors.accent,
        borderRadius: 2,
        marginBottom: Spacing.xxxl,
        alignSelf: "center",
        width: 60,
    },
    header: {
        alignItems: "center",
        marginBottom: Spacing.xxl,
    },
    title: {
        fontSize: 28,
        fontWeight: "700",
        color: Colors.accent,
        letterSpacing: 2,
    },
    subtitle: {
        fontSize: 12,
        color: Colors.textMuted,
        marginTop: Spacing.xs,
        textTransform: "uppercase",
        letterSpacing: 1,
    },
    card: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.lg,
        padding: Spacing.xl,
        borderWidth: 1,
        borderColor: Colors.border,
    },
    errorBox: {
        backgroundColor: Colors.error + "15",
        borderWidth: 1,
        borderColor: Colors.error + "40",
        borderRadius: BorderRadius.sm,
        padding: Spacing.md,
        marginBottom: Spacing.lg,
    },
    errorText: {
        color: Colors.error,
        fontSize: 13,
    },
    field: {
        marginBottom: Spacing.lg,
    },
    label: {
        color: Colors.textSecondary,
        fontSize: 13,
        marginBottom: Spacing.sm,
        fontWeight: "500",
    },
    input: {
        backgroundColor: Colors.background,
        color: Colors.textPrimary,
        borderRadius: BorderRadius.md,
        paddingHorizontal: Spacing.md,
        paddingVertical: Spacing.md,
        fontSize: 15,
        borderWidth: 1,
        borderColor: Colors.border,
    },
    button: {
        backgroundColor: Colors.accent,
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
        marginTop: Spacing.sm,
    },
    buttonDisabled: {
        backgroundColor: Colors.accentDim,
    },
    buttonText: {
        color: Colors.textPrimary,
        fontSize: 15,
        fontWeight: "600",
    },
    toggleButton: {
        marginTop: Spacing.lg,
        alignItems: "center",
    },
    toggleText: {
        color: Colors.accent,
        fontSize: 13,
    },
    apiToggle: {
        marginTop: Spacing.lg,
        alignItems: "center",
    },
    apiToggleText: {
        color: Colors.textMuted,
        fontSize: 12,
    },
    apiUrlCard: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.lg,
        padding: Spacing.lg,
        marginTop: Spacing.sm,
        borderWidth: 1,
        borderColor: Colors.border,
    },
});
