package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class Cut {
    private final int child1TileId;
    private final int child2TileId;
    private final int cutCoord;
    private final boolean isHorizontal;
    private final int originalHeight;
    private final int originalTileId;
    private final int originalWidth;
    private final int x1;
    private final int x2;
    private final int y1;
    private final int y2;

    public Cut(Cut cut) {
        this.x1 = cut.x1;
        this.y1 = cut.y1;
        this.x2 = cut.x2;
        this.y2 = cut.y2;
        this.originalWidth = cut.originalWidth;
        this.originalHeight = cut.originalHeight;
        this.isHorizontal = cut.isHorizontal;
        this.cutCoord = cut.cutCoord;
        this.originalTileId = cut.originalTileId;
        this.child1TileId = cut.child1TileId;
        this.child2TileId = cut.child2TileId;
    }

    public Cut(int i, int i2, int i3, int i4, int i5, int i6, boolean z, int i7, int i8, int i9, int i10) {
        this.x1 = i;
        this.x2 = i3;
        this.y1 = i2;
        this.y2 = i4;
        this.originalWidth = i5;
        this.originalHeight = i6;
        this.isHorizontal = z;
        this.cutCoord = i7;
        this.originalTileId = i8;
        this.child1TileId = i9;
        this.child2TileId = i10;
    }

    public Cut(Builder builder) {
        this.x1 = builder.x1;
        this.x2 = builder.x2;
        this.y1 = builder.y1;
        this.y2 = builder.y2;
        this.originalWidth = builder.originalWidth;
        this.originalHeight = builder.originalHeight;
        this.isHorizontal = builder.isHorizontal;
        this.cutCoord = builder.cutCoord;
        this.originalTileId = builder.originalTileId;
        this.child1TileId = builder.child1TileId;
        this.child2TileId = builder.child2TileId;
    }

    public int getX1() {
        return this.x1;
    }

    public int getY1() {
        return this.y1;
    }

    public int getX2() {
        return this.x2;
    }

    public int getY2() {
        return this.y2;
    }

    public int getOriginalTileId() {
        return this.originalTileId;
    }

    public int getChild1TileId() {
        return this.child1TileId;
    }

    public int getChild2TileId() {
        return this.child2TileId;
    }

    public int getOriginalWidth() {
        return this.originalWidth;
    }

    public int getOriginalHeight() {
        return this.originalHeight;
    }

    public boolean getIsHorizontal() {
        return this.isHorizontal;
    }

    public int getCutCoord() {
        return this.cutCoord;
    }

    public long getLenght() {
        return Math.abs(this.x2 - this.x1) + Math.abs(this.y2 - this.y1);
    }

    public static class Builder {
        private int child1TileId;
        private int child2TileId;
        private int cutCoord;
        private boolean isHorizontal;
        private int originalHeight;
        private int originalTileId;
        private int originalWidth;
        private int x1;
        private int x2;
        private int y1;
        private int y2;

        public int getX1() {
            return this.x1;
        }

        public Builder setX1(int i) {
            this.x1 = i;
            return this;
        }

        public int getY1() {
            return this.y1;
        }

        public Builder setY1(int i) {
            this.y1 = i;
            return this;
        }

        public int getX2() {
            return this.x2;
        }

        public Builder setX2(int i) {
            this.x2 = i;
            return this;
        }

        public int getY2() {
            return this.y2;
        }

        public Builder setY2(int i) {
            this.y2 = i;
            return this;
        }

        public int getOriginalWidth() {
            return this.originalWidth;
        }

        public Builder setOriginalWidth(int i) {
            this.originalWidth = i;
            return this;
        }

        public int getOriginalHeight() {
            return this.originalHeight;
        }

        public Builder setOriginalHeight(int i) {
            this.originalHeight = i;
            return this;
        }

        public boolean isHorizontal() {
            return this.isHorizontal;
        }

        public Builder setHorizontal(boolean z) {
            this.isHorizontal = z;
            return this;
        }

        public int getCutCoord() {
            return this.cutCoord;
        }

        public Builder setCutCoords(int i) {
            this.cutCoord = i;
            return this;
        }

        public int getOriginalTileId() {
            return this.originalTileId;
        }

        public Builder setOriginalTileId(int i) {
            this.originalTileId = i;
            return this;
        }

        public int getChild1TileId() {
            return this.child1TileId;
        }

        public Builder setChild1TileId(int i) {
            this.child1TileId = i;
            return this;
        }

        public int getChild2TileId() {
            return this.child2TileId;
        }

        public Builder setChild2TileId(int i) {
            this.child2TileId = i;
            return this;
        }

        public Cut build() {
            return new Cut(this);
        }
    }
}
