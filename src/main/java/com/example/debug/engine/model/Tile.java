package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class Tile {
    private final int x1;
    private final int x2;
    private final int y1;
    private final int y2;

    public Tile(TileDimensions tileDimensions) {
        this.x1 = 0;
        this.x2 = tileDimensions.getWidth();
        this.y1 = 0;
        this.y2 = tileDimensions.getHeight();
    }

    public Tile(int i, int i2, int i3, int i4) {
        this.x1 = i;
        this.x2 = i2;
        this.y1 = i3;
        this.y2 = i4;
    }

    public Tile(Tile tile) {
        this.x1 = tile.x1;
        this.x2 = tile.x2;
        this.y1 = tile.y1;
        this.y2 = tile.y2;
    }

    public int getX1() {
        return this.x1;
    }

    public int getX2() {
        return this.x2;
    }

    public int getY1() {
        return this.y1;
    }

    public int getY2() {
        return this.y2;
    }

    public int getWidth() {
        return this.x2 - this.x1;
    }

    public int getHeight() {
        return this.y2 - this.y1;
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

    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (!(obj instanceof Tile)) {
            return false;
        }
        Tile tile = (Tile) obj;
        return this.x1 == tile.x1 && this.x2 == tile.x2 && this.y1 == tile.y1 && this.y2 == tile.y2;
    }
}
