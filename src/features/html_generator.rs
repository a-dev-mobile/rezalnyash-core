use crate::features::{
    solution::Solution,
    placement::Placement,
};
use std::fs::File;
use std::io::Write;

/// Генератор HTML визуализации для результатов раскроя
pub struct HtmlGenerator;

impl HtmlGenerator {
    /// Генерирует HTML файл для визуализации решения раскроя
    pub fn generate_html(solution: &Solution, precision: u32, elapsed_time_ms: u128, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = Self::build_html_content(solution, precision, elapsed_time_ms);
        
        let mut file = File::create(output_path)?;
        file.write_all(html_content.as_bytes())?;
        
        println!("\n=== HTML визуализация создана ===");
        println!("Файл: {}", output_path);
        println!("Откройте файл в браузере для просмотра схемы раскроя");
        
        Ok(())
    }

    fn build_html_content(solution: &Solution, precision: u32, elapsed_time_ms: u128) -> String {
        let mut html = String::new();
        
        // HTML заголовок и стили
        html.push_str(&Self::get_html_header());
        
        // Заголовок страницы
        html.push_str("    <h1>Результат оптимизации раскроя</h1>\n");
        
        // Общая статистика
        html.push_str(&Self::generate_stats_section(solution, precision, elapsed_time_ms));
        
        // Генерируем визуализацию для каждого листа
        for (i, placement) in solution.placements.iter().enumerate() {
            html.push_str(&Self::generate_placement_section(placement, i + 1, precision, &solution.stocks));
        }
        
        // Неразмещенные панели
        if !solution.unplaced_panels.is_empty() {
            html.push_str(&Self::generate_unplaced_panels_section(&solution.unplaced_panels, precision));
        }
        
        // Масштаб информация
        html.push_str("    <div class='info'>\n");
        html.push_str("        <small>Масштаб: 1 мм = 2 пикселя. Красные линии - резы.</small>\n");
        html.push_str("    </div>\n");
        
        // Закрытие HTML
        html.push_str("</body>\n</html>");
        
        html
    }

    fn get_html_header() -> String {
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset='UTF-8'>
    <title>Результат раскроя</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .mosaic { border: 2px solid #000; margin: 20px 0; position: relative; display: inline-block; }
        .panel { position: absolute; border: 1px solid #333; text-align: center; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; }
        .info { margin: 10px 0; }
        .cuts { position: absolute; background: #ff0000; }
        .cut-h { height: 1px; }
        .cut-v { width: 1px; }
        h2 { color: #333; }
        .stats { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }
    </style>
</head>
<body>
"#.to_string()
    }

    fn generate_stats_section(solution: &Solution, precision: u32, elapsed_time_ms: u128) -> String {
        let scale_factor = 10f64.powi(precision as i32);
        let total_used_area = solution.total_used_area as f64 / (scale_factor * scale_factor);
        let total_waste_area = solution.total_waste_area as f64 / (scale_factor * scale_factor);
        let efficiency_percent = solution.total_efficiency * 100.0;
        let total_cut_length = solution.total_cut_length as f64 / scale_factor;

        format!(
            r#"    <div class='stats'>
        <h3>Общая статистика:</h3>
        <p>Общая использованная площадь: {:.2}</p>
        <p>Общая потерянная площадь: {:.2}</p>
        <p>Коэффициент использования: {:.2}%</p>
        <p>Количество резов: {}</p>
        <p>Время выполнения: {} мс</p>
    </div>
"#,
            total_used_area,
            total_waste_area,
            efficiency_percent,
            solution.total_cuts,
            elapsed_time_ms
        )
    }

    fn generate_placement_section(placement: &Placement, sheet_number: usize, precision: u32, stocks: &[crate::features::input::models::panel::Panel]) -> String {
        let scale_factor = 10f64.powi(precision as i32);
        let scale_visual = 2.0; // 1 мм = 2 пикселя
        
        let used_area = placement.used_area as f64 / (scale_factor * scale_factor);
        let waste_area = placement.waste_area as f64 / (scale_factor * scale_factor);
        let efficiency = if placement.used_area + placement.waste_area > 0 {
            placement.used_area as f64 / (placement.used_area + placement.waste_area) as f64 * 100.0
        } else {
            0.0
        };

        // Найдем заготовку по stock_id
        let stock = stocks.iter().find(|s| s.id as i32 == placement.stock_id).unwrap_or(&stocks[0]);
        let stock_width_px = (stock.width as f64 / scale_factor * scale_visual) as i32;
        let stock_height_px = (stock.height as f64 / scale_factor * scale_visual) as i32;

        let mut html = format!(
            r#"    <h2>Лист {}</h2>
    <div class='info'>
        Использованная площадь: {:.2}, Потери: {:.2} ({:.1}% использования)
    </div>
    <div class='mosaic' style='width: {}px; height: {}px;'>
"#,
            sheet_number,
            used_area,
            waste_area,
            efficiency,
            stock_width_px,
            stock_height_px
        );

        // Генерируем цвета для панелей
        let colors = ["#FFB6C1", "#87CEEB", "#98FB98", "#F0E68C", "#DDA0DD", "#FFA07A", "#B0E0E6", "#FFEFD5"];
        
        // Отображаем размещенные панели
        for (i, panel) in placement.placed_panels.iter().enumerate() {
            let color = colors[i % colors.len()];
            let x_px = (panel.x as f64 / scale_factor * scale_visual) as i32;
            let y_px = (panel.y as f64 / scale_factor * scale_visual) as i32;
            let width_px = (panel.width as f64 / scale_factor * scale_visual) as i32;
            let height_px = (panel.height as f64 / scale_factor * scale_visual) as i32;
            let width_mm = panel.width as f64 / scale_factor;
            let height_mm = panel.height as f64 / scale_factor;

            html.push_str(&format!(
                r#"        <div class='panel' style='left: {}px; top: {}px; width: {}px; height: {}px; background-color: {};'>
            {:.0}x{:.0}<br>{}
        </div>
"#,
                x_px, y_px, width_px, height_px, color,
                width_mm, height_mm, panel.label
            ));
        }

        // Отображаем резы
        for cut in &placement.cuts {
            let x1_px = (cut.x1 as f64 / scale_factor * scale_visual) as i32;
            let y1_px = (cut.y1 as f64 / scale_factor * scale_visual) as i32;
            let x2_px = (cut.x2 as f64 / scale_factor * scale_visual) as i32;
            let y2_px = (cut.y2 as f64 / scale_factor * scale_visual) as i32;

            if cut.is_horizontal {
                let width_px = x2_px - x1_px;
                html.push_str(&format!(
                    "        <div class='cuts cut-h' style='left: {}px; top: {}px; width: {}px;'></div>\n",
                    x1_px, y1_px, width_px
                ));
            } else {
                let height_px = y2_px - y1_px;
                html.push_str(&format!(
                    "        <div class='cuts cut-v' style='left: {}px; top: {}px; height: {}px;'></div>\n",
                    x1_px, y1_px, height_px
                ));
            }
        }

        html.push_str("    </div>\n");

        // Список панелей в этом листе
        if !placement.placed_panels.is_empty() {
            html.push_str("    <div class='info'>\n");
            html.push_str("        <strong>Детали в листе:</strong><br>\n");
            
            // Группируем панели по размеру и названию
            let mut panel_groups: std::collections::HashMap<String, (f64, f64, usize, String)> = std::collections::HashMap::new();
            
            for panel in &placement.placed_panels {
                let width_mm = panel.width as f64 / scale_factor;
                let height_mm = panel.height as f64 / scale_factor;
                let key = format!("{:.0}x{:.0}_{}", width_mm, height_mm, panel.label);
                
                let entry = panel_groups.entry(key).or_insert((width_mm, height_mm, 0, panel.label.clone()));
                entry.2 += 1;
            }

            for (_, (width, height, count, label)) in panel_groups {
                html.push_str(&format!(
                    "        • {:.0}x{:.0} (кол-во: {}) [{}]<br>\n",
                    width, height, count, label
                ));
            }
            
            html.push_str("    </div>\n");
        }

        html
    }

    fn generate_unplaced_panels_section(unplaced_panels: &[crate::features::input::models::panel::Panel], precision: u32) -> String {
        let scale_factor = 10f64.powi(precision as i32);
        
        let mut html = String::from("    <div class='stats'>\n");
        html.push_str("        <h3 style='color: #d00;'>Неразмещенные панели:</h3>\n");
        
        for panel in unplaced_panels {
            let width_mm = panel.width as f64 / scale_factor;
            let height_mm = panel.height as f64 / scale_factor;
            
            html.push_str(&format!(
                "        • {:.0}x{:.0} (кол-во: {}) [{}]<br>\n",
                width_mm, height_mm, panel.count, panel.label
            ));
        }
        
        html.push_str("    </div>\n");
        html
    }
}