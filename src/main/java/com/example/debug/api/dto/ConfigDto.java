package com.example.debug.api.dto;

public class ConfigDto {
    private Integer maxThreads;
    private Integer checkInterval;
    private Double optimizationFactor;
    
    public Integer getMaxThreads() { return maxThreads; }
    public void setMaxThreads(Integer maxThreads) { this.maxThreads = maxThreads; }
    
    public Integer getCheckInterval() { return checkInterval; }
    public void setCheckInterval(Integer checkInterval) { this.checkInterval = checkInterval; }
    
    public Double getOptimizationFactor() { return optimizationFactor; }
    public void setOptimizationFactor(Double optimizationFactor) { this.optimizationFactor = optimizationFactor; }
}