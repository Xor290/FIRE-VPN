export const Colors = {
    background: "#1a1a1a",
    card: "#161B22",
    accent: "#58A6FF",
    accentHover: "#6EB4FF",
    accentDim: "#1A3A5C",
    success: "#3FB950",
    successDim: "#235C2D",
    error: "#F85149",
    danger: "#DA3633",
    textPrimary: "#E6EDF3",
    textSecondary: "#8B949E",
    textMuted: "#6E7681",
    border: "#303C3D",
};

export const Spacing = {
    xs: 4,
    sm: 8,
    md: 12,
    lg: 16,
    xl: 20,
    xxl: 24,
    xxxl: 32,
};

export const BorderRadius = {
    sm: 6,
    md: 10,
    lg: 12,
};

export const COUNTRY_FLAGS: Record<string, string> = {
    france: "\u{1F1EB}\u{1F1F7}",
    germany: "\u{1F1E9}\u{1F1EA}",
    "united states": "\u{1F1FA}\u{1F1F8}",
    usa: "\u{1F1FA}\u{1F1F8}",
    "united kingdom": "\u{1F1EC}\u{1F1E7}",
    uk: "\u{1F1EC}\u{1F1E7}",
    canada: "\u{1F1E8}\u{1F1E6}",
    netherlands: "\u{1F1F3}\u{1F1F1}",
    japan: "\u{1F1EF}\u{1F1F5}",
    australia: "\u{1F1E6}\u{1F1FA}",
    singapore: "\u{1F1F8}\u{1F1EC}",
    switzerland: "\u{1F1E8}\u{1F1ED}",
    sweden: "\u{1F1F8}\u{1F1EA}",
    brazil: "\u{1F1E7}\u{1F1F7}",
    india: "\u{1F1EE}\u{1F1F3}",
    spain: "\u{1F1EA}\u{1F1F8}",
    italy: "\u{1F1EE}\u{1F1F9}",
    poland: "\u{1F1F5}\u{1F1F1}",
    finland: "\u{1F1EB}\u{1F1EE}",
    norway: "\u{1F1F3}\u{1F1F4}",
};

export function getCountryFlag(country: string): string {
    return COUNTRY_FLAGS[country.toLowerCase()] ?? "\u{1F310}";
}
