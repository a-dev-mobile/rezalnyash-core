package com.example.debug.api.dto;

public class TileDto {
    private int id;
    private Integer requestObjId;
    private double x;
    private double y;
    private double width;
    private double height;
    private int orientation;
    private String label;
    private boolean isFinal;
    private boolean hasChildren;
    private boolean isRotated;
    
    public TileDto() {}
    
    public TileDto(int id, Integer requestObjId, double x, double y, double width, double height,
                  int orientation, String label, boolean isFinal, boolean hasChildren, boolean isRotated) {
        this.id = id;
        this.requestObjId = requestObjId;
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
        this.orientation = orientation;
        this.label = label;
        this.isFinal = isFinal;
        this.hasChildren = hasChildren;
        this.isRotated = isRotated;
    }
    
    public int getId() { return id; }
    public void setId(int id) { this.id = id; }
    
    public Integer getRequestObjId() { return requestObjId; }
    public void setRequestObjId(Integer requestObjId) { this.requestObjId = requestObjId; }
    
    public double getX() { return x; }
    public void setX(double x) { this.x = x; }
    
    public double getY() { return y; }
    public void setY(double y) { this.y = y; }
    
    public double getWidth() { return width; }
    public void setWidth(double width) { this.width = width; }
    
    public double getHeight() { return height; }
    public void setHeight(double height) { this.height = height; }
    
    public int getOrientation() { return orientation; }
    public void setOrientation(int orientation) { this.orientation = orientation; }
    
    public String getLabel() { return label; }
    public void setLabel(String label) { this.label = label; }
    
    public boolean isFinal() { return isFinal; }
    public void setFinal(boolean isFinal) { this.isFinal = isFinal; }
    
    public boolean isHasChildren() { return hasChildren; }
    public void setHasChildren(boolean hasChildren) { this.hasChildren = hasChildren; }
    
    public boolean isRotated() { return isRotated; }
    public void setRotated(boolean isRotated) { this.isRotated = isRotated; }
}