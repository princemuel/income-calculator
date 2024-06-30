slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |value| {
            let ui = ui_handle.unwrap();

            let income = value.trim().parse::<f64>().unwrap();
            let tax = income * TAXPER;
            let owner = income * OWNERPER;
            let profit = income * PROFITPER;
            let opex = income * OPEXPER;

            let result = format!("Taxes: {tax:.2}\nOwner: {owner:.2}\nProfit: {profit:.2}\nOperating Expenses: {opex:.2}");

            ui.set_results(result.into());
        }
    });

    ui.run()
}
