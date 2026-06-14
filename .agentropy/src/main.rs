// # generated - do not hand-edit
#[tokio::main(worker_threads = 2)]
async fn main() {
    agentropy_cli::run(host_api::plugins![
        #[cfg(feature = "stock-chat-codex")]
        chat_codex::ChatCodexExtension,
        #[cfg(feature = "stock-chat-opencode")]
        chat_opencode::ChatOpenCodeExtension,
        #[cfg(feature = "stock-chat-pi")]
        chat_pi::ChatPiExtension,
        #[cfg(feature = "stock-dashboard")]
        dashboard::DashboardExtension::default(),
        #[cfg(feature = "stock-frontend-log")]
        frontend_log::FrontendLogExtension,
        #[cfg(feature = "stock-orchestrator")]
        orchestrator::OrchestratorExtension::default(),
        #[cfg(feature = "stock-runner-cli")]
        runner_cli::RunnerCliExtension,
        #[cfg(feature = "stock-runner-codex")]
        runner_codex::RunnerCodexExtension,
        #[cfg(feature = "stock-runner-fake")]
        runner_fake::RunnerFakeExtension,
        #[cfg(feature = "stock-runner-opencode")]
        runner_opencode::RunnerOpenCodeExtension,
        #[cfg(feature = "stock-runner-pi")]
        runner_pi::RunnerPiExtension,
        #[cfg(feature = "stock-tracker-linear")]
        tracker_linear::TrackerLinearExtension,
        #[cfg(feature = "stock-tui")]
        tui::TuiExtension,
    ])
    .await
}
