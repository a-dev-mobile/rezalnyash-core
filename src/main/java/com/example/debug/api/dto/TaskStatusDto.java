package com.example.debug.api.dto;

public class TaskStatusDto {
    private String status;
    private int percentageDone;
    private SolutionDto solution;
    private Integer initPercentage;
    
    public TaskStatusDto() {}
    
    public TaskStatusDto(String status, int percentageDone, SolutionDto solution, Integer initPercentage) {
        this.status = status;
        this.percentageDone = percentageDone;
        this.solution = solution;
        this.initPercentage = initPercentage;
    }
    
    public String getStatus() { return status; }
    public void setStatus(String status) { this.status = status; }
    
    public int getPercentageDone() { return percentageDone; }
    public void setPercentageDone(int percentageDone) { this.percentageDone = percentageDone; }
    
    public SolutionDto getSolution() { return solution; }
    public void setSolution(SolutionDto solution) { this.solution = solution; }
    
    public Integer getInitPercentage() { return initPercentage; }
    public void setInitPercentage(Integer initPercentage) { this.initPercentage = initPercentage; }
}