import React, { useEffect, useState } from "react";
import {
    View,
    Text,
    FlatList,
    TouchableOpacity,
    StyleSheet,
    ActivityIndicator,
    RefreshControl,
    ImageBackground,
    Image,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { NativeStackNavigationProp } from "@react-navigation/native-stack";
import { Colors, Spacing, BorderRadius } from "../theme";
import { useAuth } from "../contexts/AuthContext";
import { Server } from "../types";
import { ServerCard } from "../components/ServerCard";

type Props = {
    navigation: NativeStackNavigationProp<any>;
};

export function ServerListScreen({ navigation }: Props) {
    const {
        servers,
        isLoading,
        loadServers,
        connectToServer,
        disconnectFromServer,
        connectedServer,
        logout,
        user,
    } = useAuth();

    const [selectedServer, setSelectedServer] = useState<Server | null>(null);
    const [isConnecting, setIsConnecting] = useState(false);

    const isConnected = !!connectedServer;

    useEffect(() => {
        loadServers();
    }, []);

    const handleConnect = async () => {
        if (!selectedServer || isConnecting) return;
        setIsConnecting(true);
        await connectToServer(selectedServer);
        setIsConnecting(false);
    };

    const handleDisconnect = async () => {
        if (isConnecting) return;
        setIsConnecting(true);
        await disconnectFromServer();
        setIsConnecting(false);
    };

    const canConnect =
        selectedServer && selectedServer.is_active && !isConnecting;

    return (
        <ImageBackground
            source={require("../../assets/bg-ghost.png")}
            style={styles.backgroundImage}
            imageStyle={{
                opacity: 0.15,
                resizeMode: "cover",
                height: "90%",
                width: "105%",
            }}
        >
            <SafeAreaView style={styles.container}>
                {/* Accent bar */}
                <View style={styles.accentBar} />

                {/* Header */}
                <View style={styles.header}>
                    <View style={styles.headerLeft}>
                        <Image
                            source={require("../../assets/logo-ghost.png")}
                            style={styles.headerLogo}
                        />
                        {user && (
                            <Text style={styles.userEmail}>{user.email}</Text>
                        )}
                    </View>

                    <View style={styles.headerActions}>
                        {/* Bouton Profil */}
                        <TouchableOpacity
                            onPress={() => navigation.navigate("Profile")}
                            style={styles.profileButton}
                        >
                            <Text style={styles.profileText}>Profil</Text>
                        </TouchableOpacity>

                        {/* Bouton Déconnexion */}
                        <TouchableOpacity onPress={logout}>
                            <Text style={styles.logoutText}>Deconnexion</Text>
                        </TouchableOpacity>
                    </View>
                </View>

                {/* Section heading */}
                <Text style={styles.sectionHeading}>SERVEURS DISPONIBLES</Text>

                {/* Server list */}
                <FlatList
                    data={servers}
                    keyExtractor={(item) => item.id.toString()}
                    renderItem={({ item }) => (
                        <ServerCard
                            server={item}
                            selected={selectedServer?.id === item.id}
                            onPress={() => setSelectedServer(item)}
                        />
                    )}
                    contentContainerStyle={[
                        styles.list,
                        servers.length === 0 && styles.listEmpty,
                    ]}
                    refreshControl={
                        <RefreshControl
                            refreshing={isLoading}
                            onRefresh={loadServers}
                            tintColor={Colors.accent}
                            colors={[Colors.accent]}
                        />
                    }
                    ListEmptyComponent={
                        <View style={styles.centered}>
                            {isLoading ? (
                                <>
                                    <ActivityIndicator
                                        size="large"
                                        color={Colors.accent}
                                    />
                                    <Text style={styles.loadingText}>
                                        Chargement des serveurs...
                                    </Text>
                                </>
                            ) : (
                                <>
                                    <Text style={styles.emptyText}>
                                        Aucun serveur disponible
                                    </Text>
                                    <Text style={styles.emptySubtext}>
                                        Tirez vers le bas pour actualiser
                                    </Text>
                                    <TouchableOpacity
                                        style={styles.retryButton}
                                        onPress={loadServers}
                                    >
                                        <Text style={styles.retryText}>
                                            Reessayer
                                        </Text>
                                    </TouchableOpacity>
                                </>
                            )}
                        </View>
                    }
                />

                {/* Connect / Disconnect button */}
                <View style={styles.footer}>
                    {isConnected ? (
                        <TouchableOpacity
                            style={styles.disconnectButton}
                            onPress={handleDisconnect}
                            disabled={isConnecting}
                            activeOpacity={0.8}
                        >
                            {isConnecting ? (
                                <ActivityIndicator color={Colors.textPrimary} />
                            ) : (
                                <Text style={styles.disconnectButtonText}>
                                    Se deconnecter
                                </Text>
                            )}
                        </TouchableOpacity>
                    ) : (
                        <TouchableOpacity
                            style={[
                                styles.connectButton,
                                !canConnect && styles.connectButtonDisabled,
                            ]}
                            onPress={handleConnect}
                            disabled={!canConnect}
                            activeOpacity={0.8}
                        >
                            {isConnecting ? (
                                <ActivityIndicator color={Colors.textPrimary} />
                            ) : (
                                <Text
                                    style={[
                                        styles.connectButtonText,
                                        !canConnect &&
                                            styles.connectButtonTextDisabled,
                                    ]}
                                >
                                    Se connecter
                                </Text>
                            )}
                        </TouchableOpacity>
                    )}
                </View>
            </SafeAreaView>
        </ImageBackground>
    );
}

const styles = StyleSheet.create({
    backgroundImage: {
        flex: 1,
        backgroundColor: Colors.background,
    },
    container: {
        flex: 1,
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
    headerActions: {
        flexDirection: "row",
        alignItems: "center",
        gap: Spacing.md,
    },
    profileButton: {
        paddingHorizontal: Spacing.md,
        paddingVertical: Spacing.sm,
        borderRadius: BorderRadius.sm,
        backgroundColor: Colors.accent,
        marginRight: Spacing.sm,
    },
    profileText: {
        color: Colors.textPrimary,
        fontSize: 13,
        fontWeight: "500",
    },
    headerLeft: {
        alignItems: "flex-start",
    },
    headerLogo: {
        width: 40,
        height: 40,
        borderRadius: 20,
    },
    userEmail: {
        color: Colors.textMuted,
        fontSize: 11,
        marginTop: Spacing.xs,
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
    sectionHeading: {
        color: Colors.textMuted,
        fontSize: 11,
        fontWeight: "600",
        letterSpacing: 1.5,
        paddingHorizontal: Spacing.xl,
        marginBottom: Spacing.md,
    },
    centered: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
    },
    loadingText: {
        color: Colors.textSecondary,
        marginTop: Spacing.md,
        fontSize: 14,
    },
    emptyText: {
        color: Colors.textMuted,
        fontSize: 14,
    },
    emptySubtext: {
        color: Colors.textMuted,
        fontSize: 12,
        marginTop: Spacing.sm,
    },
    retryButton: {
        marginTop: Spacing.lg,
        paddingHorizontal: Spacing.xl,
        paddingVertical: Spacing.sm,
        borderRadius: BorderRadius.md,
        borderWidth: 1,
        borderColor: Colors.accent,
    },
    retryText: {
        color: Colors.accent,
        fontSize: 13,
        fontWeight: "500",
    },
    listEmpty: {
        flexGrow: 1,
        justifyContent: "center",
        alignItems: "center",
    },
    list: {
        paddingHorizontal: Spacing.xl,
        paddingBottom: Spacing.md,
    },
    footer: {
        padding: Spacing.xl,
        borderTopWidth: 1,
        borderTopColor: Colors.border,
    },
    connectButton: {
        backgroundColor: Colors.accent,
        borderRadius: BorderRadius.md,
        height: 44,
        justifyContent: "center",
        alignItems: "center",
    },
    connectButtonDisabled: {
        backgroundColor: Colors.accentDim,
    },
    connectButtonText: {
        color: Colors.textPrimary,
        fontSize: 15,
        fontWeight: "600",
    },
    connectButtonTextDisabled: {
        color: Colors.textMuted,
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
