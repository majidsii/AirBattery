//! Optional Linux session D-Bus bridge used by the GNOME Shell companion.

#[cfg(not(target_os = "linux"))]
use tauri::AppHandle;

#[cfg(not(target_os = "linux"))]
use crate::error::CommandError;

#[cfg(target_os = "linux")]
mod linux {
    use airbattery_dbus::{
        BackendSnapshot, ServiceHandle, ServiceRequest, ServiceSnapshot, start_service,
    };
    use tauri::{AppHandle, Manager};
    use time::OffsetDateTime;
    use tokio::sync::Mutex;

    use crate::{
        commands,
        error::CommandError,
        model::{PlatformKind, RefreshMode},
        state::RuntimeState,
        windowing,
    };

    /// Managed lifecycle for the optional GNOME session-bus service.
    #[derive(Debug, Default)]
    pub struct GnomeIntegration {
        handle: Mutex<Option<ServiceHandle>>,
    }

    impl GnomeIntegration {
        async fn enable(&self, app: &AppHandle) -> Result<(), CommandError> {
            let mut guard = self.handle.lock().await;
            if guard.is_some() {
                return Ok(());
            }

            let initial = current_snapshot(app).await?;
            let (handle, mut requests) = start_service(initial)
                .await
                .map_err(|error| CommandError::new("gnome_ipc_unavailable", error.to_string()))?;
            *guard = Some(handle);
            drop(guard);

            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                while let Some(request) = requests.recv().await {
                    match request {
                        ServiceRequest::Refresh { response } => {
                            let result = refresh_snapshot(&app_handle).await;
                            if let Err(message) = &result {
                                tracing::warn!(message, "GNOME refresh request failed");
                            }
                            let _delivery = response.send(result);
                        }
                        ServiceRequest::OpenSettings { response } => {
                            let result =
                                commands::open_settings(app_handle.clone()).map_err(|error| {
                                    tracing::warn!(
                                        code = error.code,
                                        message = %error.message,
                                        "GNOME settings request failed"
                                    );
                                    error.message
                                });
                            let _delivery = response.send(result);
                        }
                        ServiceRequest::ShowMainWindow { response } => {
                            let result = windowing::show_main(&app_handle).map_err(|error| {
                                tracing::warn!(
                                    code = error.code,
                                    message = %error.message,
                                    "GNOME main-window request failed"
                                );
                                error.message
                            });
                            let _delivery = response.send(result);
                        }
                    }
                }
            });
            Ok(())
        }

        async fn disable(&self) {
            self.handle.lock().await.take();
        }

        async fn service_handle(&self) -> Option<ServiceHandle> {
            self.handle.lock().await.clone()
        }
    }

    /// Enables or disables the session D-Bus service to match persisted settings.
    pub async fn reconcile(app: &AppHandle, enabled: bool) -> Result<(), CommandError> {
        let integration = app.state::<GnomeIntegration>();
        if enabled {
            integration.enable(app).await
        } else {
            integration.disable().await;
            Ok(())
        }
    }

    /// Publishes the latest complete snapshot when integration is active.
    pub async fn publish(app: &AppHandle) -> Result<(), CommandError> {
        let integration = app.state::<GnomeIntegration>();
        let Some(handle) = integration.service_handle().await else {
            return Ok(());
        };
        handle
            .publish(current_snapshot(app).await?)
            .await
            .map_err(|error| CommandError::new("gnome_ipc_delivery", error.to_string()))
    }

    async fn refresh_snapshot(app: &AppHandle) -> Result<String, String> {
        commands::refresh_and_emit(app, RefreshMode::BoundedDiscovery)
            .await
            .map_err(command_message)?;
        current_snapshot(app)
            .await
            .map_err(command_message)?
            .to_json()
            .map_err(|error| error.to_string())
    }

    fn command_message(error: CommandError) -> String {
        error.message
    }

    async fn current_snapshot(app: &AppHandle) -> Result<ServiceSnapshot, CommandError> {
        let state = app.state::<RuntimeState>();
        let backend = state.backend_status().await;
        let devices = state.devices().await;
        let settings = state.load_settings().map_err(CommandError::storage)?;
        let generated_at = OffsetDateTime::now_utc();

        Ok(ServiceSnapshot::new(
            generated_at,
            Some(BackendSnapshot {
                platform: platform_name(backend.platform).to_owned(),
                available: backend.available,
                adapter_name: backend.adapter_name,
                powered: backend.powered,
                discovering: backend.discovering,
                detail: backend.detail,
            }),
            devices,
            settings.preferred_device_id,
        ))
    }

    const fn platform_name(platform: PlatformKind) -> &'static str {
        match platform {
            PlatformKind::Linux => "linux",
            PlatformKind::Windows => "windows",
            PlatformKind::Unsupported => "unsupported",
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux::{GnomeIntegration, publish, reconcile};

#[cfg(not(target_os = "linux"))]
/// No-op managed state outside Linux.
#[derive(Debug, Default)]
pub struct GnomeIntegration;

#[cfg(not(target_os = "linux"))]
/// GNOME integration is not started outside Linux.
pub async fn reconcile(_app: &AppHandle, _enabled: bool) -> Result<(), CommandError> {
    Ok(())
}

#[cfg(not(target_os = "linux"))]
/// GNOME publication is a no-op outside Linux.
pub async fn publish(_app: &AppHandle) -> Result<(), CommandError> {
    Ok(())
}
