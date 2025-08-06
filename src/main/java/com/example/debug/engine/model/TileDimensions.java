package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class TileDimensions {
    public static String DEFAULT_MATERIAL = "DEFAULT_MATERIAL";
    protected final int height;
    protected final int id;
    protected final boolean isRotated;
    protected final String label;
    protected final String material;
    protected final int orientation;
    protected final int width;

    public TileDimensions(TileDimensions tileDimensions) {
        this.id = tileDimensions.id;
        this.width = tileDimensions.width;
        this.height = tileDimensions.height;
        this.material = tileDimensions.material;
        this.orientation = tileDimensions.orientation;
        this.label = tileDimensions.label;
        this.isRotated = tileDimensions.isRotated;
    }

    public TileDimensions(int i, int i2, int i3, String str, int i4, String str2) {
        this.id = i;
        this.width = i2;
        this.height = i3;
        this.material = str;
        this.orientation = i4;
        this.label = str2;
        this.isRotated = false;
    }

    public TileDimensions(int i, int i2, int i3, String str, int i4, String str2, boolean z) {
        this.id = i;
        this.width = i2;
        this.height = i3;
        this.material = str;
        this.orientation = i4;
        this.label = str2;
        this.isRotated = z;
    }

    public TileDimensions(int i, int i2) {
        this.id = -1;
        this.width = i;
        this.height = i2;
        this.material = DEFAULT_MATERIAL;
        this.orientation = 0;
        this.label = null;
        this.isRotated = false;
    }

    public int getId() {
        return this.id;
    }

    public int getWidth() {
        return this.width;
    }

    public int getHeight() {
        return this.height;
    }

    public String getMaterial() {
        return this.material;
    }

    public int getOrientation() {
        return this.orientation;
    }

    public boolean isRotated() {
        return this.isRotated;
    }

    public int getMaxDimension() {
        return Math.max(this.width, this.height);
    }

    public long getArea() {
        return this.width * this.height;
    }

    public TileDimensions rotate90() {
        return new TileDimensions(this.id, this.height, this.width, this.material, this.orientation == 1 ? 2 : 1, this.label, true);
    }

    public boolean isSquare() {
        return this.width == this.height;
    }

    public boolean isHorizontal() {
        return this.width > this.height;
    }

    public String toString() {
        return "id=" + this.id + "[" + this.width + "x" + this.height + ']';
    }

    public String dimensionsToString() {
        return this.width + "x" + this.height;
    }

    public boolean hasSameDimensions(TileDimensions tileDimensions) {
        int i = this.width;
        int i2 = tileDimensions.width;
        if (i == i2 && this.height == tileDimensions.height) {
            return true;
        }
        return i == tileDimensions.height && this.height == i2;
    }

    public boolean fits(TileDimensions tileDimensions) {
        int i = this.width;
        int i2 = tileDimensions.width;
        return (i >= i2 && this.height >= tileDimensions.height) || (this.height >= i2 && i >= tileDimensions.height);
    }

    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()) {
            return false;
        }
        TileDimensions tileDimensions = (TileDimensions) obj;
        return this.id == tileDimensions.id && this.width == tileDimensions.width && this.height == tileDimensions.height;
    }

    public int hashCode() {
        return (((this.id * 31) + this.width) * 31) + this.height;
    }

    public int dimensionsBasedHashCode() {
        return (this.width * 31) + this.height;
    }
}
