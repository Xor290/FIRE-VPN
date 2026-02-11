export type TunnelStatus = "UP" | "DOWN";

export type ExpoWireguardModuleEvents = {
    onStatusChange: (params: { status: TunnelStatus }) => void;
};
