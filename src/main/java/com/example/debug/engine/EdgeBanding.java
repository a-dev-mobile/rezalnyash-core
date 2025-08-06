package com.example.debug.engine;

import com.example.debug.engine.model.CalculationRequest;
import com.example.debug.engine.model.TileNode;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/* loaded from: classes.dex */
public class EdgeBanding {
    public static Map<String, Double> calcEdgeBands(List<TileNode> list, List<CalculationRequest.Panel> list2, double d) {
        int height;
        int height2;
        HashMap map = new HashMap();
        for (CalculationRequest.Panel panel : list2) {
            if (panel.getEdge() != null) {
                for (TileNode tileNode : list) {
                    if (tileNode.getExternalId() == panel.getId()) {
                        if (tileNode.isRotated()) {
                            height = tileNode.getWidth();
                            height2 = tileNode.getHeight();
                        } else {
                            int width = tileNode.getWidth();
                            height = tileNode.getHeight();
                            height2 = width;
                        }
                        String top = panel.getEdge().getTop();
                        String left = panel.getEdge().getLeft();
                        String bottom = panel.getEdge().getBottom();
                        String right = panel.getEdge().getRight();
                        if (top != null) {
                            map.put(top, Double.valueOf((map.get(top) != null ? ((Double) map.get(top)).doubleValue() : 0.0d) + (height2 / d)));
                        }
                        if (left != null) {
                            map.put(left, Double.valueOf((map.get(left) != null ? ((Double) map.get(left)).doubleValue() : 0.0d) + (height / d)));
                        }
                        if (bottom != null) {
                            map.put(bottom, Double.valueOf((map.get(bottom) != null ? ((Double) map.get(bottom)).doubleValue() : 0.0d) + (height2 / d)));
                        }
                        if (right != null) {
                            map.put(right, Double.valueOf((map.get(right) != null ? ((Double) map.get(right)).doubleValue() : 0.0d) + (height / d)));
                        }
                    }
                }
            }
        }
        return map;
    }
}
