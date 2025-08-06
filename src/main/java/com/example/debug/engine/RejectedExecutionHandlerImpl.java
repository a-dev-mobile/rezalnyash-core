package com.example.debug.engine;

import java.util.concurrent.RejectedExecutionHandler;
import java.util.concurrent.ThreadPoolExecutor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
public class RejectedExecutionHandlerImpl implements RejectedExecutionHandler {
    private static final Logger logger = LoggerFactory.getLogger((Class<?>) RejectedExecutionHandlerImpl.class);
    private RunningTasks runningTasks;

    public RejectedExecutionHandlerImpl(RunningTasks runningTasks) {
        this.runningTasks = runningTasks;
    }

    @Override // java.util.concurrent.RejectedExecutionHandler
    public void rejectedExecution(Runnable runnable, ThreadPoolExecutor threadPoolExecutor) {
        this.runningTasks.getRunningThreads().remove(runnable);
        logger.warn(runnable.toString() + " was rejected");
    }
}
