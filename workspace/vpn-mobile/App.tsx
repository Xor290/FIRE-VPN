import React from "react";
import { StatusBar } from "expo-status-bar";
import { NavigationContainer, DefaultTheme } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { SafeAreaProvider } from "react-native-safe-area-context";
import { AuthProvider, useAuth } from "./src/contexts/AuthContext";
import { ToastProvider } from "./src/contexts/ToastContext";
import { LoginScreen } from "./src/screens/LoginScreen";
import { ServerListScreen } from "./src/screens/ServerListScreen";
import { ConnectedScreen } from "./src/screens/ConnectedScreen";
import { ProfileScreen } from "./src/screens/ProfileScreen";
import { Colors } from "./src/theme";

const Stack = createNativeStackNavigator();

const DarkTheme = {
    ...DefaultTheme,
    dark: true,
    colors: {
        ...DefaultTheme.colors,
        primary: Colors.accent,
        background: Colors.background,
        card: Colors.card,
        text: Colors.textPrimary,
        border: Colors.border,
        notification: Colors.accent,
    },
};

function AppNavigator() {
    const { token, connectedServer } = useAuth();

    return (
        <Stack.Navigator
            screenOptions={{ headerShown: false, animation: "fade" }}
        >
            {!token ? (
                <Stack.Screen name="Login" component={LoginScreen} />
            ) : (
                <>
                    {connectedServer ? (
                        <Stack.Screen
                            name="Connected"
                            component={ConnectedScreen}
                        />
                    ) : (
                        <Stack.Screen
                            name="ServerList"
                            component={ServerListScreen}
                        />
                    )}
                    <Stack.Screen name="Profile" component={ProfileScreen} />
                </>
            )}
        </Stack.Navigator>
    );
}

export default function App() {
    return (
        <SafeAreaProvider>
            <ToastProvider>
                <AuthProvider>
                    <NavigationContainer theme={DarkTheme}>
                        <StatusBar style="light" />
                        <AppNavigator />
                    </NavigationContainer>
                </AuthProvider>
            </ToastProvider>
        </SafeAreaProvider>
    );
}
