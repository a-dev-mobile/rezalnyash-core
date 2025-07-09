use crate::{
    features::input::{
        models::{panel::Panel, panel_input::PanelInput, stock_input::StockInput},
        traits::dimensions::Dimensions,
    },
    scaled_math::{ScaledConverter, ScaledError},
};

/// Конвертер для преобразования входных моделей в рабочие модели
pub struct InputConverter {
    scale_converter: ScaledConverter,
}

impl InputConverter {
    /// Создает новый конвертер на основе входных данных
    pub fn new(panels: &[PanelInput], stocks: &[StockInput]) -> Result<Self, ScaledError> {
        // Собираем все размеры для определения точности
        let dimensions: Vec<&str> = panels
            .iter()
            .flat_map(|item| item.get_dimensions())
            .chain(stocks.iter().flat_map(|item| item.get_dimensions()))
            .collect();

        let scale_converter = ScaledConverter::from_strings(&dimensions)?;

        Ok(Self { scale_converter })
    }

    /// Возвращает точность конвертера
    pub fn precision(&self) -> u8 {
        self.scale_converter.precision()
    }

    /// Преобразует PanelInput в Panel
    fn convert_panel(&self, input: &PanelInput) -> Result<Panel, ScaledError> {
        let width_scaled = self.scale_converter.convert_string(&input.width)?;
        let height_scaled = self.scale_converter.convert_string(&input.height)?;

        Ok(Panel::new(
            input.id,
            width_scaled.raw_value_u32()?,
            height_scaled.raw_value_u32()?,
            input.count,
            input.label.clone(),
            input.material.clone(),
        ))
    }

    /// Преобразует массив PanelInput в массив Panel
    fn convert_panels(&self, panels_input: &[PanelInput]) -> Result<Vec<Panel>, ScaledError> {
        panels_input
            .iter()
            .map(|panel| self.convert_panel(panel))
            .collect()
    }

    /// Преобразует StockInput в Stock
    fn convert_stock(&self, input: &StockInput) -> Result<Panel, ScaledError> {
        let width_scaled = self.scale_converter.convert_string(&input.width)?;
        let height_scaled = self.scale_converter.convert_string(&input.height)?;

        Ok(Panel::new(
            input.id,
            width_scaled.raw_value_u32()?,
            height_scaled.raw_value_u32()?,
            input.count,
            input.label.clone(),
            input.material.clone(),
        ))
    }

    /// Преобразует массив StockInput в массив Stock
    fn convert_stocks(&self, stocks_input: &[StockInput]) -> Result<Vec<Panel>, ScaledError> {
        stocks_input
            .iter()
            .map(|stock| self.convert_stock(stock))
            .collect()
    }

    /// Комплексное преобразование: возвращает и панели, и заготовки
    pub fn convert_all(
        &self,
        panels_input: &[PanelInput],
        stocks_input: &[StockInput],
    ) -> Result<(Vec<Panel>, Vec<Panel>), ScaledError> {
        let panels = self.convert_panels(panels_input)?;
        let stocks = self.convert_stocks(stocks_input)?;
        Ok((panels, stocks))
    }
}
