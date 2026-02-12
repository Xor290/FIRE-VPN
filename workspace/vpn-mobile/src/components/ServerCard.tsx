import React, { useEffect, useRef } from "react";
import {
    View,
    Text,
    StyleSheet,
    TouchableOpacity,
    Animated,
} from "react-native";
import { Server } from "../types";
import { Colors, Spacing, BorderRadius, getCountryFlag } from "../theme";
import { StatusPill } from "./StatusPill";

interface ServerCardProps {
    server: Server;
    selected?: boolean;
    onPress: () => void;
    rightElement?: React.ReactNode;
}

export function ServerCard({
    server,
    selected,
    onPress,
    rightElement,
}: ServerCardProps) {
    const wave = useRef(new Animated.Value(0)).current;

    useEffect(() => {
        const animation = Animated.loop(
            Animated.sequence([
                Animated.timing(wave, {
                    toValue: 1,
                    duration: 600,
                    useNativeDriver: true,
                }),
                Animated.timing(wave, {
                    toValue: -1,
                    duration: 600,
                    useNativeDriver: true,
                }),
                Animated.timing(wave, {
                    toValue: 0,
                    duration: 600,
                    useNativeDriver: true,
                }),
            ]),
        );
        animation.start();
        return () => animation.stop();
    }, [wave]);

    const rotate = wave.interpolate({
        inputRange: [-1, 0, 1],
        outputRange: ["-6deg", "0deg", "6deg"],
    });

    const scaleX = wave.interpolate({
        inputRange: [-1, 0, 1],
        outputRange: [0.95, 1, 0.97],
    });

    return (
        <TouchableOpacity
            style={[styles.card, selected && styles.cardSelected]}
            onPress={onPress}
            activeOpacity={0.7}
        >
            <View style={styles.row}>
                <Animated.Text
                    style={[
                        styles.flag,
                        {
                            transform: [{ rotate }, { scaleX }],
                        },
                    ]}
                >
                    {getCountryFlag(server.country)}
                </Animated.Text>
                <View style={styles.info}>
                    <Text style={styles.name}>{server.name}</Text>
                    <Text style={styles.details}>{server.country}</Text>
                </View>
                <View style={styles.right}>
                    {rightElement ?? (
                        <StatusPill
                            label={server.is_active ? "En ligne" : "Hors ligne"}
                            color={
                                server.is_active
                                    ? Colors.success
                                    : Colors.textMuted
                            }
                        />
                    )}
                </View>
            </View>
        </TouchableOpacity>
    );
}

const styles = StyleSheet.create({
    card: {
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.lg,
        padding: Spacing.lg,
        marginBottom: Spacing.sm,
        borderWidth: 1,
        borderColor: Colors.border,
    },
    cardSelected: {
        borderColor: Colors.accent,
        backgroundColor: Colors.accentDim + "30",
    },
    row: {
        flexDirection: "row",
        alignItems: "center",
    },
    flag: {
        fontSize: 28,
        marginRight: Spacing.md,
    },
    info: {
        flex: 1,
    },
    name: {
        color: Colors.textPrimary,
        fontSize: 16,
        fontWeight: "600",
        marginBottom: 2,
    },
    details: {
        color: Colors.textMuted,
        fontSize: 12,
    },
    right: {
        marginLeft: Spacing.sm,
    },
});
