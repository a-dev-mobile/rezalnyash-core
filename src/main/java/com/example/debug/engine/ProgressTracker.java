package com.example.debug.engine;

import com.example.debug.engine.model.Task;

/* loaded from: classes.dex */
public class ProgressTracker {
    private static int MAX_PERMUTATIONS_WITH_SOLUTION = 150;
    private String material;
    private PermutationThreadSpawner permutationThreadSpawner;
    private Task task;
    private int totalPermutations;

    public ProgressTracker(PermutationThreadSpawner permutationThreadSpawner, int i, Task task, String str) {
        this.permutationThreadSpawner = permutationThreadSpawner;
        this.totalPermutations = i;
        this.task = task;
        this.material = str;
    }

    public void refreshTaskStatusInfo() {
        int iMin;
        if (this.task.hasSolutionAllFit()) {
            iMin = Math.min(Math.max((int) ((this.task.getElapsedTime() / 60000.0f) * 100.0f), (int) (((this.permutationThreadSpawner.getNbrTotalThreads() - 1) / Math.min(MAX_PERMUTATIONS_WITH_SOLUTION, this.totalPermutations)) * 100.0f)), 100);
        } else {
            iMin = Math.min(Math.max((int) ((this.task.getElapsedTime() / 600000.0f) * 100.0f), (int) (((this.permutationThreadSpawner.getNbrTotalThreads() - 1) / this.totalPermutations) * 100.0f)), 100);
        }
        this.task.setMaterialPercentageDone(this.material, Integer.valueOf(iMin));
    }
}
