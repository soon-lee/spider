use std::collections::HashMap;

use chrono::{DateTime, Utc};
use tokio::time::{Instant, sleep};

async fn delay_until(target: Instant) {
    let delay = target.duration_since(Instant::now());
    if let Ok(d) = delay {
        sleep(d).await;
    } else {
        // 如果目标时间在过去，那么立即返回
        return;
    }
}
/// 定义一个异步函数类型作为定时任务
type Task = Box<dyn Fn() + Send + Sync + 'static>;

/// Scheduler 结构体用于管理定时任务
struct Scheduler {
    tasks: HashMap<DateTime<Utc>, Task>,
}

impl Scheduler {
    /// 创建一个新的 Scheduler 实例
    fn new() -> Self {
        Scheduler {
            tasks: HashMap::new(),
        }
    }

    /// 注册一个任务，参数为任务函数和触发时间
    fn register_task(&mut self, trigger_time: DateTime<Utc>, task: Task) {
        self.tasks.insert(trigger_time, task);
    }

    /// 启动调度器，开始执行任务
    async fn start(&mut self) {
        let mut current_time = Utc::now();
        while let Some((trigger_time, task)) = self.tasks.range(current_time..).next().map(|(k, v)| (*k, v.clone())) {
            let delay = delay_until(trigger_time);
            tokio::spawn(async move {
                delay.await;
                (task)();
            });

            // 更新当前时间以检查下一个任务
            current_time = Utc::now();
        }
    }
}
