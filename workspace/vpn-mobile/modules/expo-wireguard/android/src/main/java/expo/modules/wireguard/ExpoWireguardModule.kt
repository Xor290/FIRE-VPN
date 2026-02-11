package expo.modules.wireguard

import android.app.Activity
import android.content.Intent
import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import expo.modules.kotlin.Promise
import expo.modules.kotlin.exception.CodedException
import com.wireguard.android.backend.GoBackend
import com.wireguard.android.backend.Tunnel
import com.wireguard.config.Config
import java.io.BufferedReader
import java.io.StringReader

class ExpoWireguardModule : Module() {

    private var backend: GoBackend? = null
    private var currentTunnel: WgTunnel? = null

    private class WgTunnel(private val tunnelName: String) : Tunnel {
        override fun getName(): String = tunnelName
        override fun onStateChange(newState: Tunnel.State) {}
    }

    override fun definition() = ModuleDefinition {
        Name("ExpoWireguard")

        Events("onStatusChange")

        AsyncFunction("connect") { configStr: String, promise: Promise ->
            try {
                val context = appContext.reactContext
                    ?: throw CodedException("NO_CONTEXT", "Application context not available", null)

                if (backend == null) {
                    backend = GoBackend(context)
                }

                val vpnIntent = GoBackend.VpnService.prepare(context)
                if (vpnIntent != null) {
                    val activity = appContext.activityProvider?.currentActivity
                        ?: throw CodedException("NO_ACTIVITY", "No activity available to request VPN permission", null)
                    pendingConfig = configStr
                    pendingPromise = promise
                    activity.startActivityForResult(vpnIntent, VPN_REQUEST_CODE)
                    return@AsyncFunction
                }

                connectTunnel(configStr)
                promise.resolve(true)
            } catch (e: Exception) {
                promise.reject(CodedException("CONNECT_ERROR", e.message ?: "Failed to connect", e))
            }
        }

        AsyncFunction("disconnect") { promise: Promise ->
            try {
                val tunnel = currentTunnel
                val be = backend
                if (tunnel != null && be != null) {
                    be.setState(tunnel, Tunnel.State.DOWN, null)
                    currentTunnel = null
                    sendEvent("onStatusChange", mapOf("status" to "DOWN"))
                }
                promise.resolve(true)
            } catch (e: Exception) {
                promise.reject(CodedException("DISCONNECT_ERROR", e.message ?: "Failed to disconnect", e))
            }
        }

        Function("getStatus") {
            if (currentTunnel != null && backend != null) {
                try {
                    val state = backend!!.getState(currentTunnel!!)
                    return@Function state.toString()
                } catch (_: Exception) {
                }
            }
            return@Function "DOWN"
        }

        OnActivityResult { _, payload ->
            if (payload.requestCode == VPN_REQUEST_CODE) {
                val config = pendingConfig
                val promise = pendingPromise
                pendingConfig = null
                pendingPromise = null

                if (payload.resultCode == Activity.RESULT_OK && config != null && promise != null) {
                    try {
                        connectTunnel(config)
                        promise.resolve(true)
                    } catch (e: Exception) {
                        promise.reject(
                            CodedException(
                                "CONNECT_ERROR",
                                e.message ?: "Failed to connect after permission",
                                e
                            )
                        )
                    }
                } else {
                    promise?.reject(CodedException("VPN_PERMISSION_DENIED", "User denied VPN permission", null))
                }
            }
        }
    }

    private fun connectTunnel(configStr: String) {
        val be = backend ?: throw Exception("Backend not initialized")

        if (currentTunnel != null) {
            try {
                be.setState(currentTunnel!!, Tunnel.State.DOWN, null)
            } catch (_: Exception) {
            }
        }

        val config = Config.parse(BufferedReader(StringReader(configStr)))
        val tunnel = WgTunnel("fire-vpn")
        be.setState(tunnel, Tunnel.State.UP, config)
        currentTunnel = tunnel

        sendEvent("onStatusChange", mapOf("status" to "UP"))
    }

    companion object {
        private const val VPN_REQUEST_CODE = 24601
        private var pendingConfig: String? = null
        private var pendingPromise: Promise? = null
    }
}
