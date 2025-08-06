package com.example.debug.engine.model;

import com.fasterxml.jackson.annotation.JsonIgnore;
import java.util.List;

/* loaded from: classes.dex */
public class CalculationRequest {

    private Configuration configuration;
    private List<Panel> panels;
    private List<Panel> stockPanels;
    private ClientInfo clientInfo;

    public Configuration getConfiguration() {
        return this.configuration;
    }

    public void setConfiguration(Configuration configuration) {
        this.configuration = configuration;
    }

    public List<Panel> getStockPanels() {
        return this.stockPanels;
    }

    public void setStockPanels(List<Panel> list) {
        this.stockPanels = list;
    }

    public List<Panel> getPanels() {
        return this.panels;
    }

    public void setPanels(List<Panel> list) {
        this.panels = list;
    }

    public String tilesToString() {
        StringBuilder sb = new StringBuilder();
        for (Panel panel : this.panels) {
            if (panel.getCount() > 0) {
                sb.append(" " + panel.toString());
            }
        }
        return sb.toString();
    }

    public String baseTilesToString() {
        StringBuilder sb = new StringBuilder();
        for (Panel panel : this.stockPanels) {
            if (panel.getCount() > 0) {
                sb.append(" " + panel.toString());
            }
        }
        return sb.toString();
    }
    
    public ClientInfo getClientInfo() {
        return this.clientInfo;
    }
    
    public void setClientInfo(ClientInfo clientInfo) {
        this.clientInfo = clientInfo;
    }

    public static class Panel {
        private int count;
        private Edge edge;
        private boolean enabled;
        private String height;
        private int id;
        private String label;
        private String material = TileDimensions.DEFAULT_MATERIAL;
        private int orientation;
        private String width;

        public int getId() {
            return this.id;
        }

        public void setId(int i) {
            this.id = i;
        }

        public String getWidth() {
            return this.width;
        }

        public void setWidth(String str) {
            this.width = str;
        }

        public String getHeight() {
            return this.height;
        }

        public void setHeight(String str) {
            this.height = str;
        }

        public int getCount() {
            return this.count;
        }

        public void setCount(int i) {
            this.count = i;
        }

        public String getMaterial() {
            return this.material;
        }

        public void setMaterial(String str) {
            if (str != null) {
                this.material = str;
            }
        }

        public boolean isEnabled() {
            return this.enabled;
        }

        public void setEnabled(boolean z) {
            this.enabled = z;
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

        public Edge getEdge() {
            return this.edge;
        }

        public void setEdge(Edge edge) {
            this.edge = edge;
        }

        @JsonIgnore
        public boolean isValid() {
            String str;
            String str2;
            try {
                if (!this.enabled || this.count <= 0 || (str = this.width) == null || Double.parseDouble(str) <= 0.0d || (str2 = this.height) == null) {
                    return false;
                }
                return Double.parseDouble(str2) > 0.0d;
            } catch (Exception unused) {
                return false;
            }
        }

        public String toString() {
            StringBuilder sb = new StringBuilder("[");
            sb.append(this.width);
            sb.append("x");
            sb.append(this.height);
            sb.append("]*");
            sb.append(this.count);
            sb.append(this.enabled ? "" : "-disabled");
            return sb.toString();
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
}
