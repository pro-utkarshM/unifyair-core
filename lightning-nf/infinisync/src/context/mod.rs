struct InfinySyncContext {
    pub config: InfinySyncConfig,
    pub state: InfinySyncState,
    pub logger: Logger,
    pub client: InfinySyncClient,
    pub syncer: InfinySyncSyncer,
    pub watcher: InfinySyncWatcher,
    pub notifier: InfinySyncNotifier,
    pub scheduler: InfinySyncScheduler,
    pub metrics: InfinySyncMetrics,
    pub shutdown: Shutdown,
}

pub struct PFCP {
    

}