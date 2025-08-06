package com.example.debug.engine;

import com.example.debug.engine.model.Status;
import com.example.debug.engine.model.Task;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadPoolExecutor;
import org.apache.commons.lang3.time.DateUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
public class WatchDog implements Runnable {
    private static final int FINISHED_TASK_TTL = 60000;
    public static final int LONG_RUNNING_TASK_TTL = 600000;
    public static final int LONG_RUNNING_TASK_WITH_SOLUTION_TTL = 60000;
    private static final int ORPHAN_TASK_TTL = 60000;
    private static final int RUNNING_INTERVAL = 5000;
    private static final int TASK_ERROR_THREAD_THRESHOLD = 100;
    private static final Logger logger = LoggerFactory.getLogger((Class<?>) WatchDog.class);
    private CutListLogger cutListLogger;
    private CutListOptimizerService cutListOptimizerService;
    private RunningTasks runningTasks;
    private ThreadPoolExecutor taskExecutor;
    private List<TaskReport> taskReports = new ArrayList();

    public RunningTasks getRunningTasks() {
        return this.runningTasks;
    }

    public void setRunningTasks(RunningTasks runningTasks) {
        this.runningTasks = runningTasks;
    }

    public ThreadPoolExecutor getTaskExecutor() {
        return this.taskExecutor;
    }

    public void setTaskExecutor(ThreadPoolExecutor threadPoolExecutor) {
        this.taskExecutor = threadPoolExecutor;
    }

    public CutListOptimizerService getCutListOptimizerService() {
        return this.cutListOptimizerService;
    }

    public void setCutListOptimizerService(CutListOptimizerService cutListOptimizerService) {
        this.cutListOptimizerService = cutListOptimizerService;
    }

    public CutListLogger getCutListLogger() {
        return this.cutListLogger;
    }

    public void setCutListLogger(CutListLogger cutListLogger) {
        this.cutListLogger = cutListLogger;
    }

    public List<TaskReport> getTaskReports() {
        return this.taskReports;
    }

    @Override // java.lang.Runnable
    public void run() {
        while (true) {
            try {
                try {
                    logger.debug("Tasks: Active[{}] Total[{}] - Threads: Active[{}/{}] Queued[{}] Completed[{}]", Integer.valueOf(this.runningTasks.getTasks().size()), Long.valueOf(this.runningTasks.getNbrTotalTasks()), Integer.valueOf(this.taskExecutor.getActiveCount()), Integer.valueOf(this.taskExecutor.getPoolSize()), Integer.valueOf(this.taskExecutor.getQueue().size()), Long.valueOf(this.taskExecutor.getCompletedTaskCount()));
                    synchronized (this.runningTasks) {
                        this.taskReports.clear();
                        for (Task task : this.runningTasks.getTasks()) {
                            try {
                                Logger logger2 = logger;
                                logger2.debug("Watching task " + task.getId());
                                TaskReport taskReport = new TaskReport();
                                taskReport.setTaskId(task.getId());
                                taskReport.setClientId(task.getClientInfo().getId());
                                taskReport.setStatus(task.getStatus().toString());
                                taskReport.setNbrRunningThreads(task.getNbrRunningThreads());
                                taskReport.setNbrQueuedThreads(task.getNbrQueuedThreads());
                                taskReport.setNbrCompletedThreads(task.getNbrTotalThreads());
                                taskReport.setNbrPanels(task.getCalculationRequest().getPanels().size());
                                taskReport.setPercentageDone(task.getPercentageDone());
                                taskReport.setElapsedTime(Utils.longElapsedTime2HumanReadable(task.getElapsedTime()));
                                this.taskReports.add(taskReport);
                                logger2.info("Task[{}] {} - Threads: R[{}/{}] Q[{}] T[{}] - Panels[{}] Done[{}%] ElapsedTime[{}]", task.getId(), task.getStatus(), Integer.valueOf(task.getNbrRunningThreads()), Integer.valueOf(CutListOptimizerServiceImpl.MAX_ACTIVE_THREADS_PER_TASK), Integer.valueOf(task.getNbrQueuedThreads()), Integer.valueOf(task.getNbrTotalThreads()), Integer.valueOf(task.getCalculationRequest().getPanels().size()), Integer.valueOf(task.getPercentageDone()), Utils.longElapsedTime2HumanReadable(task.getElapsedTime()));
                                if (task.isRunning() && task.getNbrErrorThreads() > 100 && task.getNbrErrorThreads() == task.getNbrTotalThreads()) {
                                    this.cutListLogger.error("Error thread threshold reached");
                                    task.terminateError();
                                }
                                this.cutListLogger.logExecution(task);
                            } catch (Exception e) {
                                this.cutListLogger.error("Error while logging task execution", e);
                            }
                        }
                    }
                    try {
                        logger.trace("Sleeping 5000ms");
                        Thread.sleep(5000L);
                    } catch (InterruptedException e2) {
                        this.cutListLogger.error("Interrupted", e2);
                        cleanFinishedThreads();
                    }
                } catch (Throwable th) {
                    try {
                        logger.trace("Sleeping 5000ms");
                        Thread.sleep(5000L);
                    } catch (InterruptedException e3) {
                        this.cutListLogger.error("Interrupted", e3);
                    }
                    cleanFinishedThreads();
                    throw th;
                }
            } catch (Exception e4) {
                this.cutListLogger.error("Error while logging task executor status", e4);
                try {
                    logger.trace("Sleeping 5000ms");
                    Thread.sleep(5000L);
                } catch (InterruptedException e5) {
                    this.cutListLogger.error("Interrupted", e5);
                    cleanFinishedThreads();
                }
            }
            cleanFinishedThreads();
        }
    }

    private void cleanFinishedThreads() {
        logger.debug("Cleaning finished tasks");
        synchronized (this.runningTasks) {
            ArrayList arrayList = new ArrayList();
            for (Task task : this.runningTasks.getTasks()) {
                if ((task.getStatus().equals(Status.FINISHED) || task.getStatus().equals(Status.STOPPED) || task.getStatus().equals(Status.TERMINATED) || task.getStatus().equals(Status.ERROR)) && System.currentTimeMillis() - task.getEndTime() > DateUtils.MILLIS_PER_MINUTE) {
                    arrayList.add(task);
                }
                if (task.getStatus().equals(Status.RUNNING) && task.getElapsedTime() > DateUtils.MILLIS_PER_MINUTE && task.hasSolutionAllFit()) {
                    this.cutListLogger.warn(task.getClientInfo().getId(), task.getId(), "Task with solution has been running for more than 1m and will be terminated");
                    task.appendLineToLog("Task with solution has been running for more than 1m and will be terminated");
                    if (this.cutListOptimizerService.terminateTask(task.getId()) != 0) {
                        this.cutListLogger.error(task.getClientInfo().getId(), task.getId(), "Unable to terminate task");
                    }
                }
                if (task.getStatus().equals(Status.RUNNING) && task.getElapsedTime() > 600000) {
                    this.cutListLogger.warn(task.getClientInfo().getId(), task.getId(), "Task has been running for more than 10m and will be terminated");
                    task.appendLineToLog("Task has been running for more than 10m and will be terminated");
                    if (this.cutListOptimizerService.terminateTask(task.getId()) != 0) {
                        this.cutListLogger.error(task.getClientInfo().getId(), task.getId(), "Unable to terminate task");
                    }
                }
                if (task.getStatus().equals(Status.RUNNING) && System.currentTimeMillis() - task.getLastQueried() > DateUtils.MILLIS_PER_MINUTE) {
                    this.cutListLogger.warn(task.getClientInfo().getId(), task.getId(), "Task status was not queried for more than 1m and will be terminated");
                    task.appendLineToLog("Task status was not queried for more than 1m and will be terminated");
                    if (this.cutListOptimizerService.terminateTask(task.getId()) != 0) {
                        this.cutListLogger.error(task.getClientInfo().getId(), task.getId(), "Unable to terminate task");
                    }
                }
            }
            this.runningTasks.removeAllTasks(arrayList);
            logger.trace("Cleared " + arrayList.size() + " tasks");
            ArrayList arrayList2 = new ArrayList();
            for (CutListThread cutListThread : this.runningTasks.getRunningThreads()) {
                if (this.runningTasks.getTask(cutListThread.getTask().getId()) == null) {
                    arrayList2.add(cutListThread);
                }
            }
            this.runningTasks.getRunningThreads().removeAll(arrayList2);
            logger.trace("Cleared " + arrayList2.size() + " threads");
        }
    }
}
