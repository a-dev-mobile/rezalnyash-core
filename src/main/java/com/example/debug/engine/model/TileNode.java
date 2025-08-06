package com.example.debug.engine.model;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/* loaded from: classes.dex */
public class TileNode {
    private static final AtomicInteger NEXT_ID = new AtomicInteger(0);
    private TileNode child1;
    private TileNode child2;
    private int externalId;
    private final int id;
    private boolean isAreaTotallyUsed;
    private boolean isFinal;
    private boolean isRotated;
    private Tile tile;
    private long totallyUsedArea;

    public TileNode(int i, int i2, int i3, int i4) {
        this.externalId = -1;
        this.isAreaTotallyUsed = false;
        this.totallyUsedArea = 0L;
        this.isRotated = false;
        this.tile = new Tile(i, i2, i3, i4);
        this.id = NEXT_ID.getAndIncrement();
    }

    public TileNode(TileDimensions tileDimensions) {
        this.externalId = -1;
        this.isAreaTotallyUsed = false;
        this.totallyUsedArea = 0L;
        this.isRotated = false;
        this.tile = new Tile(tileDimensions);
        this.id = NEXT_ID.getAndIncrement();
    }

    public TileNode(TileNode tileNode) {
        this.externalId = -1;
        this.isAreaTotallyUsed = false;
        this.totallyUsedArea = 0L;
        this.isRotated = false;
        this.id = tileNode.id;
        this.externalId = tileNode.externalId;
        this.tile = tileNode.tile;
        this.isFinal = tileNode.isFinal;
        this.isAreaTotallyUsed = false;
        this.totallyUsedArea = 0L;
        this.isRotated = tileNode.isRotated;
        if (tileNode.getChild1() != null) {
            this.child1 = tileNode.getChild1();
        }
        if (tileNode.getChild2() != null) {
            this.child2 = tileNode.getChild2();
        }
    }

    public Tile getTile() {
        return this.tile;
    }

    public void setTile(Tile tile) {
        this.tile = tile;
    }

    public boolean isFinal() {
        return this.isFinal;
    }

    public void setFinal(boolean z) {
        this.isFinal = z;
    }

    public int getExternalId() {
        return this.externalId;
    }

    public void setExternalId(int i) {
        this.externalId = i;
    }

    public int getId() {
        return this.id;
    }

    public TileNode getChild1() {
        return this.child1;
    }

    public void setChild1(TileNode tileNode) {
        this.child1 = tileNode;
    }

    public TileNode getChild2() {
        return this.child2;
    }

    public void setChild2(TileNode tileNode) {
        this.child2 = tileNode;
    }

    public boolean hasChildren() {
        return (this.child1 == null && this.child2 == null) ? false : true;
    }

    public boolean isRotated() {
        return this.isRotated;
    }

    public void setRotated(boolean z) {
        this.isRotated = z;
    }

    public TileNode findTile(TileNode tileNode) {
        TileNode tileNodeFindTile;
        if (equals(tileNode)) {
            return this;
        }
        if (getChild1() != null && (tileNodeFindTile = getChild1().findTile(tileNode)) != null) {
            return tileNodeFindTile;
        }
        if (getChild2() != null) {
            return getChild2().findTile(tileNode);
        }
        return null;
    }

    public TileNode replaceTile(TileNode tileNode, TileNode tileNode2) {
        if (getChild1() != null && getChild1().findTile(tileNode2) != null) {
            setChild1(tileNode);
            return this.child1;
        }
        if (getChild2() == null || getChild2().findTile(tileNode2) == null) {
            return null;
        }
        setChild2(tileNode);
        return this.child2;
    }

    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (!(obj instanceof TileNode)) {
            return false;
        }
        TileNode tileNode = (TileNode) obj;
        if (this.id == tileNode.id && this.tile.equals(tileNode.tile) && this.isFinal == tileNode.isFinal && this.child1 == null && tileNode.child1 == null) {
            return true;
        }
        TileNode tileNode2 = this.child1;
        if (tileNode2 != null && tileNode2.equals(tileNode.child1) && this.child2 == null && tileNode.child2 == null) {
            return true;
        }
        TileNode tileNode3 = this.child2;
        return tileNode3 != null && tileNode3.equals(tileNode.child2);
    }

    public String toString() {
        return appendToString("");
    }

    public String appendToString(String str) {
        String str2 = System.getProperty("line.separator") + str + "(" + getX1() + ", " + getY1() + ")(" + getX2() + ", " + getY2() + ")";
        if (isFinal()) {
            str2 = str2 + '*';
        }
        if (this.child1 != null) {
            String str3 = str + "    ";
            str2 = str2 + this.child1.appendToString(str3);
            str = str3.substring(0, str3.length() - 4);
        }
        if (this.child2 == null) {
            return str2;
        }
        String str4 = str + "    ";
        return str2 + str4 + this.child2.appendToString(str4);
    }

    public String toStringIdentifier() {
        StringBuilder sb = new StringBuilder();
        appendToStringIdentifier(sb);
        return sb.toString();
    }

    private void appendToStringIdentifier(StringBuilder sb) {
        sb.append(this.tile.getX1());
        sb.append(this.tile.getY1());
        sb.append(this.tile.getX2());
        sb.append(this.tile.getY2());
        sb.append(this.isFinal);
        TileNode tileNode = this.child1;
        if (tileNode != null) {
            tileNode.appendToStringIdentifier(sb);
        }
        TileNode tileNode2 = this.child2;
        if (tileNode2 != null) {
            tileNode2.appendToStringIdentifier(sb);
        }
    }

    public long getUsedArea() {
        if (this.isAreaTotallyUsed) {
            return this.totallyUsedArea;
        }
        if (this.isFinal) {
            return getArea();
        }
        TileNode tileNode = this.child1;
        long usedArea = tileNode != null ? tileNode.getUsedArea() : 0L;
        TileNode tileNode2 = this.child2;
        if (tileNode2 != null) {
            usedArea += tileNode2.getUsedArea();
        }
        if (usedArea == getArea()) {
            this.isAreaTotallyUsed = true;
            this.totallyUsedArea = getArea();
        }
        return usedArea;
    }

    public List<TileNode> getUnusedTiles() {
        ArrayList arrayList = new ArrayList();
        getUnusedTiles(arrayList);
        return arrayList;
    }

    public void getUnusedTiles(List<TileNode> list) {
        if (!isFinal() && getChild1() == null && getChild2() == null) {
            list.add(this);
        }
        if (getChild1() != null) {
            getChild1().getUnusedTiles(list);
        }
        if (getChild2() != null) {
            getChild2().getUnusedTiles(list);
        }
    }

    public List<Tile> getFinalTiles() {
        ArrayList arrayList = new ArrayList();
        getFinalTiles(arrayList);
        return arrayList;
    }

    public void getFinalTiles(List<Tile> list) {
        if (isFinal()) {
            list.add(this.tile);
        }
        if (getChild1() != null) {
            getChild1().getFinalTiles(list);
        }
        if (getChild2() != null) {
            getChild2().getFinalTiles(list);
        }
    }

    public List<TileNode> getFinalTileNodes() {
        ArrayList arrayList = new ArrayList();
        getFinalTileNodes(arrayList);
        return arrayList;
    }

    public void getFinalTileNodes(List<TileNode> list) {
        if (isFinal()) {
            list.add(this);
        }
        if (getChild1() != null) {
            getChild1().getFinalTileNodes(list);
        }
        if (getChild2() != null) {
            getChild2().getFinalTileNodes(list);
        }
    }

    public long getUnusedArea() {
        return getArea() - getUsedArea();
    }

    public float getUsedAreaRatio() {
        return (float) (getUsedArea() / getArea());
    }

    public boolean hasFinal() {
        TileNode tileNode;
        TileNode tileNode2;
        return isFinal() || ((tileNode = this.child1) != null && tileNode.hasFinal()) || ((tileNode2 = this.child2) != null && tileNode2.hasFinal());
    }

    public int getNbrUnusedTiles() {
        int nbrUnusedTiles = (!isFinal() && this.child1 == null && this.child2 == null) ? 1 : 0;
        TileNode tileNode = this.child1;
        if (tileNode != null) {
            nbrUnusedTiles += tileNode.getNbrUnusedTiles();
        }
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? nbrUnusedTiles + tileNode2.getNbrUnusedTiles() : nbrUnusedTiles;
    }

    public int getDepth() {
        TileNode tileNode = this.child1;
        int depth = tileNode != null ? 1 + tileNode.getDepth() : 0;
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? depth + 1 + tileNode2.getDepth() : depth;
    }

    /* JADX WARN: Type inference failed for: r0v0, types: [boolean, int] */
    public int getNbrFinalTiles() {
        int IsFinal = isFinal() ? 1 : 0;
        TileNode tileNode = this.child1;
        int nbrFinalTiles = IsFinal;
        if (tileNode != null) {
            nbrFinalTiles = IsFinal + tileNode.getNbrFinalTiles();
        }
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? nbrFinalTiles + tileNode2.getNbrFinalTiles() : nbrFinalTiles;
    }

    public long getBiggestArea() {
        long area = (getChild1() == null && getChild2() == null && !this.isFinal) ? getArea() : 0L;
        TileNode tileNode = this.child1;
        if (tileNode != null) {
            area = Math.max(tileNode.getBiggestArea(), area);
        }
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? Math.max(tileNode2.getBiggestArea(), area) : area;
    }

    public int getNbrFinalHorizontal() {
        int nbrFinalHorizontal = (isFinal() && isHorizontal()) ? 1 : 0;
        TileNode tileNode = this.child1;
        if (tileNode != null) {
            nbrFinalHorizontal += tileNode.getNbrFinalHorizontal();
        }
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? nbrFinalHorizontal + tileNode2.getNbrFinalHorizontal() : nbrFinalHorizontal;
    }

    public int getNbrFinalVertical() {
        int nbrFinalVertical = (isFinal() && isVertical()) ? 1 : 0;
        TileNode tileNode = this.child1;
        if (tileNode != null) {
            nbrFinalVertical += tileNode.getNbrFinalVertical();
        }
        TileNode tileNode2 = this.child2;
        return tileNode2 != null ? nbrFinalVertical + tileNode2.getNbrFinalVertical() : nbrFinalVertical;
    }

    public HashSet<Integer> getDistictTileSet() {
        return getDistictTileSet(new HashSet<>());
    }

    private HashSet<Integer> getDistictTileSet(HashSet<Integer> hashSet) {
        if (this.isFinal) {
            int width = this.tile.getWidth();
            int height = this.tile.getHeight();
            int i = width + height;
            hashSet.add(Integer.valueOf(((i * (i + 1)) / 2) + height));
        } else {
            TileNode tileNode = this.child1;
            if (tileNode != null) {
                tileNode.getDistictTileSet(hashSet);
            }
            TileNode tileNode2 = this.child2;
            if (tileNode2 != null) {
                tileNode2.getDistictTileSet(hashSet);
            }
        }
        return hashSet;
    }

    public TileDimensions toTileDimensions() {
        return new TileDimensions(getWidth(), getHeight());
    }

    public int getX1() {
        return this.tile.getX1();
    }

    public int getX2() {
        return this.tile.getX2();
    }

    public int getY1() {
        return this.tile.getY1();
    }

    public int getY2() {
        return this.tile.getY2();
    }

    public int getWidth() {
        return this.tile.getWidth();
    }

    public int getHeight() {
        return this.tile.getHeight();
    }

    public long getArea() {
        return getWidth() * getHeight();
    }

    public int getMaxSide() {
        return Math.max(getWidth(), getHeight());
    }

    public boolean isHorizontal() {
        return getWidth() > getHeight();
    }

    public boolean isVertical() {
        return getHeight() > getWidth();
    }
}
