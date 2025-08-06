package com.example.debug.engine.model;

import com.example.debug.engine.stock.StockSolution;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/* loaded from: classes.dex */
public class Solution {
    private static final AtomicInteger idAtomicInteger = new AtomicInteger(0);
    private String auxInfo;
    private String creatorThreadGroup;
    private final int id;
    private List<Mosaic> mosaics;
    private List<TileDimensions> noFitPanels;
    private final long timestamp;
    LinkedList<TileDimensions> unusedStockPanels;

    public Solution() {
        this.unusedStockPanels = new LinkedList<>();
        this.timestamp = System.currentTimeMillis();
        this.mosaics = new ArrayList();
        this.noFitPanels = new ArrayList();
        this.id = idAtomicInteger.getAndIncrement();
    }

    public Solution(Solution solution) {
        this.unusedStockPanels = new LinkedList<>();
        this.timestamp = System.currentTimeMillis();
        this.mosaics = new ArrayList();
        Iterator<Mosaic> it = solution.mosaics.iterator();
        while (it.hasNext()) {
            this.mosaics.add(new Mosaic(it.next()));
        }
        Iterator<TileDimensions> it2 = solution.getUnusedStockPanels().iterator();
        while (it2.hasNext()) {
            this.unusedStockPanels.add(new TileDimensions(it2.next()));
        }
        this.noFitPanels = new ArrayList(solution.getNoFitPanels());
        this.id = idAtomicInteger.getAndIncrement();
    }

    public Solution(TileDimensions tileDimensions) {
        this.unusedStockPanels = new LinkedList<>();
        this.timestamp = System.currentTimeMillis();
        this.mosaics = new ArrayList();
        this.noFitPanels = new ArrayList();
        this.id = idAtomicInteger.getAndIncrement();
        addMosaic(new Mosaic(tileDimensions));
    }

    public Solution(StockSolution stockSolution) {
        this.unusedStockPanels = new LinkedList<>();
        this.timestamp = System.currentTimeMillis();
        this.mosaics = new ArrayList();
        this.noFitPanels = new ArrayList();
        Iterator<TileDimensions> it = stockSolution.getStockTileDimensions().iterator();
        while (it.hasNext()) {
            this.unusedStockPanels.add(new TileDimensions(it.next()));
        }
        addMosaic(new Mosaic(this.unusedStockPanels.poll()));
        this.id = idAtomicInteger.getAndIncrement();
    }

    public Solution(Solution solution, Mosaic mosaic) {
        this.unusedStockPanels = new LinkedList<>();
        this.timestamp = System.currentTimeMillis();
        this.mosaics = new ArrayList();
        for (Mosaic mosaic2 : solution.mosaics) {
            if (mosaic2 != mosaic) {
                this.mosaics.add(new Mosaic(mosaic2));
            }
        }
        Iterator<TileDimensions> it = solution.getUnusedStockPanels().iterator();
        while (it.hasNext()) {
            this.unusedStockPanels.add(new TileDimensions(it.next()));
        }
        this.noFitPanels = new ArrayList(solution.getNoFitPanels());
        this.id = idAtomicInteger.getAndIncrement();
    }

    public LinkedList<TileDimensions> getUnusedStockPanels() {
        return this.unusedStockPanels;
    }

    public String getCreatorThreadGroup() {
        return this.creatorThreadGroup;
    }

    public void setCreatorThreadGroup(String str) {
        this.creatorThreadGroup = str;
    }

    public String getAuxInfo() {
        return this.auxInfo;
    }

    public void setAuxInfo(String str) {
        this.auxInfo = str;
    }

    public int getId() {
        return this.id;
    }

    private void sortMosaics() {
        Collections.sort(this.mosaics, new Comparator<Mosaic>() { // from class: com.cutlistoptimizer.engine.model.Solution.1
            @Override // java.util.Comparator
            public int compare(Mosaic mosaic, Mosaic mosaic2) {
                return Long.compare(mosaic.getUnusedArea(), mosaic2.getUnusedArea());
            }
        });
    }

    public void addMosaic(Mosaic mosaic) {
        this.mosaics.add(mosaic);
        sortMosaics();
    }

    public void addAllMosaics(Collection<Mosaic> collection) {
        this.mosaics.addAll(collection);
        sortMosaics();
    }

    public final List<Mosaic> getMosaics() {
        return this.mosaics;
    }

    public void removeMosaic(Mosaic mosaic) {
        this.mosaics.remove(mosaic);
    }

    public float getUsedAreaRatio() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        float usedAreaRatio = 0.0f;
        while (it.hasNext()) {
            usedAreaRatio += it.next().getRootTileNode().getUsedAreaRatio();
        }
        return usedAreaRatio / this.mosaics.size();
    }

    public boolean hasUnusedBaseTile() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        return it.hasNext() && !it.next().getRootTileNode().hasFinal();
    }

    public int getNbrUnusedTiles() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int nbrUnusedTiles = 0;
        while (it.hasNext()) {
            nbrUnusedTiles += it.next().getRootTileNode().getNbrUnusedTiles();
        }
        return nbrUnusedTiles;
    }

    public String getBasesAsString() {
        String str = new String();
        for (Mosaic mosaic : this.mosaics) {
            str = str + "[" + mosaic.getRootTileNode().getWidth() + "x" + mosaic.getRootTileNode().getHeight() + "]";
        }
        return str;
    }

    public int getNbrHorizontal() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int nbrFinalHorizontal = 0;
        while (it.hasNext()) {
            nbrFinalHorizontal += it.next().getRootTileNode().getNbrFinalHorizontal();
        }
        return nbrFinalHorizontal;
    }

    public List<TileNode> getFinalTileNodes() {
        ArrayList arrayList = new ArrayList();
        Iterator<Mosaic> it = this.mosaics.iterator();
        while (it.hasNext()) {
            arrayList.addAll(it.next().getRootTileNode().getFinalTileNodes());
        }
        return arrayList;
    }

    public List<Tile> getFinalTiles() {
        ArrayList arrayList = new ArrayList();
        Iterator<Mosaic> it = this.mosaics.iterator();
        while (it.hasNext()) {
            arrayList.addAll(it.next().getRootTileNode().getFinalTiles());
        }
        return arrayList;
    }

    public int getNbrFinalTiles() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int nbrFinalTiles = 0;
        while (it.hasNext()) {
            nbrFinalTiles += it.next().getRootTileNode().getNbrFinalTiles();
        }
        return nbrFinalTiles;
    }

    public float getHVDiff() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        float hVDiff = 0.0f;
        while (it.hasNext()) {
            hVDiff += it.next().getHVDiff();
        }
        return hVDiff / this.mosaics.size();
    }

    public long getTotalArea() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        long area = 0;
        while (it.hasNext()) {
            area += it.next().getRootTileNode().getArea();
        }
        return area;
    }

    public long getUsedArea() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        long usedArea = 0;
        while (it.hasNext()) {
            usedArea += it.next().getRootTileNode().getUsedArea();
        }
        return usedArea;
    }

    public long getUnusedArea() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        long unusedArea = 0;
        while (it.hasNext()) {
            unusedArea += it.next().getRootTileNode().getUnusedArea();
        }
        return unusedArea;
    }

    public List<TileDimensions> getNoFitPanels() {
        return this.noFitPanels;
    }

    public void setNoFitPanels(List<TileDimensions> list) {
        this.noFitPanels = list;
    }

    public void addAllNoFitPanels(Collection<TileDimensions> collection) {
        this.noFitPanels.addAll(collection);
    }

    public int getMaxDepth() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int iMax = 0;
        while (it.hasNext()) {
            iMax = Math.max(it.next().getDepth(), iMax);
        }
        return iMax;
    }

    public int getNbrCuts() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int nbrCuts = 0;
        while (it.hasNext()) {
            nbrCuts += it.next().getNbrCuts();
        }
        return nbrCuts;
    }

    public int getDistictTileSet() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        int iMax = 0;
        while (it.hasNext()) {
            iMax = Math.max(it.next().getDistictTileSet().size(), iMax);
        }
        return iMax;
    }

    public int getNbrMosaics() {
        return this.mosaics.size();
    }

    public List<TileDimensions> getStockTilesDimensions() {
        ArrayList arrayList = new ArrayList();
        Iterator<Mosaic> it = this.mosaics.iterator();
        while (it.hasNext()) {
            arrayList.add(it.next().getRootTileNode().toTileDimensions());
        }
        return arrayList;
    }

    public long getMostUnusedPanelArea() {
        long unusedArea = 0;
        for (Mosaic mosaic : this.mosaics) {
            if (unusedArea < mosaic.getUnusedArea()) {
                unusedArea = mosaic.getUnusedArea();
            }
        }
        return unusedArea;
    }

    public float getCenterOfMassDistanceToOrigin() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        float centerOfMassDistanceToOrigin = 0.0f;
        while (it.hasNext()) {
            centerOfMassDistanceToOrigin += it.next().getCenterOfMassDistanceToOrigin();
        }
        return centerOfMassDistanceToOrigin / getNbrMosaics();
    }

    public long getBiggestArea() {
        Iterator<Mosaic> it = this.mosaics.iterator();
        long jMax = 0;
        while (it.hasNext()) {
            jMax = Math.max(it.next().getBiggestArea(), jMax);
        }
        return jMax;
    }

    public long getTimestamp() {
        return this.timestamp;
    }

    public String getMaterial() {
        if (this.mosaics.size() > 0) {
            return this.mosaics.get(0).getMaterial();
        }
        return null;
    }
}
