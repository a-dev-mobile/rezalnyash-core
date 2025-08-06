package com.example.debug.engine;

/* loaded from: classes.dex */
public class TaskReport {
    private String clientId;
    private String elapsedTime;
    private int nbrCompletedThreads;
    private int nbrPanels;
    private int nbrQueuedThreads;
    private int nbrRunningThreads;
    private int percentageDone;
    private String status;
    private String taskId;

    public String getTaskId() {
        return this.taskId;
    }

    public void setTaskId(String str) {
        this.taskId = str;
    }

    public String getClientId() {
        return this.clientId;
    }

    public void setClientId(String str) {
        this.clientId = str;
    }

    public String getStatus() {
        return this.status;
    }

    public void setStatus(String str) {
        this.status = str;
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

    public int getNbrCompletedThreads() {
        return this.nbrCompletedThreads;
    }

    public void setNbrCompletedThreads(int i) {
        this.nbrCompletedThreads = i;
    }

    public int getNbrPanels() {
        return this.nbrPanels;
    }

    public void setNbrPanels(int i) {
        this.nbrPanels = i;
    }

    public int getPercentageDone() {
        return this.percentageDone;
    }

    public void setPercentageDone(int i) {
        this.percentageDone = i;
    }

    public String getElapsedTime() {
        return this.elapsedTime;
    }

    public void setElapsedTime(String str) {
        this.elapsedTime = str;
    }
}
