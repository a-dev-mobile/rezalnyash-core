package com.example.debug.engine;

import com.example.debug.engine.model.CalculationRequest;
import com.example.debug.engine.model.Configuration;
import java.util.List;
import org.apache.commons.lang3.StringUtils;

/* loaded from: classes.dex */
public class CutListDataStringifier {
    public static String getTileDTOListAsCsv(List<CalculationRequest.Panel> list) {
        String string = "";
        for (CalculationRequest.Panel panel : list) {
            if ((panel.getWidth() != null && !panel.getWidth().trim().isEmpty()) || ((panel.getHeight() != null && !panel.getHeight().trim().isEmpty()) || panel.getCount() != 0 || (panel.getLabel() != null && !panel.getLabel().trim().isEmpty()))) {
                if (string.length() > 0) {
                    string = string + StringUtils.LF;
                }
                StringBuilder sb = new StringBuilder();
                sb.append(string);
                sb.append(panel.getHeight() != null ? panel.getHeight() : "");
                String str = sb.toString() + ",";
                StringBuilder sb2 = new StringBuilder();
                sb2.append(str);
                sb2.append(panel.getWidth() != null ? panel.getWidth() : "");
                String str2 = ((sb2.toString() + ",") + panel.getCount()) + ",";
                StringBuilder sb3 = new StringBuilder();
                sb3.append(str2);
                sb3.append(panel.getMaterial() != null ? panel.getMaterial() : "");
                String str3 = sb3.toString() + ",";
                StringBuilder sb4 = new StringBuilder();
                sb4.append(str3);
                sb4.append(panel.getLabel() != null ? panel.getLabel() : "");
                String str4 = ((((sb4.toString() + ",") + panel.isEnabled()) + ",") + panel.getOrientation()) + ",";
                if (panel.getEdge() != null) {
                    StringBuilder sb5 = new StringBuilder();
                    sb5.append(str4);
                    sb5.append(panel.getEdge().getTop() != null ? panel.getEdge().getTop() : "");
                    String str5 = sb5.toString() + ",";
                    StringBuilder sb6 = new StringBuilder();
                    sb6.append(str5);
                    sb6.append(panel.getEdge().getLeft() != null ? panel.getEdge().getLeft() : "");
                    String str6 = sb6.toString() + ",";
                    StringBuilder sb7 = new StringBuilder();
                    sb7.append(str6);
                    sb7.append(panel.getEdge().getBottom() != null ? panel.getEdge().getBottom() : "");
                    String str7 = sb7.toString() + ",";
                    StringBuilder sb8 = new StringBuilder();
                    sb8.append(str7);
                    sb8.append(panel.getEdge().getRight() != null ? panel.getEdge().getRight() : "");
                    string = sb8.toString();
                } else {
                    string = str4 + ",,,";
                }
            }
        }
        return string;
    }

    private static String generateConfigurationDataString(Configuration configuration) {
        String str = "cutThickness " + configuration.getCutThickness() + StringUtils.LF;
        StringBuilder sb = new StringBuilder();
        sb.append(str);
        sb.append("considerGrain ");
        sb.append(configuration.isConsiderOrientation() ? "1" : "0");
        sb.append(StringUtils.LF);
        String string = sb.toString();
        StringBuilder sb2 = new StringBuilder();
        sb2.append(string);
        sb2.append("useSingleStockUnit ");
        sb2.append(configuration.isUseSingleStockUnit() ? "1" : "0");
        sb2.append(StringUtils.LF);
        return sb2.toString() + "units " + configuration.getUnits() + StringUtils.LF;
    }

    public static String generateProjectDataString(CalculationRequest calculationRequest) {
        return ((((("panels {\n" + getTileDTOListAsCsv(calculationRequest.getPanels()) + StringUtils.LF) + "}\n") + "stock {\n") + getTileDTOListAsCsv(calculationRequest.getStockPanels()) + StringUtils.LF) + "}\n") + generateConfigurationDataString(calculationRequest.getConfiguration());
    }
}
