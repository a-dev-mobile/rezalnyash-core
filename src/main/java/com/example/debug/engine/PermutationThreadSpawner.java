package com.example.debug.engine;

import java.lang.Thread;
import java.util.ArrayList;
import java.util.List;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
public class PermutationThreadSpawner {
    private static final Logger logger = LoggerFactory.getLogger((Class<?>) PermutationThreadSpawner.class);
    private ProgressTracker progressTracker;
    private int maxAliveSpawnerThreads = 5;
    private long intervalBetweenMaxAliveCheck = 1000;
    private List<Thread> threads = new ArrayList();

    public void setProgressTracker(ProgressTracker progressTracker) {
        this.progressTracker = progressTracker;
    }

    public void spawn(Thread thread) throws InterruptedException {
        this.threads.add(thread);
        while (getNbrUnfinishedThreads() >= this.maxAliveSpawnerThreads) {
            try {
                this.progressTracker.refreshTaskStatusInfo();
                Thread.sleep(this.intervalBetweenMaxAliveCheck);
            } catch (InterruptedException e) {
                logger.error("Permutation thread spawner interrupted", (Throwable) e);
            }
        }
        thread.start();
    }

    public int getNbrUnfinishedThreads() {
        int i = 0;
        for (Thread thread : this.threads) {
            if (thread.isAlive() || Thread.State.NEW.equals(thread.getState())) {
                i++;
            }
        }
        return i;
    }

    public int getNbrTotalThreads() {
        return this.threads.size();
    }

    public int getMaxAliveSpawnerThreads() {
        return this.maxAliveSpawnerThreads;
    }

    public void setMaxAliveSpawnerThreads(int i) {
        this.maxAliveSpawnerThreads = i;
    }

    public long getIntervalBetweenMaxAliveCheck() {
        return this.intervalBetweenMaxAliveCheck;
    }

    public void setIntervalBetweenMaxAliveCheck(long j) {
        this.intervalBetweenMaxAliveCheck = j;
    }
}
