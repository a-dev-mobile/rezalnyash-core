package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class PerformanceThresholds {
    private int maxSimultaneousTasks = 1;
    private int maxSimultaneousThreads;
    private long threadCheckInterval;

    public PerformanceThresholds() {
    }

    public PerformanceThresholds(int i, long j) {
        this.maxSimultaneousThreads = i;
        this.threadCheckInterval = j;
    }

    public long getThreadCheckInterval() {
        return this.threadCheckInterval;
    }

    public void setThreadCheckInterval(long j) {
        this.threadCheckInterval = j;
    }

    public int getMaxSimultaneousThreads() {
        return this.maxSimultaneousThreads;
    }

    public void setMaxSimultaneousThreads(int i) {
        this.maxSimultaneousThreads = i;
    }

    public int getMaxSimultaneousTasks() {
        return this.maxSimultaneousTasks;
    }

    public void setMaxSimultaneousTasks(int i) {
        this.maxSimultaneousTasks = i;
    }
}
