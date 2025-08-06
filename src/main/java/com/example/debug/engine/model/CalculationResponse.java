package com.example.debug.engine.model;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/* loaded from: classes.dex */
public class CalculationResponse {
    private static String version = "1.2";
    private Map<String, Double> edgeBands;
    private long elapsedTime;
    private String id;
    private List<FinalTile> panels;
    private CalculationRequest request;
    private Long solutionElapsedTime;
    private String taskId;
    private double totalCutLength;
    private long totalNbrCuts;
    private double totalUsedArea;
    private double totalUsedAreaRatio;
    private double totalWastedArea;
    private List<FinalTile> usedStockPanels;
    private List<NoFitTile> noFitPanels = new ArrayList();
    private List<Mosaic> mosaics = new ArrayList();

    public String getVersion() {
        return version;
    }

    public String getId() {
        return this.id;
    }

    public void setId(String str) {
        this.id = str;
    }

    public String getTaskId() {
        return this.taskId;
    }

    public void setTaskId(String str) {
        this.taskId = str;
    }

    public long getElapsedTime() {
        return this.elapsedTime;
    }

    public void setElapsedTime(long j) {
        this.elapsedTime = j;
    }

    public Long getSolutionElapsedTime() {
        return this.solutionElapsedTime;
    }

    public void setSolutionElapsedTime(Long l) {
        this.solutionElapsedTime = l;
    }

    public double getTotalUsedArea() {
        return this.totalUsedArea;
    }

    public void setTotalUsedArea(double d) {
        this.totalUsedArea = d;
    }

    public double getTotalWastedArea() {
        return this.totalWastedArea;
    }

    public void setTotalWastedArea(double d) {
        this.totalWastedArea = d;
    }

    public double getTotalUsedAreaRatio() {
        return this.totalUsedAreaRatio;
    }

    public void setTotalUsedAreaRatio(double d) {
        this.totalUsedAreaRatio = d;
    }

    public long getTotalNbrCuts() {
        return this.totalNbrCuts;
    }

    public void setTotalNbrCuts(long j) {
        this.totalNbrCuts = j;
    }

    public double getTotalCutLength() {
        return this.totalCutLength;
    }

    public void setTotalCutLength(double d) {
        this.totalCutLength = d;
    }

    public CalculationRequest getRequest() {
        return this.request;
    }

    public void setRequest(CalculationRequest calculationRequest) {
        this.request = calculationRequest;
    }

    public List<FinalTile> getPanels() {
        return this.panels;
    }

    public void setPanels(List<FinalTile> list) {
        this.panels = list;
    }

    public List<FinalTile> getUsedStockPanels() {
        return this.usedStockPanels;
    }

    public void setUsedStockPanels(List<FinalTile> list) {
        this.usedStockPanels = list;
    }

    public Map<String, Double> getEdgeBands() {
        return this.edgeBands;
    }

    public void setEdgeBands(Map<String, Double> map) {
        this.edgeBands = map;
    }

    public List<NoFitTile> getNoFitPanels() {
        return this.noFitPanels;
    }

    public void setNoFitPanels(List<NoFitTile> list) {
        this.noFitPanels = list;
    }

    public List<Mosaic> getMosaics() {
        return this.mosaics;
    }

    public void setMosaics(List<Mosaic> list) {
        this.mosaics = list;
    }

    public static class Mosaic {
        private double cutLength;
        private Map<String, Double> edgeBands;
        private String material;
        private int nbrFinalPanels;
        private int nbrWastedPanels;
        private List<FinalTile> panels;
        private Integer requestStockId;
        private String stockLabel;
        private double usedArea;
        private float usedAreaRatio;
        private double wastedArea;
        private List<Tile> tiles = new ArrayList();
        private List<Cut> cuts = new ArrayList();

        public Integer getRequestStockId() {
            return this.requestStockId;
        }

        public void setRequestStockId(Integer num) {
            this.requestStockId = num;
        }

        public String getStockLabel() {
            return this.stockLabel;
        }

        public void setStockLabel(String str) {
            this.stockLabel = str;
        }

        public double getUsedArea() {
            return this.usedArea;
        }

        public void setUsedArea(double d) {
            this.usedArea = d;
        }

        public double getWastedArea() {
            return this.wastedArea;
        }

        public void setWastedArea(double d) {
            this.wastedArea = d;
        }

        public float getUsedAreaRatio() {
            return this.usedAreaRatio;
        }

        public void setUsedAreaRatio(float f) {
            this.usedAreaRatio = f;
        }

        public int getNbrFinalPanels() {
            return this.nbrFinalPanels;
        }

        public void setNbrFinalPanels(int i) {
            this.nbrFinalPanels = i;
        }

        public int getNbrWastedPanels() {
            return this.nbrWastedPanels;
        }

        public void setNbrWastedPanels(int i) {
            this.nbrWastedPanels = i;
        }

        public double getCutLength() {
            return this.cutLength;
        }

        public void setCutLength(double d) {
            this.cutLength = d;
        }

        public String getMaterial() {
            return this.material;
        }

        public void setMaterial(String str) {
            if (TileDimensions.DEFAULT_MATERIAL.equals(str)) {
                return;
            }
            this.material = str;
        }

        public Map<String, Double> getEdgeBands() {
            return this.edgeBands;
        }

        public void setEdgeBands(Map<String, Double> map) {
            this.edgeBands = map;
        }

        public List<FinalTile> getPanels() {
            return this.panels;
        }

        public void setPanels(List<FinalTile> list) {
            this.panels = list;
        }

        public List<Tile> getTiles() {
            return this.tiles;
        }

        public void setTiles(List<Tile> list) {
            this.tiles = list;
        }

        public List<Cut> getCuts() {
            return this.cuts;
        }

        public void setCuts(List<Cut> list) {
            this.cuts = list;
        }
    }

    public static class Tile {
        private boolean hasChildren;
        private double height;
        private int id;
        private boolean isFinal;
        private String label;
        private int orientation;
        private Integer requestObjId;
        private double width;
        private double x;
        private double y;
        private Edge edge = new Edge();
        private boolean isRotated = false;

        public Tile() {
        }

        public Tile(int i, int i2, int i3, int i4, int i5) {
            this.id = i;
            this.x = i2;
            this.y = i3;
            this.width = i4;
            this.height = i5;
        }

        public Tile(TileNode tileNode, double d) {
            this.id = tileNode.getId();
            this.requestObjId = tileNode.getExternalId() != -1 ? Integer.valueOf(tileNode.getExternalId()) : null;
            this.x = tileNode.getX1() / d;
            this.y = tileNode.getY1() / d;
            this.width = tileNode.getWidth() / d;
            this.height = tileNode.getHeight() / d;
            setFinal(tileNode.isFinal());
            setHasChildren(tileNode.hasChildren());
        }

        public int getId() {
            return this.id;
        }

        public void setId(int i) {
            this.id = i;
        }

        public Integer getRequestObjId() {
            return this.requestObjId;
        }

        public void setRequestObjId(Integer num) {
            this.requestObjId = num;
        }

        public double getX() {
            return this.x;
        }

        public void setX(double d) {
            this.x = d;
        }

        public double getY() {
            return this.y;
        }

        public void setY(double d) {
            this.y = d;
        }

        public double getWidth() {
            return this.width;
        }

        public void setWidth(double d) {
            this.width = d;
        }

        public double getHeight() {
            return this.height;
        }

        public void setHeight(double d) {
            this.height = d;
        }

        public int getOrientation() {
            return this.orientation;
        }

        public void setOrientation(int i) {
            this.orientation = i;
        }

        public String getLabel() {
            return this.label;
        }

        public void setLabel(String str) {
            this.label = str;
        }

        public boolean isFinal() {
            return this.isFinal;
        }

        public void setFinal(boolean z) {
            this.isFinal = z;
        }

        public boolean isHasChildren() {
            return this.hasChildren;
        }

        public void setHasChildren(boolean z) {
            this.hasChildren = z;
        }

        public Edge getEdge() {
            return this.edge;
        }

        public void setEdge(Edge edge) {
            this.edge = edge;
        }

        public boolean isRotated() {
            return this.isRotated;
        }

        public void setRotated(boolean z) {
            this.isRotated = z;
        }

        public static class Edge {
            private String bottom;
            private String left;
            private String right;
            private String top;

            public String getTop() {
                return this.top;
            }

            public void setTop(String str) {
                this.top = str;
            }

            public String getLeft() {
                return this.left;
            }

            public void setLeft(String str) {
                this.left = str;
            }

            public String getBottom() {
                return this.bottom;
            }

            public void setBottom(String str) {
                this.bottom = str;
            }

            public String getRight() {
                return this.right;
            }

            public void setRight(String str) {
                this.right = str;
            }
        }
    }

    public static class NoFitTile {
        private int count;
        private double height;
        private int id;
        private String label;
        private String material;
        private double width;

        public NoFitTile() {
        }

        public NoFitTile(int i, int i2, int i3, int i4) {
            this.id = i;
            this.width = i2;
            this.height = i3;
            this.count = i4;
        }

        public int getId() {
            return this.id;
        }

        public void setId(int i) {
            this.id = i;
        }

        public double getWidth() {
            return this.width;
        }

        public void setWidth(double d) {
            this.width = d;
        }

        public double getHeight() {
            return this.height;
        }

        public void setHeight(double d) {
            this.height = d;
        }

        public int getCount() {
            return this.count;
        }

        public void setCount(int i) {
            this.count = i;
        }

        public String getLabel() {
            return this.label;
        }

        public void setLabel(String str) {
            this.label = str;
        }

        public String getMaterial() {
            return this.material;
        }

        public void setMaterial(String str) {
            this.material = str;
        }
    }

    public static class FinalTile {
        private int count;
        private double height;
        private String label;
        private int requestObjId;
        private double width;

        public int getRequestObjId() {
            return this.requestObjId;
        }

        public void setRequestObjId(int i) {
            this.requestObjId = i;
        }

        public double getWidth() {
            return this.width;
        }

        public void setWidth(double d) {
            this.width = d;
        }

        public double getHeight() {
            return this.height;
        }

        public void setHeight(double d) {
            this.height = d;
        }

        public String getLabel() {
            return this.label;
        }

        public void setLabel(String str) {
            this.label = str;
        }

        public int getCount() {
            return this.count;
        }

        public void setCount(int i) {
            this.count = i;
        }

        public int countPlusPlus() {
            int i = this.count;
            this.count = i + 1;
            return i;
        }
    }

    public static class Cut {
        private int child1TileId;
        private int child2TileId;
        private double cutCoord;
        private boolean isHorizontal;
        private double originalHeight;
        private int originalTileId;
        private double originalWidth;
        private double x1;
        private double x2;
        private double y1;
        private double y2;

        public Cut() {
        }

        public Cut(com.example.debug.engine.model.Cut cut, double d) {
            this.x1 = cut.getX1() / d;
            this.y1 = cut.getY1() / d;
            this.x2 = cut.getX2() / d;
            this.y2 = cut.getY2() / d;
            this.originalWidth = cut.getOriginalWidth() / d;
            this.originalHeight = cut.getOriginalHeight() / d;
            this.isHorizontal = cut.getIsHorizontal();
            this.cutCoord = cut.getCutCoord() / d;
            this.originalTileId = cut.getOriginalTileId();
            this.child1TileId = cut.getChild1TileId();
            this.child2TileId = cut.getChild2TileId();
        }

        public double getX1() {
            return this.x1;
        }

        public void setX1(double d) {
            this.x1 = d;
        }

        public double getY1() {
            return this.y1;
        }

        public void setY1(double d) {
            this.y1 = d;
        }

        public double getX2() {
            return this.x2;
        }

        public void setX2(double d) {
            this.x2 = d;
        }

        public double getY2() {
            return this.y2;
        }

        public void setY2(double d) {
            this.y2 = d;
        }

        public double getCutCoord() {
            return this.cutCoord;
        }

        public void setCutCoord(double d) {
            this.cutCoord = d;
        }

        public boolean isHorizontal() {
            return this.isHorizontal;
        }

        public void setHorizontal(boolean z) {
            this.isHorizontal = z;
        }

        public int getOriginalTileId() {
            return this.originalTileId;
        }

        public void setOriginalTileId(int i) {
            this.originalTileId = i;
        }

        public double getOriginalWidth() {
            return this.originalWidth;
        }

        public void setOriginalWidth(double d) {
            this.originalWidth = d;
        }

        public double getOriginalHeight() {
            return this.originalHeight;
        }

        public void setOriginalHeight(double d) {
            this.originalHeight = d;
        }

        public int getChild1TileId() {
            return this.child1TileId;
        }

        public void setChild1TileId(int i) {
            this.child1TileId = i;
        }

        public int getChild2TileId() {
            return this.child2TileId;
        }

        public void setChild2TileId(int i) {
            this.child2TileId = i;
        }
    }
}
