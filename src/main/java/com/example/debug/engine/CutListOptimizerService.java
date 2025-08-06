package com.example.debug.engine;

import com.example.debug.engine.model.CalculationRequest;
import com.example.debug.engine.model.CalculationSubmissionResult;
import com.example.debug.engine.model.Stats;
import com.example.debug.engine.model.Status;
import com.example.debug.engine.model.TaskStatusResponse;
import java.util.List;

/* loaded from: classes.dex */
public interface CutListOptimizerService {
    Stats getStats();

    TaskStatusResponse getTaskStatus(String str);

    List<String> getTasks(String str, Status status);

    void init(int i);

    void setAllowMultipleTasksPerClient(boolean z);

    void setCutListLogger(CutListLogger cutListLogger);

    TaskStatusResponse stopTask(String str);

    CalculationSubmissionResult submitTask(CalculationRequest calculationRequest);

    int terminateTask(String str);

    public enum StatusCode {
        OK(0),
        INVALID_TILES(1),
        INVALID_STOCK_TILES(2),
        TASK_ALREADY_RUNNING(3),
        SERVER_UNAVAILABLE(4),
        TOO_MANY_PANELS(5),
        TOO_MANY_STOCK_PANELS(6);

        private final int value;

        StatusCode(int i) {
            this.value = i;
        }

        public int getValue() {
            return this.value;
        }

        public String getStringValue() {
            return Integer.toString(this.value);
        }
    }
}
