package com.example.debug.api.dto;

public class CutDto {
    private double x1;
    private double y1;
    private double x2;
    private double y2;
    private double cutCoord;
    private boolean isHorizontal;
    private int originalTileId;
    private double originalWidth;
    private double originalHeight;
    private int child1TileId;
    private int child2TileId;
    
    public CutDto() {}
    
    public CutDto(double x1, double y1, double x2, double y2, double cutCoord, boolean isHorizontal,
                 int originalTileId, double originalWidth, double originalHeight, int child1TileId, int child2TileId) {
        this.x1 = x1;
        this.y1 = y1;
        this.x2 = x2;
        this.y2 = y2;
        this.cutCoord = cutCoord;
        this.isHorizontal = isHorizontal;
        this.originalTileId = originalTileId;
        this.originalWidth = originalWidth;
        this.originalHeight = originalHeight;
        this.child1TileId = child1TileId;
        this.child2TileId = child2TileId;
    }
    
    public double getX1() { return x1; }
    public void setX1(double x1) { this.x1 = x1; }
    
    public double getY1() { return y1; }
    public void setY1(double y1) { this.y1 = y1; }
    
    public double getX2() { return x2; }
    public void setX2(double x2) { this.x2 = x2; }
    
    public double getY2() { return y2; }
    public void setY2(double y2) { this.y2 = y2; }
    
    public double getCutCoord() { return cutCoord; }
    public void setCutCoord(double cutCoord) { this.cutCoord = cutCoord; }
    
    public boolean isHorizontal() { return isHorizontal; }
    public void setHorizontal(boolean isHorizontal) { this.isHorizontal = isHorizontal; }
    
    public int getOriginalTileId() { return originalTileId; }
    public void setOriginalTileId(int originalTileId) { this.originalTileId = originalTileId; }
    
    public double getOriginalWidth() { return originalWidth; }
    public void setOriginalWidth(double originalWidth) { this.originalWidth = originalWidth; }
    
    public double getOriginalHeight() { return originalHeight; }
    public void setOriginalHeight(double originalHeight) { this.originalHeight = originalHeight; }
    
    public int getChild1TileId() { return child1TileId; }
    public void setChild1TileId(int child1TileId) { this.child1TileId = child1TileId; }
    
    public int getChild2TileId() { return child2TileId; }
    public void setChild2TileId(int child2TileId) { this.child2TileId = child2TileId; }
}