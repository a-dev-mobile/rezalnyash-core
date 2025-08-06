package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class GroupedTileDimensions extends TileDimensions {
    private final int group;

    public GroupedTileDimensions(GroupedTileDimensions groupedTileDimensions) {
        super(groupedTileDimensions.width, groupedTileDimensions.height);
        this.group = groupedTileDimensions.group;
    }

    public GroupedTileDimensions(TileDimensions tileDimensions, int i) {
        super(tileDimensions);
        this.group = i;
    }

    public GroupedTileDimensions(int i, int i2, int i3) {
        super(i, i2);
        this.group = i3;
    }

    public int getGroup() {
        return this.group;
    }

    @Override // com.cutlistoptimizer.engine.model.TileDimensions
    public String toString() {
        return "id=" + this.id + ", gropup=" + this.group + "[" + this.width + "x" + this.height + ']';
    }

    @Override // com.cutlistoptimizer.engine.model.TileDimensions
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        return obj != null && getClass() == obj.getClass() && super.equals(obj) && this.group == ((GroupedTileDimensions) obj).group;
    }

    @Override // com.cutlistoptimizer.engine.model.TileDimensions
    public int hashCode() {
        return (super.hashCode() * 31) + this.group;
    }
}
