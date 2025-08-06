package com.example.debug.engine.model;

import com.example.debug.engine.TaskReport;
import java.util.List;

/* loaded from: classes.dex */
public class Stats {
    private long nbrErrorTasks;
    private long nbrFinishedTasks;
    private long nbrFinishedThreads;
    private long nbrIdleTasks;
    private int nbrQueuedThreads;
    private long nbrRunningTasks;
    private int nbrRunningThreads;
    private long nbrStoppedTasks;
    private long nbrTerminatedTasks;
    private List<TaskReport> taskReports;

    public long getNbrIdleTasks() {
        return this.nbrIdleTasks;
    }

    public void setNbrIdleTasks(long j) {
        this.nbrIdleTasks = j;
    }

    public long getNbrRunningTasks() {
        return this.nbrRunningTasks;
    }

    public void setNbrRunningTasks(long j) {
        this.nbrRunningTasks = j;
    }

    public long getNbrFinishedTasks() {
        return this.nbrFinishedTasks;
    }

    public void setNbrFinishedTasks(long j) {
        this.nbrFinishedTasks = j;
    }

    public long getNbrStoppedTasks() {
        return this.nbrStoppedTasks;
    }

    public void setNbrStoppedTasks(long j) {
        this.nbrStoppedTasks = j;
    }

    public long getNbrTerminatedTasks() {
        return this.nbrTerminatedTasks;
    }

    public void setNbrTerminatedTasks(long j) {
        this.nbrTerminatedTasks = j;
    }

    public long getNbrErrorTasks() {
        return this.nbrErrorTasks;
    }

    public void setNbrErrorTasks(long j) {
        this.nbrErrorTasks = j;
    }

    public int getNbrRunningThreads() {
        return this.nbrRunningThreads;
    }

    public void setNbrRunningThreads(int i) {
        this.nbrRunningThreads = i;
    }

    public int getNbrQueuedThreads() {
        return this.nbrQueuedThreads;
    }

    public void setNbrQueuedThreads(int i) {
        this.nbrQueuedThreads = i;
    }

    public long getNbrFinishedThreads() {
        return this.nbrFinishedThreads;
    }

    public void setNbrFinishedThreads(long j) {
        this.nbrFinishedThreads = j;
    }

    public List<TaskReport> getTaskReports() {
        return this.taskReports;
    }

    public void setTaskReports(List<TaskReport> list) {
        this.taskReports = list;
    }
}
