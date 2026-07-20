// # generated - do not hand-edit
#[tokio::main(worker_threads = 2)]
async fn main() {
    let mut plugins: Vec<std::sync::Arc<dyn host_api::Extension>> = Vec::new();
    #[cfg(feature = "stock-tool-registry-host")]
    plugins.push(std::sync::Arc::new(tool_registry_host::ToolRegistryHostExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-frontend-log")]
    plugins.push(std::sync::Arc::new(frontend_log::FrontendLogExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-system-context")]
    plugins.push(std::sync::Arc::new(system_context::SystemContextExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-tracker-linear")]
    plugins.push(std::sync::Arc::new(tracker_linear::TrackerLinearExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-orchestrator")]
    plugins.push(std::sync::Arc::new(orchestrator::OrchestratorExtension::default()) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-dashboard")]
    plugins.push(std::sync::Arc::new(dashboard::DashboardExtension::default()) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-runner-pi")]
    plugins.push(std::sync::Arc::new(runner_pi::RunnerPiExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-runner-codex")]
    plugins.push(std::sync::Arc::new(runner_codex::RunnerCodexExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-runner-opencode")]
    plugins.push(std::sync::Arc::new(runner_opencode::RunnerOpenCodeExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-runner-cli")]
    plugins.push(std::sync::Arc::new(runner_cli::RunnerCliExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-runner-fake")]
    plugins.push(std::sync::Arc::new(runner_fake::RunnerFakeExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-chat-opencode")]
    plugins.push(std::sync::Arc::new(chat_opencode::ChatOpenCodeExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-chat-pi")]
    plugins.push(std::sync::Arc::new(chat_pi::ChatPiExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-chat-codex")]
    plugins.push(std::sync::Arc::new(chat_codex::ChatCodexExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-chat-web")]
    plugins.push(std::sync::Arc::new(chat_web::ChatWebExtension::default()) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-tui")]
    plugins.push(std::sync::Arc::new(tui::TuiExtension) as std::sync::Arc<dyn host_api::Extension>);
    #[cfg(feature = "stock-scheduler")]
    plugins.push(std::sync::Arc::new(scheduler::SchedulerExtension::default()) as std::sync::Arc<dyn host_api::Extension>);
    plugins.shrink_to_fit();
    dar_cli_core::run(plugins).await
}
