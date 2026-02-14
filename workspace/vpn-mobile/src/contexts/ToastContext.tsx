import React, {
    createContext,
    useContext,
    useState,
    useCallback,
    useRef,
} from "react";
import {
    Animated,
    Text,
    StyleSheet,
    TouchableOpacity,
    Dimensions,
} from "react-native";
import { Colors, Spacing, BorderRadius } from "../theme";

type ToastType = "error" | "success" | "info";

interface ToastData {
    message: string;
    type: ToastType;
}

interface ToastContextValue {
    showToast: (message: string, type?: ToastType) => void;
}

const ToastContext = createContext<ToastContextValue | null>(null);

const SCREEN_WIDTH = Dimensions.get("window").width;
const TOAST_DURATION = 3000;

const TYPE_COLORS: Record<ToastType, string> = {
    error: Colors.error,
    success: Colors.success,
    info: Colors.accent,
};

export function ToastProvider({ children }: { children: React.ReactNode }) {
    const [toast, setToast] = useState<ToastData | null>(null);
    const translateX = useRef(new Animated.Value(SCREEN_WIDTH)).current;
    const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    const hideToast = useCallback(() => {
        Animated.timing(translateX, {
            toValue: SCREEN_WIDTH,
            duration: 300,
            useNativeDriver: true,
        }).start(() => setToast(null));
    }, [translateX]);

    const showToast = useCallback(
        (message: string, type: ToastType = "error") => {
            if (timeoutRef.current) clearTimeout(timeoutRef.current);

            setToast({ message, type });
            translateX.setValue(SCREEN_WIDTH);

            Animated.spring(translateX, {
                toValue: 0,
                useNativeDriver: true,
                tension: 80,
                friction: 10,
            }).start();

            timeoutRef.current = setTimeout(hideToast, TOAST_DURATION);
        },
        [translateX, hideToast],
    );

    return (
        <ToastContext.Provider value={{ showToast }}>
            {children}
            {toast && (
                <Animated.View
                    style={[
                        styles.container,
                        {
                            transform: [{ translateX }],
                            borderLeftColor: TYPE_COLORS[toast.type],
                        },
                    ]}
                >
                    <TouchableOpacity
                        style={styles.content}
                        onPress={hideToast}
                        activeOpacity={0.8}
                    >
                        <Text style={styles.message}>{toast.message}</Text>
                    </TouchableOpacity>
                </Animated.View>
            )}
        </ToastContext.Provider>
    );
}

export function useToast(): ToastContextValue {
    const ctx = useContext(ToastContext);
    if (!ctx) throw new Error("useToast must be used within ToastProvider");
    return ctx;
}

const styles = StyleSheet.create({
    container: {
        position: "absolute",
        top: 60,
        right: Spacing.lg,
        left: Spacing.lg,
        backgroundColor: Colors.card,
        borderRadius: BorderRadius.md,
        borderLeftWidth: 4,
        elevation: 10,
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 4 },
        shadowOpacity: 0.3,
        shadowRadius: 8,
    },
    content: {
        padding: Spacing.lg,
    },
    message: {
        color: Colors.textPrimary,
        fontSize: 14,
        fontWeight: "500",
    },
});
