package com.example.debug.api.dto;

public class PanelDto {
    private int id;
    private double width;
    private double height;
    private int count;
    private String label;
    
    public int getId() { return id; }
    public void setId(int id) { this.id = id; }
    
    public double getWidth() { return width; }
    public void setWidth(double width) { this.width = width; }
    
    public double getHeight() { return height; }
    public void setHeight(double height) { this.height = height; }
    
    public int getCount() { return count; }
    public void setCount(int count) { this.count = count; }
    
    public String getLabel() { return label; }
    public void setLabel(String label) { this.label = label; }
}