package com.example.debug.api.dto;

import java.util.List;

public class MosaicDto {
    private String stockLabel;
    private double usedArea;
    private double wastedArea;
    private float usedAreaRatio;
    private int nbrFinalPanels;
    private int nbrWastedPanels;
    private double cutLength;
    private String material;
    private List<TileDto> tiles;
    private List<CutDto> cuts;
    
    public MosaicDto() {}
    
    public MosaicDto(String stockLabel, double usedArea, double wastedArea, float usedAreaRatio,
                    int nbrFinalPanels, int nbrWastedPanels, double cutLength, String material,
                    List<TileDto> tiles, List<CutDto> cuts) {
        this.stockLabel = stockLabel;
        this.usedArea = usedArea;
        this.wastedArea = wastedArea;
        this.usedAreaRatio = usedAreaRatio;
        this.nbrFinalPanels = nbrFinalPanels;
        this.nbrWastedPanels = nbrWastedPanels;
        this.cutLength = cutLength;
        this.material = material;
        this.tiles = tiles;
        this.cuts = cuts;
    }
    
    public String getStockLabel() { return stockLabel; }
    public void setStockLabel(String stockLabel) { this.stockLabel = stockLabel; }
    
    public double getUsedArea() { return usedArea; }
    public void setUsedArea(double usedArea) { this.usedArea = usedArea; }
    
    public double getWastedArea() { return wastedArea; }
    public void setWastedArea(double wastedArea) { this.wastedArea = wastedArea; }
    
    public float getUsedAreaRatio() { return usedAreaRatio; }
    public void setUsedAreaRatio(float usedAreaRatio) { this.usedAreaRatio = usedAreaRatio; }
    
    public int getNbrFinalPanels() { return nbrFinalPanels; }
    public void setNbrFinalPanels(int nbrFinalPanels) { this.nbrFinalPanels = nbrFinalPanels; }
    
    public int getNbrWastedPanels() { return nbrWastedPanels; }
    public void setNbrWastedPanels(int nbrWastedPanels) { this.nbrWastedPanels = nbrWastedPanels; }
    
    public double getCutLength() { return cutLength; }
    public void setCutLength(double cutLength) { this.cutLength = cutLength; }
    
    public String getMaterial() { return material; }
    public void setMaterial(String material) { this.material = material; }
    
    public List<TileDto> getTiles() { return tiles; }
    public void setTiles(List<TileDto> tiles) { this.tiles = tiles; }
    
    public List<CutDto> getCuts() { return cuts; }
    public void setCuts(List<CutDto> cuts) { this.cuts = cuts; }
}