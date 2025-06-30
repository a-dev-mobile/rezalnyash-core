use std::fs::File;
use std::io::Write;
use crate::models::{
    calculation_response::CalculationResponse,
    mosaic::Mosaic,
    final_tile::FinalTile,
    no_fit_tile::NoFitTile,
    tile::Tile,
    cut::Cut,
};

/// HTML visualization generator for cutting optimization results
pub struct HtmlVisualizer;

impl HtmlVisualizer {
    /// Generate HTML visualization file for the cutting solution
    /// 
    /// # Arguments
    /// * `solution` - The calculation response containing mosaics and results
    /// * `filename` - Optional filename (default: "cutting_result.html")
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Success or error
    pub fn generate_html_visualization(
        solution: &CalculationResponse,
        filename: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filename = filename.unwrap_or("cutting_result.html");
        
        if solution.mosaics.is_empty() {
            println!("Нет данных для визуализации");
            return Ok(());
        }

        let mut html = String::new();
        
        // HTML header
        Self::append_html_header(&mut html);
        
        // Overall statistics
        Self::append_overall_statistics(&mut html, solution);
        
        // Scale for visualization (1 mm = 2 pixels, like in Java)
        let scale = 2.0;
        
        // Generate visualization for each mosaic
        for (i, mosaic) in solution.mosaics.iter().enumerate() {
            Self::append_mosaic_visualization(&mut html, mosaic, i + 1, scale);
        }
        
        // Unplaced panels section
        Self::append_unplaced_panels(&mut html, &solution.no_fit_panels);
        
        // Footer
        Self::append_html_footer(&mut html);
        
        // Write to file
        let mut file = File::create(filename)?;
        file.write_all(html.as_bytes())?;
        
        println!("\n=== HTML визуализация создана ===");
        println!("Файл: {}", filename);
        println!("Откройте файл в браузере для просмотра схемы раскроя");
        
        Ok(())
    }
    
    /// Append HTML header and CSS styles
    fn append_html_header(html: &mut String) {
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset='UTF-8'>\n");
        html.push_str("    <title>Результат раскроя</title>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("        .mosaic { border: 2px solid #000; margin: 20px 0; position: relative; display: inline-block; }\n");
        html.push_str("        .panel { position: absolute; border: 1px solid #333; text-align: center; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; }\n");
        html.push_str("        .info { margin: 10px 0; }\n");
        html.push_str("        .cuts { position: absolute; background: #ff0000; }\n");
        html.push_str("        .cut-h { height: 1px; }\n");
        html.push_str("        .cut-v { width: 1px; }\n");
        html.push_str("        h2 { color: #333; }\n");
        html.push_str("        .stats { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <h1>Результат оптимизации раскроя</h1>\n");
    }
    
    /// Append overall statistics section
    fn append_overall_statistics(html: &mut String, solution: &CalculationResponse) {
        html.push_str("    <div class='stats'>\n");
        html.push_str("        <h3>Общая статистика:</h3>\n");
        html.push_str(&format!("        <p>Общая использованная площадь: {:.2}</p>\n", solution.total_used_area));
        html.push_str(&format!("        <p>Общая потерянная площадь: {:.2}</p>\n", solution.total_wasted_area));
        html.push_str(&format!("        <p>Коэффициент использования: {:.2}%</p>\n", solution.total_used_area_ratio * 100.0));
        html.push_str(&format!("        <p>Количество резов: {}</p>\n", solution.total_nbr_cuts));
        html.push_str(&format!("        <p>Общая длина резов: {:.2}</p>\n", solution.total_cut_length));
        html.push_str(&format!("        <p>Время выполнения: {} мс</p>\n", solution.elapsed_time));
        html.push_str("    </div>\n");
    }
    
    /// Append visualization for a single mosaic
    fn append_mosaic_visualization(html: &mut String, mosaic: &Mosaic, sheet_number: usize, scale: f64) {
        // Calculate mosaic statistics
        let mut mosaic_copy = mosaic.clone();
        let used_area = mosaic_copy.used_area() as f64;
        let unused_area = mosaic_copy.unused_area() as f64;
        let total_area = used_area + unused_area;
        let usage_ratio = if total_area > 0.0 { used_area / total_area * 100.0 } else { 0.0 };
        
        html.push_str(&format!("    <h2>Лист {}</h2>\n", sheet_number));
        html.push_str("    <div class='info'>\n");
        html.push_str(&format!("        Использованная площадь: {:.2}, Потери: {:.2} ({:.1}% использования)\n", 
                              used_area, unused_area, usage_ratio));
        html.push_str("    </div>\n");
        
        // Get mosaic dimensions
        let mosaic_width = mosaic.width() as f64;
        let mosaic_height = mosaic.height() as f64;
        
        html.push_str(&format!("    <div class='mosaic' style='width: {}px; height: {}px;'>\n", 
                              (mosaic_width * scale) as i32, (mosaic_height * scale) as i32));
        
        // Color palette for panels
        let colors = ["#FFB6C1", "#87CEEB", "#98FB98", "#F0E68C", "#DDA0DD", "#FFA07A", "#B0E0E6", "#FFEFD5"];
        let mut color_index = 0;
        
        // Draw final tiles (panels)
        for tile_node in mosaic.final_tile_nodes() {
            let color = colors[color_index % colors.len()];
            color_index += 1;
            
            let x = tile_node.x1() as f64;
            let y = tile_node.y1() as f64;
            let width = tile_node.width() as f64;
            let height = tile_node.height() as f64;
            
            html.push_str(&format!("        <div class='panel' style='"));
            html.push_str(&format!("left: {}px; ", (x * scale) as i32));
            html.push_str(&format!("top: {}px; ", (y * scale) as i32));
            html.push_str(&format!("width: {}px; ", (width * scale) as i32));
            html.push_str(&format!("height: {}px; ", (height * scale) as i32));
            html.push_str(&format!("background-color: {};'>\n", color));
            html.push_str(&format!("            {:.0}x{:.0}", width, height));
            
            // Add label if available
            if let Some(label) = Self::get_tile_label(tile_node) {
                html.push_str(&format!("<br>{}", label));
            }
            
            html.push_str("\n        </div>\n");
        }
        
        // Draw cuts
        for cut in &mosaic.cuts {
            Self::append_cut_visualization(html, cut, scale);
        }
        
        html.push_str("    </div>\n");
        
        // List panels in this mosaic
        Self::append_mosaic_panel_list(html, mosaic);
    }
    
    /// Append cut visualization
    fn append_cut_visualization(html: &mut String, cut: &Cut, scale: f64) {
        if cut.is_horizontal() {
            // Horizontal cut
            html.push_str(&format!("        <div class='cuts cut-h' style='"));
            html.push_str(&format!("left: {}px; ", (cut.x1() as f64 * scale) as i32));
            html.push_str(&format!("top: {}px; ", (cut.y1() as f64 * scale) as i32));
            html.push_str(&format!("width: {}px;'></div>\n", ((cut.x2() - cut.x1()) as f64 * scale) as i32));
        } else {
            // Vertical cut
            html.push_str(&format!("        <div class='cuts cut-v' style='"));
            html.push_str(&format!("left: {}px; ", (cut.x1() as f64 * scale) as i32));
            html.push_str(&format!("top: {}px; ", (cut.y1() as f64 * scale) as i32));
            html.push_str(&format!("height: {}px;'></div>\n", ((cut.y2() - cut.y1()) as f64 * scale) as i32));
        }
    }
    
    /// Append panel list for a mosaic
    fn append_mosaic_panel_list(html: &mut String, mosaic: &Mosaic) {
        let final_tiles = mosaic.final_tile_nodes();
        if !final_tiles.is_empty() {
            html.push_str("    <div class='info'>\n");
            html.push_str("        <strong>Детали в листе:</strong><br>\n");
            
            for tile_node in final_tiles {
                let width = tile_node.width() as f64;
                let height = tile_node.height() as f64;
                
                html.push_str(&format!("        • {:.0}x{:.0}", width, height));
                
                if let Some(label) = Self::get_tile_label(tile_node) {
                    html.push_str(&format!(" [{}]", label));
                }
                
                html.push_str("<br>\n");
            }
            
            html.push_str("    </div>\n");
        }
    }
    
    /// Append unplaced panels section
    fn append_unplaced_panels(html: &mut String, no_fit_panels: &[NoFitTile]) {
        if !no_fit_panels.is_empty() {
            html.push_str("    <div class='stats'>\n");
            html.push_str("        <h3 style='color: #d00;'>Неразмещенные панели:</h3>\n");
            
            for no_fit in no_fit_panels {
                html.push_str(&format!("        • {:.0}x{:.0} (кол-во: {})", 
                                      no_fit.width, no_fit.height, no_fit.count));
                
                if let Some(ref label) = no_fit.label {
                    html.push_str(&format!(" [{}]", label));
                }
                
                html.push_str("<br>\n");
            }
            
            html.push_str("    </div>\n");
        }
    }
    
    /// Append HTML footer
    fn append_html_footer(html: &mut String) {
        html.push_str("    <div class='info'>\n");
        html.push_str("        <small>Масштаб: 1 мм = 2 пикселя. Красные линии - резы.</small>\n");
        html.push_str("    </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>");
    }
    
    /// Get tile label (placeholder - implement based on your TileNode structure)
    fn get_tile_label(tile_node: &crate::models::tile_node::TileNode) -> Option<String> {
        // This method should be implemented based on your TileNode structure
        // For now, returning None as placeholder
        // You might want to add a label field to TileNode or derive it from external_id
        None
    }
}

// Additional trait implementations for better integration
impl CalculationResponse {
    /// Print solution statistics to console
    pub fn print_solution(&self) {
        if self.mosaics.is_empty() {
            println!("Решение не найдено");
            return;
        }
        
        println!("\n=== Результат оптимизации ===");
        println!("Общая использованная площадь: {:.2}", self.total_used_area);
        println!("Общая потерянная площадь: {:.2}", self.total_wasted_area);
        println!("Коэффициент использования: {:.2}%", self.total_used_area_ratio * 100.0);
        println!("Количество резов: {}", self.total_nbr_cuts);
        println!("Общая длина резов: {:.2}", self.total_cut_length);
        println!("Время выполнения: {} мс", self.elapsed_time);
        
        println!("\n=== Мозаики (листы с раскроем) ===");
        for (i, mosaic) in self.mosaics.iter().enumerate() {
            let mut mosaic_copy = mosaic.clone();
            let used_area = mosaic_copy.used_area() as f64;
            let unused_area = mosaic_copy.unused_area() as f64;
            let total_area = used_area + unused_area;
            let usage_ratio = if total_area > 0.0 { used_area / total_area * 100.0 } else { 0.0 };
            
            println!("Лист {}:", i + 1);
            println!("  Использование: {:.2}% ({:.2}/{:.2})", usage_ratio, used_area, total_area);
            
            for tile_node in mosaic.final_tile_nodes() {
                println!("    {:.1}x{:.1}", tile_node.width(), tile_node.height());
            }
        }
        
        if !self.no_fit_panels.is_empty() {
            println!("\n=== Неразмещенные детали ===");
            for no_fit in &self.no_fit_panels {
                println!("  {:.1}x{:.1} x{}", no_fit.width, no_fit.height, no_fit.count);
                if let Some(ref label) = no_fit.label {
                    println!(" [{}]", label);
                }
            }
        } else {
            println!("\n=== Все детали размещены успешно! ===");
        }
    }
    
    /// Generate HTML visualization with default filename
    pub fn generate_html_visualization(&self) -> Result<(), Box<dyn std::error::Error>> {
        HtmlVisualizer::generate_html_visualization(self, None)
    }
    
    /// Generate HTML visualization with custom filename
    pub fn generate_html_visualization_with_filename(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        HtmlVisualizer::generate_html_visualization(self, Some(filename))
    }
}

// Usage example in your main function:
/*
fn example_usage(solution: &CalculationResponse) {
    // Print solution to console
    solution.print_solution();
    
    // Generate HTML visualization
    match solution.generate_html_visualization() {
        Ok(_) => println!("HTML визуализация успешно создана"),
        Err(e) => eprintln!("Ошибка создания HTML визуализации: {}", e),
    }
}
*/