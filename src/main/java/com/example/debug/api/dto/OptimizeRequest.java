package com.example.debug.api.dto;

import java.util.List;

public class OptimizeRequest {
    private List<PanelDto> panels;
    private List<StockPanelDto> stockPanels;
    private ConfigDto config;
    private Double acceptableQuality;
    
    public List<PanelDto> getPanels() { return panels; }
    public void setPanels(List<PanelDto> panels) { this.panels = panels; }
    
    public List<StockPanelDto> getStockPanels() { return stockPanels; }
    public void setStockPanels(List<StockPanelDto> stockPanels) { this.stockPanels = stockPanels; }
    
    public ConfigDto getConfig() { return config; }
    public void setConfig(ConfigDto config) { this.config = config; }
    
    public Double getAcceptableQuality() { return acceptableQuality; }
    public void setAcceptableQuality(Double acceptableQuality) { this.acceptableQuality = acceptableQuality; }
}