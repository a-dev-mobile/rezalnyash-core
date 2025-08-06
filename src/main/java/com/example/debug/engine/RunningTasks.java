package com.example.debug.engine;

import com.example.debug.engine.model.Status;
import com.example.debug.engine.model.Task;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;

/* loaded from: classes.dex */
public class RunningTasks {
    private static final RunningTasks instance = new RunningTasks();
    private List<Task> tasks = new ArrayList();
    private List<CutListThread> runningThreads = new ArrayList();
    private long nbrTotalTasks = 0;
    private long nbrArchivedFinishedTasks = 0;
    private long nbrArchivedStoppedTasks = 0;
    private long nbrArchivedTerminatedTasks = 0;
    private long nbrArchivedErrorTasks = 0;

    private RunningTasks() {
    }

    public static RunningTasks getInstance() {
        return instance;
    }

    public long getNbrTotalTasks() {
        return this.nbrTotalTasks;
    }

    public List<Task> getTasks() {
        return Collections.unmodifiableList(this.tasks);
    }

    public boolean removeAllTasks(List<Task> list) {
        for (Task task : list) {
            if (Status.FINISHED.equals(task.getStatus())) {
                this.nbrArchivedFinishedTasks++;
            } else if (Status.STOPPED.equals(task.getStatus())) {
                this.nbrArchivedStoppedTasks++;
            } else if (Status.TERMINATED.equals(task.getStatus())) {
                this.nbrArchivedTerminatedTasks++;
            } else if (Status.ERROR.equals(task.getStatus())) {
                this.nbrArchivedErrorTasks++;
            }
        }
        return this.tasks.removeAll(list);
    }

    public synchronized boolean addTask(Task task) {
        this.nbrTotalTasks++;
        return this.tasks.add(task);
    }

    public Task getTask(String str) {
        for (Task task : this.tasks) {
            if (task.getId().equals(str)) {
                return task;
            }
        }
        return null;
    }

    public List<CutListThread> getRunningThreads() {
        return this.runningThreads;
    }

    public long geNbrIdleTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.IDLE.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public long geNbrRunningTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.RUNNING.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public long geNbrFinishedTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.FINISHED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i + this.nbrArchivedFinishedTasks;
    }

    public long geNbrStoppedTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.STOPPED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i + this.nbrArchivedStoppedTasks;
    }

    public long geNbrTerminatedTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.TERMINATED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i + this.nbrArchivedTerminatedTasks;
    }

    public long geNbrErrorTasks() {
        Iterator<Task> it = this.tasks.iterator();
        int i = 0;
        while (it.hasNext()) {
            if (Status.ERROR.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i + this.nbrArchivedErrorTasks;
    }
}
