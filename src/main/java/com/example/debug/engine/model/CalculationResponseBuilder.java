package com.example.debug.engine.model;

import com.example.debug.engine.EdgeBanding;
import com.example.debug.engine.model.CalculationRequest;
import com.example.debug.engine.model.CalculationResponse;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
public class CalculationResponseBuilder {
    private static final Logger logger = LoggerFactory.getLogger((Class<?>) CalculationResponseBuilder.class);
    private CalculationRequest calculationRequest;
    private List<TileDimensions> noStockMaterialPanels;
    private Map<String, List<Solution>> solutions;
    private Task task;

    public CalculationResponseBuilder setTask(Task task) {
        this.task = task;
        return this;
    }

    public CalculationResponseBuilder setCalculationRequest(CalculationRequest calculationRequest) {
        this.calculationRequest = calculationRequest;
        return this;
    }

    public CalculationResponseBuilder setSolutions(Map<String, List<Solution>> map) {
        this.solutions = map;
        return this;
    }

    public CalculationResponseBuilder setNoStockMaterialPanels(List<TileDimensions> list) {
        this.noStockMaterialPanels = list;
        return this;
    }

    public CalculationResponse build() {
        CalculationResponse calculationResponse = new CalculationResponse();
        List<CalculationRequest.Panel> panels = this.calculationRequest.getPanels();
        List<CalculationRequest.Panel> stockPanels = this.calculationRequest.getStockPanels();
        Solution solution = new Solution();
        ArrayList arrayList = new ArrayList();
        Iterator<Map.Entry<String, List<Solution>>> it = this.solutions.entrySet().iterator();
        long timestamp = 0;
        while (it.hasNext()) {
            List<Solution> value = it.next().getValue();
            if (value != null && value.size() > 0) {
                arrayList.add(Integer.valueOf(value.get(0).getId()));
                solution.addAllMosaics(value.get(0).getMosaics());
                solution.addAllNoFitPanels(value.get(0).getNoFitPanels());
                if (value.get(0).getTimestamp() > timestamp) {
                    timestamp = value.get(0).getTimestamp();
                }
            }
        }
        if (solution.getMosaics().size() > 0) {
            for (Map.Entry<String, List<Solution>> entry : this.solutions.entrySet()) {
                if (entry.getValue() == null || entry.getValue().size() == 0) {
                    Iterator<TileDimensions> it2 = this.task.getTileDimensionsPerMaterial().get(entry.getKey()).iterator();
                    while (it2.hasNext()) {
                        addNoFitTile(calculationResponse, it2.next());
                    }
                }
            }
        }
        List<TileDimensions> list = this.noStockMaterialPanels;
        if (list != null) {
            solution.addAllNoFitPanels(list);
        }
        calculationResponse.setId(Integer.toString(arrayList.hashCode()));
        calculationResponse.setSolutionElapsedTime(timestamp > 0 ? Long.valueOf(timestamp - this.task.getStartTime()) : null);
        calculationResponse.setRequest(this.calculationRequest);
        for (Mosaic mosaic : solution.getMosaics()) {
            CalculationResponse.Mosaic mosaic2 = new CalculationResponse.Mosaic();
            mosaic2.setRequestStockId(Integer.valueOf(mosaic.getStockId()));
            mosaic2.setUsedArea(mosaic.getRootTileNode().getUsedArea() / Math.pow(this.task.getFactor(), 2.0d));
            mosaic2.setUsedAreaRatio(mosaic.getRootTileNode().getUsedAreaRatio());
            mosaic2.setNbrFinalPanels(mosaic.getRootTileNode().getNbrFinalTiles());
            mosaic2.setNbrWastedPanels(mosaic.getRootTileNode().getNbrUnusedTiles());
            mosaic2.setWastedArea(mosaic.getUnusedArea() / Math.pow(this.task.getFactor(), 2.0d));
            mosaic2.setMaterial(mosaic.getMaterial());
            addChildrenToList(mosaic.getRootTileNode(), mosaic2.getTiles());
            calculationResponse.getMosaics().add(mosaic2);
            Iterator<Cut> it3 = mosaic.getCuts().iterator();
            long lenght = 0;
            while (it3.hasNext()) {
                lenght += it3.next().getLenght();
            }
            mosaic2.setCutLength(lenght / this.task.getFactor());
            mosaic2.setEdgeBands(EdgeBanding.calcEdgeBands(mosaic.getFinalTileNodes(), panels, this.task.getFactor()));
            for (CalculationRequest.Panel panel : panels) {
                for (CalculationResponse.Tile tile : mosaic2.getTiles()) {
                    if (tile.getRequestObjId() != null && tile.getRequestObjId().intValue() == panel.getId()) {
                        tile.setOrientation(panel.getOrientation());
                        if (panel.getLabel() != null) {
                            tile.setLabel(panel.getLabel());
                        }
                        if (panel.getEdge() != null) {
                            tile.getEdge().setTop(panel.getEdge().getTop());
                            tile.getEdge().setLeft(panel.getEdge().getLeft());
                            tile.getEdge().setBottom(panel.getEdge().getBottom());
                            tile.getEdge().setRight(panel.getEdge().getRight());
                        }
                    }
                }
            }
            for (CalculationRequest.Panel panel2 : stockPanels) {
                if (mosaic2.getRequestStockId() != null && mosaic2.getRequestStockId().intValue() == panel2.getId()) {
                    mosaic2.setStockLabel(panel2.getLabel());
                    mosaic2.getTiles().get(0).setOrientation(panel2.getOrientation());
                }
            }
            HashMap map = new HashMap();
            for (TileNode tileNode : mosaic.getFinalTileNodes()) {
                if (map.get(Integer.valueOf(tileNode.getExternalId())) != null) {
                    ((CalculationResponse.FinalTile) map.get(Integer.valueOf(tileNode.getExternalId()))).countPlusPlus();
                } else {
                    CalculationResponse.FinalTile finalTile = new CalculationResponse.FinalTile();
                    finalTile.setWidth(tileNode.getWidth() / this.task.getFactor());
                    finalTile.setHeight(tileNode.getHeight() / this.task.getFactor());
                    finalTile.setCount(1);
                    for (CalculationRequest.Panel panel3 : panels) {
                        if (panel3.getLabel() != null && tileNode.getExternalId() == panel3.getId()) {
                            finalTile.setLabel(panel3.getLabel());
                        }
                    }
                    map.put(Integer.valueOf(tileNode.getExternalId()), finalTile);
                }
            }
            mosaic2.setPanels(new ArrayList(map.values()));
            ArrayList arrayList2 = new ArrayList();
            Iterator<Cut> it4 = mosaic.getCuts().iterator();
            while (it4.hasNext()) {
                arrayList2.add(new CalculationResponse.Cut(it4.next(), this.task.getFactor()));
            }
            mosaic2.setCuts(arrayList2);
        }
        Iterator<TileDimensions> it5 = solution.getNoFitPanels().iterator();
        while (it5.hasNext()) {
            addNoFitTile(calculationResponse, it5.next());
        }
        double usedArea = 0.0d;
        double wastedArea = 0.0d;
        double cutLength = 0.0d;
        int size = 0;
        for (CalculationResponse.Mosaic mosaic3 : calculationResponse.getMosaics()) {
            usedArea += mosaic3.getUsedArea();
            wastedArea += mosaic3.getWastedArea();
            size += mosaic3.getCuts().size();
            cutLength += mosaic3.getCutLength();
        }
        calculationResponse.setTaskId(this.task.getId());
        calculationResponse.setTotalUsedArea(usedArea);
        calculationResponse.setTotalWastedArea(wastedArea);
        calculationResponse.setTotalUsedAreaRatio(usedArea / (wastedArea + usedArea));
        calculationResponse.setTotalNbrCuts(size);
        calculationResponse.setTotalCutLength(cutLength);
        calculationResponse.setElapsedTime(this.task.getElapsedTime());
        calculationResponse.setEdgeBands(EdgeBanding.calcEdgeBands(solution.getFinalTileNodes(), panels, this.task.getFactor()));
        HashMap map2 = new HashMap();
        for (TileNode tileNode2 : solution.getFinalTileNodes()) {
            if (map2.get(Integer.valueOf(tileNode2.getExternalId())) != null) {
                ((CalculationResponse.FinalTile) map2.get(Integer.valueOf(tileNode2.getExternalId()))).countPlusPlus();
            } else {
                CalculationResponse.FinalTile finalTile2 = new CalculationResponse.FinalTile();
                if (!tileNode2.isRotated()) {
                    finalTile2.setWidth(tileNode2.getWidth() / this.task.getFactor());
                    finalTile2.setHeight(tileNode2.getHeight() / this.task.getFactor());
                } else {
                    finalTile2.setWidth(tileNode2.getHeight() / this.task.getFactor());
                    finalTile2.setHeight(tileNode2.getWidth() / this.task.getFactor());
                }
                finalTile2.setRequestObjId(tileNode2.getExternalId());
                finalTile2.setCount(1);
                for (CalculationRequest.Panel panel4 : panels) {
                    if (tileNode2.getExternalId() == panel4.getId() && panel4.getLabel() != null) {
                        finalTile2.setLabel(panel4.getLabel());
                    }
                }
                map2.put(Integer.valueOf(tileNode2.getExternalId()), finalTile2);
            }
        }
        calculationResponse.setPanels(new ArrayList(map2.values()));
        HashMap map3 = new HashMap();
        Iterator<Mosaic> it6 = solution.getMosaics().iterator();
        while (it6.hasNext()) {
            TileNode rootTileNode = it6.next().getRootTileNode();
            if (map3.get(Integer.valueOf(rootTileNode.getExternalId())) != null) {
                ((CalculationResponse.FinalTile) map3.get(Integer.valueOf(rootTileNode.getExternalId()))).countPlusPlus();
            } else {
                CalculationResponse.FinalTile finalTile3 = new CalculationResponse.FinalTile();
                finalTile3.setWidth(rootTileNode.getWidth() / this.task.getFactor());
                finalTile3.setHeight(rootTileNode.getHeight() / this.task.getFactor());
                finalTile3.setRequestObjId(rootTileNode.getExternalId());
                finalTile3.setCount(1);
                for (CalculationRequest.Panel panel5 : stockPanels) {
                    if (panel5.getLabel() != null && rootTileNode.getExternalId() == panel5.getId()) {
                        finalTile3.setLabel(panel5.getLabel());
                    }
                }
                map3.put(Integer.valueOf(rootTileNode.getExternalId()), finalTile3);
            }
        }
        calculationResponse.setUsedStockPanels(new ArrayList(map3.values()));
        return calculationResponse;
    }

    public void addNoFitTile(CalculationResponse calculationResponse, TileDimensions tileDimensions) {
        boolean z = false;
        for (CalculationResponse.NoFitTile noFitTile : calculationResponse.getNoFitPanels()) {
            if (noFitTile.getId() == tileDimensions.getId()) {
                noFitTile.setCount(noFitTile.getCount() + 1);
                z = true;
            }
        }
        if (z) {
            return;
        }
        CalculationResponse.NoFitTile noFitTile2 = new CalculationResponse.NoFitTile();
        noFitTile2.setId(tileDimensions.getId());
        noFitTile2.setWidth(tileDimensions.getWidth() / this.task.getFactor());
        noFitTile2.setHeight(tileDimensions.getHeight() / this.task.getFactor());
        noFitTile2.setCount(1);
        for (CalculationRequest.Panel panel : this.calculationRequest.getPanels()) {
            if (noFitTile2.getId() == panel.getId() && panel.getLabel() != null) {
                noFitTile2.setLabel(panel.getLabel());
                noFitTile2.setMaterial(panel.getMaterial());
            }
        }
        calculationResponse.getNoFitPanels().add(noFitTile2);
    }

    private void addChildrenToList(TileNode tileNode, List<CalculationResponse.Tile> list) {
        CalculationResponse.Tile tile = new CalculationResponse.Tile(tileNode, this.task.getFactor());
        list.add(tile);
        if (tileNode.hasChildren()) {
            tile.setHasChildren(true);
            if (tileNode.getChild1() != null) {
                addChildrenToList(tileNode.getChild1(), list);
            }
            if (tileNode.getChild2() != null) {
                addChildrenToList(tileNode.getChild2(), list);
                return;
            }
            return;
        }
        tile.setHasChildren(false);
        tile.setRotated(tileNode.isRotated());
    }
}
