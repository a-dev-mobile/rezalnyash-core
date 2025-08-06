package com.example.debug.api.dto;

public class StatsDto {
    private int runningThreads;
    private int queuedThreads;
    private int completedThreads;
    private int runningTasks;
    private int completedTasks;
    
    public StatsDto() {}
    
    public StatsDto(int runningThreads, int queuedThreads, int completedThreads, 
                   int runningTasks, int completedTasks) {
        this.runningThreads = runningThreads;
        this.queuedThreads = queuedThreads;
        this.completedThreads = completedThreads;
        this.runningTasks = runningTasks;
        this.completedTasks = completedTasks;
    }
    
    public int getRunningThreads() { return runningThreads; }
    public void setRunningThreads(int runningThreads) { this.runningThreads = runningThreads; }
    
    public int getQueuedThreads() { return queuedThreads; }
    public void setQueuedThreads(int queuedThreads) { this.queuedThreads = queuedThreads; }
    
    public int getCompletedThreads() { return completedThreads; }
    public void setCompletedThreads(int completedThreads) { this.completedThreads = completedThreads; }
    
    public int getRunningTasks() { return runningTasks; }
    public void setRunningTasks(int runningTasks) { this.runningTasks = runningTasks; }
    
    public int getCompletedTasks() { return completedTasks; }
    public void setCompletedTasks(int completedTasks) { this.completedTasks = completedTasks; }
}