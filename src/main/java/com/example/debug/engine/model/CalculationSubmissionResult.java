package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class CalculationSubmissionResult {
    private String statusCode;
    private String taskId;

    public CalculationSubmissionResult(String str, String str2) {
        this.statusCode = str;
        this.taskId = str2;
    }

    public CalculationSubmissionResult(String str) {
        this.statusCode = str;
    }

    public String getStatusCode() {
        return this.statusCode;
    }

    public void setStatusCode(String str) {
        this.statusCode = str;
    }

    public String getTaskId() {
        return this.taskId;
    }

    public void setTaskId(String str) {
        this.taskId = str;
    }
}