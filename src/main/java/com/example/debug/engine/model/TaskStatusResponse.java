package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class TaskStatusResponse {
    private int initPercentage;
    private int percentageDone;
    private CalculationResponse solution;
    private String status;

    public String getStatus() {
        return this.status;
    }

    public void setStatus(String str) {
        this.status = str;
    }

    public int getPercentageDone() {
        return this.percentageDone;
    }

    public void setPercentageDone(int i) {
        this.percentageDone = i;
    }

    public int getInitPercentage() {
        return this.initPercentage;
    }

    public void setInitPercentage(int i) {
        this.initPercentage = i;
    }

    public CalculationResponse getSolution() {
        return this.solution;
    }

    public void setSolution(CalculationResponse calculationResponse) {
        this.solution = calculationResponse;
    }
}
