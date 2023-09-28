use anyhow::Context;
use tracing_log::LogTracer;

use tracing_subscriber::{
    self,
    fmt::{format::Writer, time::FormatTime},
    EnvFilter,
};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn setup() -> anyhow::Result<()> {
    LogTracer::init().with_context(|| "setting default LogTracer failed")?;
    let format = tracing_subscriber::fmt::format().with_level(true).with_target(true).with_timer(LocalTimer);
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout) // 写入标准输出
        .with_ansi(true) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .finish();
    tracing::subscriber::set_global_default(subscriber).with_context(|| "setting default subscriber failed")?;
    Ok(())
}
