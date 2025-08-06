package com.example.debug.engine.model;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

/* loaded from: classes.dex */
public class Mosaic {
    private List<Cut> cuts;
    private String material;
    private int orientation;
    private TileNode rootTileNode;
    private int stockId;

    public Mosaic(Mosaic mosaic) {
        this.rootTileNode = new TileNode(mosaic.getRootTileNode());
        this.cuts = new ArrayList(mosaic.getCuts());
        this.stockId = mosaic.stockId;
        this.material = mosaic.material;
        this.orientation = mosaic.orientation;
    }

    public Mosaic(TileNode tileNode, String str) {
        this.cuts = new ArrayList();
        this.rootTileNode = new TileNode(tileNode);
        this.stockId = tileNode.getExternalId();
        this.material = str;
    }

    public Mosaic(TileDimensions tileDimensions) {
        this.cuts = new ArrayList();
        this.rootTileNode = new TileNode(tileDimensions);
        this.material = tileDimensions.material;
        this.orientation = tileDimensions.orientation;
        this.rootTileNode.setExternalId(tileDimensions.getId());
        this.stockId = tileDimensions.getId();
    }

    public TileNode getRootTileNode() {
        return this.rootTileNode;
    }

    public void setRootTileNode(TileNode tileNode) {
        this.rootTileNode = tileNode;
    }

    public List<Cut> getCuts() {
        return this.cuts;
    }

    public void setCuts(List<Cut> list) {
        this.cuts = list;
    }

    public int getNbrCuts() {
        return this.cuts.size();
    }

    public int getStockId() {
        return this.stockId;
    }

    public void setStockId(int i) {
        this.stockId = i;
    }

    public String getMaterial() {
        return this.material;
    }

    public void setMaterial(String str) {
        this.material = str;
    }

    public int getOrientation() {
        return this.orientation;
    }

    public void setOrientation(int i) {
        this.orientation = i;
    }

    public List<TileNode> getFinalTileNodes() {
        return this.rootTileNode.getFinalTileNodes();
    }

    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj instanceof Mosaic) {
            return ((Mosaic) obj).getRootTileNode().equals(getRootTileNode());
        }
        return false;
    }

    public float getHVDiff() {
        return Math.abs(this.rootTileNode.getNbrFinalHorizontal() - this.rootTileNode.getNbrFinalVertical());
    }

    public HashSet<Integer> getDistictTileSet() {
        return this.rootTileNode.getDistictTileSet();
    }

    public long getUsedArea() {
        return this.rootTileNode.getUsedArea();
    }

    public long getUnusedArea() {
        return this.rootTileNode.getUnusedArea();
    }

    public int getDepth() {
        return this.rootTileNode.getDepth();
    }

    public TileNode getBiggestUnusedTile() {
        TileNode tileNode = null;
        for (TileNode tileNode2 : this.rootTileNode.getUnusedTiles()) {
            if (tileNode == null || tileNode.getArea() < tileNode2.getArea()) {
                tileNode = tileNode2;
            }
        }
        return tileNode;
    }

    public float getCenterOfMassDistanceToOrigin() {
        float area = 0.0f;
        if (getUsedArea() == 0) {
            return 0.0f;
        }
        float area2 = 0.0f;
        for (TileNode tileNode : getRootTileNode().getFinalTileNodes()) {
            area += tileNode.getArea() * (tileNode.getX1() + (tileNode.getWidth() * 0.5f));
            area2 += tileNode.getArea() * (tileNode.getY1() + (tileNode.getHeight() * 0.5f));
        }
        return ((float) Math.sqrt(Math.pow(area / getUsedArea(), 2.0d) + Math.pow(area2 / getUsedArea(), 2.0d))) / ((float) Math.sqrt(Math.pow(getRootTileNode().getWidth(), 2.0d) + Math.pow(getRootTileNode().getHeight(), 2.0d)));
    }

    public long getBiggestArea() {
        return this.rootTileNode.getBiggestArea();
    }
}
