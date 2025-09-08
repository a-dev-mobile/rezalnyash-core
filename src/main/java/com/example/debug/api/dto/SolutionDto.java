package com.example.debug.api.dto;

import java.util.List;

public class SolutionDto {
    private double usedArea;
    private double wastedArea;
    private double usageRate;
    private int cutCount;
    private double cutLength;
    private long executionTimeMillis;
    private List<MosaicDto> mosaics;
    
    public SolutionDto() {}
    
    public SolutionDto(double usedArea, double wastedArea, double usageRate, 
                      int cutCount, double cutLength, long executionTimeMillis) {
        this.usedArea = usedArea;
        this.wastedArea = wastedArea;
        this.usageRate = usageRate;
        this.cutCount = cutCount;
        this.cutLength = cutLength;
        this.executionTimeMillis = executionTimeMillis;
    }
    
    public SolutionDto(double usedArea, double wastedArea, double usageRate, 
                      int cutCount, double cutLength, long executionTimeMillis, List<MosaicDto> mosaics) {
        this.usedArea = usedArea;
        this.wastedArea = wastedArea;
        this.usageRate = usageRate;
        this.cutCount = cutCount;
        this.cutLength = cutLength;
        this.executionTimeMillis = executionTimeMillis;
        this.mosaics = mosaics;
    }
    
    public double getUsedArea() { return usedArea; }
    public void setUsedArea(double usedArea) { this.usedArea = usedArea; }
    
    public double getWastedArea() { return wastedArea; }
    public void setWastedArea(double wastedArea) { this.wastedArea = wastedArea; }
    
    public double getUsageRate() { return usageRate; }
    public void setUsageRate(double usageRate) { this.usageRate = usageRate; }
    
    public int getCutCount() { return cutCount; }
    public void setCutCount(int cutCount) { this.cutCount = cutCount; }
    
    public double getCutLength() { return cutLength; }
    public void setCutLength(double cutLength) { this.cutLength = cutLength; }
    
    public long getExecutionTimeMillis() { return executionTimeMillis; }
    public void setExecutionTimeMillis(long executionTimeMillis) { this.executionTimeMillis = executionTimeMillis; }
    
    public List<MosaicDto> getMosaics() { return mosaics; }
    public void setMosaics(List<MosaicDto> mosaics) { this.mosaics = mosaics; }
}