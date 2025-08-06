package com.example.debug;

import com.example.debug.engine.CutListOptimizerService;
import com.example.debug.engine.CutListOptimizerServiceImpl;
import com.example.debug.engine.console.ConsoleCutListLogger;
import com.example.debug.engine.model.*;
import java.util.ArrayList;
import java.util.List;

/**

 */
public class Example {

    public static void main(String[] args) {
        System.out.println("=== Тест ===");
        
        try {
            // Инициализируем сервис точно как в 
            CutListOptimizerService optimizer = CutListOptimizerServiceImpl.getInstance();
            optimizer.init(8); // Фиксированно 8 потоков
            optimizer.setCutListLogger(new ConsoleCutListLogger());
            
            // Настройки
            optimizer.setAllowMultipleTasksPerClient(true); // Множественные задачи
            
            // Создаем запрос 
            CalculationRequest request = createRequest();
            
            System.out.println("Отправляем задачу с -настройками...");
            
            // Отправляем задачу
            CalculationSubmissionResult result = optimizer.submitTask(request);
            
            if (result != null && result.getTaskId() != null) {
                System.out.println("Задача принята. ID: " + result.getTaskId());
                
                // Ждем выполнения
                waitForCompletion(optimizer, result.getTaskId());
                
            } else {
                System.err.println("Ошибка отправки задачи: " + 
                    (result != null ? result.getStatusCode() : "null result"));
            }
            
        } catch (Exception e) {
            System.err.println("Ошибка при выполнении теста: " + e.getMessage());
            e.printStackTrace();
        }
    }
    
    private static CalculationRequest createRequest() {
        CalculationRequest request = new CalculationRequest();
        
        // Создаем информацию о клиенте
        ClientInfo clientInfo = new ClientInfo();
        clientInfo.setId("client");
        clientInfo.setDevice("Desktop"); // Эмулируем устройство
        clientInfo.setDeviceId("test-device-id");
        request.setClientInfo(clientInfo);
        
        // Создаем панели для раскроя (исходные 5 панелей)
        List<CalculationRequest.Panel> panels = new ArrayList<>();
        
        // Панель 1: 45x55
        CalculationRequest.Panel panel1 = new CalculationRequest.Panel();
        panel1.setId(5);
        panel1.setWidth("45");
        panel1.setHeight("55");
        panel1.setCount(1);
        panel1.setMaterial("DEFAULT_MATERIAL");
        panel1.setEnabled(true);
        panel1.setOrientation(0);
        panel1.setLabel("101");
        panels.add(panel1);
        
        // Панель 2: 25x35
        CalculationRequest.Panel panel2 = new CalculationRequest.Panel();
        panel2.setId(1);
        panel2.setWidth("35");
        panel2.setHeight("25");
        panel2.setCount(1);
        panel2.setMaterial("DEFAULT_MATERIAL");
        panel2.setEnabled(true);
        panel2.setOrientation(0);
        panel2.setLabel("102");
        panels.add(panel2);
        
        // Панель 3: 15x25
        CalculationRequest.Panel panel3 = new CalculationRequest.Panel();
        panel3.setId(3);
        panel3.setWidth("10");
        panel3.setHeight("25");
        panel3.setCount(4);
        panel3.setMaterial("DEFAULT_MATERIAL");
        panel3.setEnabled(true);
        panel3.setOrientation(0);
        panel3.setLabel("103");
        panels.add(panel3);
        
        // Панель 4: 20x15
        CalculationRequest.Panel panel4 = new CalculationRequest.Panel();
        panel4.setId(6);
        panel4.setWidth("20");
        panel4.setHeight("15");
        panel4.setCount(1);
        panel4.setMaterial("DEFAULT_MATERIAL");
        panel4.setEnabled(true);
        panel4.setOrientation(0);
        panel4.setLabel("104");
        panels.add(panel4);
        
        // Панель 5: 30x40
        CalculationRequest.Panel panel5 = new CalculationRequest.Panel();
        panel5.setId(7);
        panel5.setWidth("30");
        panel5.setHeight("40");
        panel5.setCount(1);
        panel5.setMaterial("DEFAULT_MATERIAL");
        panel5.setEnabled(true);
        panel5.setOrientation(0);
        panel5.setLabel("105");
        panels.add(panel5);
        
        // Панель 6: 30x40
        CalculationRequest.Panel panel6 = new CalculationRequest.Panel();
        panel6.setId(8);
        panel6.setWidth("60");
        panel6.setHeight("40");
        panel6.setCount(1);
        panel6.setMaterial("DEFAULT_MATERIAL");
        panel6.setEnabled(true);
        panel6.setOrientation(0);
        panel6.setLabel("106");
        panels.add(panel6);



        request.setPanels(panels);
        
        // Создаем исходные листы материала
        List<CalculationRequest.Panel> stockPanels = new ArrayList<>();
        
        CalculationRequest.Panel stockPanel = new CalculationRequest.Panel();
        stockPanel.setId(1001);
        stockPanel.setWidth("80");
        stockPanel.setHeight("60");
        stockPanel.setCount(1);
        stockPanel.setMaterial("DEFAULT_MATERIAL");
        stockPanel.setEnabled(true);
        stockPanel.setOrientation(0);
        stockPanel.setLabel("90");
        stockPanels.add(stockPanel);
        
        request.setStockPanels(stockPanels);
        
        // Настройки 
        Configuration config = new Configuration();
        config.setCutThickness("0"); // Точная толщина реза
        config.setUseSingleStockUnit(false); // Разрешаем использовать разные листы
        config.setOptimizationFactor(1.0); // МАКСИМАЛЬНЫЙ фактор оптимизации = 2
        config.setOptimizationPriority(0); // Приоритет
        config.setCutOrientationPreference(0); // Все направления резов
        config.setConsiderOrientation(false); // Учитываем ориентацию волокон
        config.setMinTrimDimension("8"); // Разумный минимальный отход
        

        PerformanceThresholds thresholds = new PerformanceThresholds();
        int maxThreads = Runtime.getRuntime().availableProcessors() * 2;
        thresholds.setMaxSimultaneousThreads(maxThreads); // Максимум потоков
        thresholds.setThreadCheckInterval(1000L); // Реже проверяем, больше вычисляем
        thresholds.setMaxSimultaneousTasks(1);        config.setPerformanceThresholds(thresholds);
        config.setPerformanceThresholds(thresholds);
        request.setConfiguration(config);
        
        System.out.println("Создан запрос с настройками:");
        System.out.println("- Панели для раскроя: " + panels.size());
        System.out.println("- Исходный лист: " + stockPanel.getWidth() + "x" + stockPanel.getHeight());
        System.out.println("- Потоки сервиса: 8 (фиксированно)");
        System.out.println("- Макс. потоков на задачу: 8");
        System.out.println("- Интервал проверки: 500мс");
        System.out.println("- Фактор оптимизации: 1.0 (стандартный)");
        System.out.println("- Множественные задачи: разрешены");
        
        return request;
    }
    
    private static void waitForCompletion(CutListOptimizerService optimizer, String taskId) {
        System.out.println("Ожидание завершения задачи...");
        
        int maxAttempts = 1800; // 5 минут
        int attempts = 0;
        int lastProgress = -1;
        long startTime = System.currentTimeMillis();
        
        while (attempts < maxAttempts) {
            try {
                Thread.sleep(100); 
                attempts++;
                
                TaskStatusResponse status = optimizer.getTaskStatus(taskId);
                if (status == null) {
                    System.err.println("Не удается получить статус задачи");
                    break;
                }
                
                // Показываем прогресс при его изменении или каждые 30 секунд
                int currentProgress = status.getPercentageDone();
                if (currentProgress != lastProgress || attempts % 30 == 0) {
                    long elapsedSeconds = (System.currentTimeMillis() - startTime) / 1000;
                    System.out.println("Статус: " + status.getStatus() + 
                        ", прогресс: " + currentProgress + 
                        "%, время: " + elapsedSeconds + "с");
                    lastProgress = currentProgress;
                }
                
                if ("FINISHED".equals(status.getStatus())) {
                    long totalSeconds = (System.currentTimeMillis() - startTime) / 1000;
                    System.out.println("\n=== Задача выполнена успешно за " + totalSeconds + " секунд! ===");
                    printSolution(status.getSolution());
                    generateHtmlVisualization2(status.getSolution());
                    break;
                } else if ("ERROR".equals(status.getStatus()) || 
                          "TERMINATED".equals(status.getStatus()) ||
                          "STOPPED".equals(status.getStatus())) {
                    System.err.println("Задача завершена с ошибкой: " + status.getStatus());
                    if (status.getSolution() != null) {
                        System.out.println("Частичное решение:");
                        printSolution(status.getSolution());
                    }
                    break;
                }
                                // Если прогресс достиг 100%, но статус еще не FINISHED
                if (status.getPercentageDone() >= 100 && !"FINISHED".equals(status.getStatus())) {
                    System.out.println("Достигнут 100% прогресс, ждем финализации...");
                }
                // Показываем статистику каждые 60 секунд
                if (attempts % 60 == 0) {
                    try {
                        Stats stats = optimizer.getStats();
                        System.out.println("=== статистика ===");
                        System.out.println("Потоки: активных=" + stats.getNbrRunningThreads() + 
                            ", в очереди=" + stats.getNbrQueuedThreads() +
                            ", выполнено=" + stats.getNbrFinishedThreads());
                        System.out.println("Задачи: выполняется=" + stats.getNbrRunningTasks() + 
                            ", завершено=" + stats.getNbrFinishedTasks());
                        System.out.println("===============================");
                    } catch (Exception e) {
                        // Игнорируем ошибки статистики
                    }
                }
                
            } catch (InterruptedException e) {
                System.err.println("Прервано ожидание: " + e.getMessage());
                break;
            }
        }
        
        if (attempts >= maxAttempts) {
            System.err.println("Превышено время ожидания выполнения задачи");
            // Попытаемся получить частичное решение
            try {
                TaskStatusResponse status = optimizer.getTaskStatus(taskId);
                if (status != null && status.getSolution() != null) {
                    System.out.println("Получаем частичное решение:");
                    printSolution(status.getSolution());
                }
            } catch (Exception e) {
                System.err.println("Ошибка получения частичного решения: " + e.getMessage());
            }
        }
    }
    
    private static void printSolution(CalculationResponse solution) {
        if (solution == null) {
            System.out.println("Решение не найдено");
            return;
        }
        
        System.out.println("\n=== Результат оптимизации ===");
        System.out.printf("Общая использованная площадь: %.2f\n", solution.getTotalUsedArea());
        System.out.printf("Общая потерянная площадь: %.2f\n", solution.getTotalWastedArea());
        System.out.printf("Коэффициент использования: %.2f%%\n", 
            solution.getTotalUsedAreaRatio() * 100);
        System.out.println("Количество резов: " + solution.getTotalNbrCuts());
        System.out.printf("Общая длина резов: %.2f\n", solution.getTotalCutLength());
        System.out.println("Время выполнения: " + solution.getElapsedTime() + " мс");
        
        System.out.println("\n=== Мозаики (листы с раскроем) ===");
        if (solution.getMosaics() != null) {
            for (int i = 0; i < solution.getMosaics().size(); i++) {
                CalculationResponse.Mosaic mosaic = solution.getMosaics().get(i);
                System.out.println("\nЛист " + (i + 1) + ":");
                System.out.printf("  Использованная площадь: %.2f\n", mosaic.getUsedArea());
                System.out.printf("  Потерянная площадь: %.2f\n", mosaic.getWastedArea());
                System.out.printf("  Коэффициент использования: %.2f%%\n", 
                    mosaic.getUsedAreaRatio() * 100);
                System.out.println("  Количество деталей: " + mosaic.getNbrFinalPanels());
                System.out.println("  Количество резов: " + mosaic.getCuts().size());
                
                if (mosaic.getPanels() != null) {
                    System.out.println("  Детали:");
                    for (CalculationResponse.FinalTile panel : mosaic.getPanels()) {
                        System.out.printf("    %.1fx%.1f (количество: %d) %s\n",
                            panel.getWidth(), panel.getHeight(), panel.getCount(),
                            panel.getLabel() != null ? "[" + panel.getLabel() + "]" : "");
                    }
                }
            }
        }
        
        System.out.println("\n=== Неразмещенные панели ===");
        if (solution.getNoFitPanels() != null && !solution.getNoFitPanels().isEmpty()) {
            for (CalculationResponse.NoFitTile noFit : solution.getNoFitPanels()) {
                System.out.printf("  %.1fx%.1f (количество: %d) %s\n",
                    noFit.getWidth(), noFit.getHeight(), noFit.getCount(),
                    noFit.getLabel() != null ? "[" + noFit.getLabel() + "]" : "");
            }
        } else {
            System.out.println("  Все панели размещены успешно!");
        }}


    private static void generateHtmlVisualization2(CalculationResponse solution) {
        if (solution == null || solution.getMosaics() == null) {
            System.out.println("Нет данных для визуализации");
            return;
        }
        
        try {
            StringBuilder html = new StringBuilder();
            html.append("<!DOCTYPE html>\n")
                .append("<html>\n")
                .append("<head>\n")
                .append("    <meta charset='UTF-8'>\n")
                .append("    <title>Результат раскроя</title>\n")
                .append("    <style>\n")
                .append("        body { font-family: Arial, sans-serif; margin: 20px; }\n")
                .append("        .mosaic { border: 2px solid #000; margin: 20px 0; position: relative; display: inline-block; }\n")
                .append("        .panel { position: absolute; border: 1px solid #333; text-align: center; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; }\n")
                .append("        .info { margin: 10px 0; }\n")
                .append("        .cuts { position: absolute; background: #ff0000; }\n")
                .append("        .cut-h { height: 1px; }\n")
                .append("        .cut-v { width: 1px; }\n")
                .append("        h2 { color: #333; }\n")
                .append("        .stats { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }\n")
                .append("    </style>\n")
                .append("</head>\n")
                .append("<body>\n")
                .append("    <h1>Результат оптимизации раскроя</h1>\n");
            
            // Общая статистика
            html.append("    <div class='stats'>\n")
                .append("        <h3>Общая статистика:</h3>\n")
                .append("        <p>Общая использованная площадь: ").append(String.format("%.2f", solution.getTotalUsedArea())).append("</p>\n")
                .append("        <p>Общая потерянная площадь: ").append(String.format("%.2f", solution.getTotalWastedArea())).append("</p>\n")
                .append("        <p>Коэффициент использования: ").append(String.format("%.2f%%", solution.getTotalUsedAreaRatio() * 100)).append("</p>\n")
                .append("        <p>Количество резов: ").append(solution.getTotalNbrCuts()).append("</p>\n")
                .append("        <p>Время выполнения: ").append(solution.getElapsedTime()).append(" мс</p>\n")
                .append("    </div>\n");
            
            // Масштаб для визуализации (1 мм = 2 пикселя)
            double scale = 2.0;
            
            for (int i = 0; i < solution.getMosaics().size(); i++) {
                CalculationResponse.Mosaic mosaic = solution.getMosaics().get(i);
                
                html.append("    <h2>Лист ").append(i + 1).append("</h2>\n");
                html.append("    <div class='info'>\n")
                    .append("        Использованная площадь: ").append(String.format("%.2f", mosaic.getUsedArea()))
                    .append(", Потери: ").append(String.format("%.2f", mosaic.getWastedArea()))
                    .append(" (").append(String.format("%.1f%%", mosaic.getUsedAreaRatio() * 100)).append(" использования)\n")
                    .append("    </div>\n");
                
                if (mosaic.getTiles() != null && !mosaic.getTiles().isEmpty()) {
                    // Находим размеры листа (корневая плитка)
                    CalculationResponse.Tile rootTile = null;
                    for (CalculationResponse.Tile tile : mosaic.getTiles()) {
                        if (!tile.isHasChildren() && !tile.isFinal()) {
                            continue; // Пропускаем отходы
                        }
                        if (rootTile == null || (tile.getX() == 0 && tile.getY() == 0 && 
                            tile.getWidth() > rootTile.getWidth() && tile.getHeight() > rootTile.getHeight())) {
                            rootTile = tile;
                        }
                    }
                    
                    if (rootTile == null) {
                        // Если не нашли корневую, берем первую плитку и находим максимальные размеры
                        double maxX = 0, maxY = 0;
                        for (CalculationResponse.Tile tile : mosaic.getTiles()) {
                            maxX = Math.max(maxX, tile.getX() + tile.getWidth());
                            maxY = Math.max(maxY, tile.getY() + tile.getHeight());
                        }
                        html.append("    <div class='mosaic' style='width: ").append((int)(maxX * scale))
                            .append("px; height: ").append((int)(maxY * scale)).append("px;'>\n");
                    } else {
                        html.append("    <div class='mosaic' style='width: ").append((int)(rootTile.getWidth() * scale))
                            .append("px; height: ").append((int)(rootTile.getHeight() * scale)).append("px;'>\n");
                    }
                    
                    // Генерируем случайные цвета для панелей
                    String[] colors = {"#FFB6C1", "#87CEEB", "#98FB98", "#F0E68C", "#DDA0DD", "#FFA07A", "#B0E0E6", "#FFEFD5"};
                    int colorIndex = 0;
                    
                    // Отображаем финальные панели
                    for (CalculationResponse.Tile tile : mosaic.getTiles()) {
                        if (tile.isFinal()) {
                            String color = colors[colorIndex % colors.length];
                            colorIndex++;
                            
                            html.append("        <div class='panel' style='")
                                .append("left: ").append((int)(tile.getX() * scale)).append("px; ")
                                .append("top: ").append((int)(tile.getY() * scale)).append("px; ")
                                .append("width: ").append((int)(tile.getWidth() * scale)).append("px; ")
                                .append("height: ").append((int)(tile.getHeight() * scale)).append("px; ")
                                .append("background-color: ").append(color).append(";'>\n")
                                .append("            ").append(String.format("%.0fx%.0f", tile.getWidth(), tile.getHeight()));
                            
                            if (tile.getLabel() != null && !tile.getLabel().isEmpty()) {
                                html.append("<br>").append(tile.getLabel());
                            }
                            
                            html.append("\n        </div>\n");
                        }
                    }
                    
                    // Отображаем резы
                    if (mosaic.getCuts() != null) {
                        for (CalculationResponse.Cut cut : mosaic.getCuts()) {
                            if (cut.isHorizontal()) {
                                // Горизонтальный рез
                                html.append("        <div class='cuts cut-h' style='")
                                    .append("left: ").append((int)(cut.getX1() * scale)).append("px; ")
                                    .append("top: ").append((int)(cut.getY1() * scale)).append("px; ")
                                    .append("width: ").append((int)((cut.getX2() - cut.getX1()) * scale)).append("px;'></div>\n");
                            } else {
                                // Вертикальный рез
                                html.append("        <div class='cuts cut-v' style='")
                                    .append("left: ").append((int)(cut.getX1() * scale)).append("px; ")
                                    .append("top: ").append((int)(cut.getY1() * scale)).append("px; ")
                                    .append("height: ").append((int)((cut.getY2() - cut.getY1()) * scale)).append("px;'></div>\n");
                            }
                        }
                    }
                    
                    html.append("    </div>\n");
                }
                
                // Список панелей в этой мозаике
                if (mosaic.getPanels() != null && !mosaic.getPanels().isEmpty()) {
                    html.append("    <div class='info'>\n")
                        .append("        <strong>Детали в листе:</strong><br>\n");
                    for (CalculationResponse.FinalTile panel : mosaic.getPanels()) {
                        html.append("        • ").append(String.format("%.0fx%.0f", panel.getWidth(), panel.getHeight()))
                            .append(" (кол-во: ").append(panel.getCount()).append(")");
                        if (panel.getLabel() != null && !panel.getLabel().isEmpty()) {
                            html.append(" [").append(panel.getLabel()).append("]");
                        }
                        html.append("<br>\n");
                    }
                    html.append("    </div>\n");
                }
            }
            
            // Неразмещенные панели
            if (solution.getNoFitPanels() != null && !solution.getNoFitPanels().isEmpty()) {
                html.append("    <div class='stats'>\n")
                    .append("        <h3 style='color: #d00;'>Неразмещенные панели:</h3>\n");
                for (CalculationResponse.NoFitTile noFit : solution.getNoFitPanels()) {
                    html.append("        • ").append(String.format("%.0fx%.0f", noFit.getWidth(), noFit.getHeight()))
                        .append(" (кол-во: ").append(noFit.getCount()).append(")");
                    if (noFit.getLabel() != null && !noFit.getLabel().isEmpty()) {
                        html.append(" [").append(noFit.getLabel()).append("]");
                    }
                    html.append("<br>\n");
                }
                html.append("    </div>\n");
            }
            
            html.append("    <div class='info'>\n")
                .append("        <small>Масштаб: 1 мм = 2 пикселя. Красные линии - резы.</small>\n")
                .append("    </div>\n")
                .append("</body>\n")
                .append("</html>");
            
            // Записываем HTML в файл
            java.io.FileWriter writer = new java.io.FileWriter("cutting_result.html");
            writer.write(html.toString());
            writer.close();
            
            System.out.println("\n=== HTML визуализация создана ===");
            System.out.println("Файл: cutting_result.html");
            System.out.println("Откройте файл в браузере для просмотра схемы раскроя");
            
        } catch (Exception e) {
            System.err.println("Ошибка создания HTML визуализации: " + e.getMessage());
            e.printStackTrace();
        }

      }
}