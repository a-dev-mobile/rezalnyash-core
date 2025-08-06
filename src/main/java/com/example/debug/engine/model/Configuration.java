package com.example.debug.engine.model;

import com.fasterxml.jackson.annotation.JsonIgnore;

/* loaded from: classes.dex */
public class Configuration {
    private boolean considerOrientation;
    private int cutOrientationPreference;
    private String cutThickness;
    private String minTrimDimension;
    private double optimizationFactor;
    private int optimizationPriority;

    @JsonIgnore
    private PerformanceThresholds performanceThresholds;
    private Integer units;
    private boolean useSingleStockUnit;

    public String getCutThickness() {
        return this.cutThickness;
    }

    public void setCutThickness(String str) {
        this.cutThickness = str;
    }

    public boolean isUseSingleStockUnit() {
        return this.useSingleStockUnit;
    }

    public void setUseSingleStockUnit(boolean z) {
        this.useSingleStockUnit = z;
    }

    public double getOptimizationFactor() {
        return this.optimizationFactor;
    }

    public void setOptimizationFactor(double d) {
        this.optimizationFactor = d;
    }

    public int getOptimizationPriority() {
        return this.optimizationPriority;
    }

    public void setOptimizationPriority(int i) {
        this.optimizationPriority = i;
    }

    public int getCutOrientationPreference() {
        return this.cutOrientationPreference;
    }

    public void setCutOrientationPreference(int i) {
        this.cutOrientationPreference = i;
    }

    public Integer getUnits() {
        return this.units;
    }

    public void setUnits(Integer num) {
        this.units = num;
    }

    public boolean isConsiderOrientation() {
        return this.considerOrientation;
    }

    public void setConsiderOrientation(boolean z) {
        this.considerOrientation = z;
    }

    public String getMinTrimDimension() {
        return this.minTrimDimension;
    }

    public void setMinTrimDimension(String str) {
        this.minTrimDimension = str;
    }

    public PerformanceThresholds getPerformanceThresholds() {
        return this.performanceThresholds;
    }

    public void setPerformanceThresholds(PerformanceThresholds performanceThresholds) {
        this.performanceThresholds = performanceThresholds;
    }

    public String toString() {
        return "Configuration{cutThickness=" + this.cutThickness + ", useSingleStockUnit=" + this.useSingleStockUnit + ", optimizationFactor=" + this.optimizationFactor + ", optimizationPriority=" + this.optimizationPriority + ", units=" + this.units + ", considerOrientation=" + this.considerOrientation + '}';
    }
}
