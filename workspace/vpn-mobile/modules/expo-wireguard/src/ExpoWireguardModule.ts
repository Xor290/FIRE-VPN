import { NativeModule, requireNativeModule } from "expo";

import { ExpoWireguardModuleEvents } from "./ExpoWireguard.types";

declare class ExpoWireguardModule extends NativeModule<ExpoWireguardModuleEvents> {
    connect(config: string): Promise<boolean>;
    disconnect(): Promise<boolean>;
    getStatus(): string;
}

export default requireNativeModule<ExpoWireguardModule>("ExpoWireguard");
