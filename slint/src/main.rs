use std::error::Error;

use production_calendar::{calendar::ProductionCalendar, types::Month};
use production_calendar_loader::{Country, ProductionCalendarLoader};
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let app = ui.as_weak();
    ui.on_calc(move || {
        let ui = app.unwrap();
        match calc(app.clone()) {
            Ok(result) => ui.set_result(result.into()),
            Err(e) => show_error(app.clone(), e.to_string().to_owned()),
        }
    });

    ui.run()
}

fn show_error(app: Weak<AppWindow>, text: String) {
    let ui = app.unwrap();
    ui.set_error_text(text.into());
    ui.invoke_show_error_popup();
}

fn calc(app: Weak<AppWindow>) -> Result<String, Box<dyn Error>> {
    let year = get_year(app.clone())?;
    let month = get_month(app.clone())?;
    let invoice = get_invoice(app.clone())?;

    let calendar: ProductionCalendar =
        ProductionCalendarLoader::new_sync().load(Country::Ru, year)?;

    let prepayment_day_number = calendar.get_prev_work_day(20.into(), month)?.day;
    let prepayment_date = format!(
        "{} {}",
        Into::<u8>::into(prepayment_day_number),
        month_to_word(month)?
    );
    let prepayment: f32 = invoice as f32 / calendar.count_work_days_in_month(month)? as f32
        * calendar.count_work_days_in_month_before(month, 15.into())? as f32;

    let final_payment_day_number = calendar.get_prev_work_day(5.into(), month.next())?.day;
    let final_payment_date = format!(
        "{} {}",
        Into::<u8>::into(final_payment_day_number),
        month_to_word(month.next())?
    );
    let final_payment: f32 = invoice as f32 / calendar.count_work_days_in_month(month)? as f32
        * calendar.count_work_days_in_month_after(month, 15.into())? as f32;

    let result = format!(
        "Дата аванса: {}\nСумма аванса: {:.2} руб.\nДата зарплаты: {}\nСумма зарплаты: {:.2} руб.",
        prepayment_date, prepayment, final_payment_date, final_payment
    );

    Ok(result)
}

fn get_invoice(app: Weak<AppWindow>) -> Result<u32, String> {
    let ui = app.unwrap();
    let invoice = ui.get_invoice();

    match invoice.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Нужно ввести общую сумму".to_owned()),
    }
}

fn get_year(app: Weak<AppWindow>) -> Result<u32, String> {
    let ui = app.unwrap();
    let year = ui.get_year();

    match year.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Нужно выбрать год".to_owned()),
    }
}

fn get_month(app: Weak<AppWindow>) -> Result<Month, String> {
    let ui = app.unwrap();
    let month = ui.get_month();

    match map_to_month(Into::<String>::into(month).as_str()) {
        Ok(value) => Ok(value),
        Err(_) => Err("Нужно выбрать месяц".to_owned()),
    }
}

fn map_to_month(month: &str) -> Result<Month, String> {
    match month {
        "январь" => Ok(Month::January),
        "февраль" => Ok(Month::February),
        "март" => Ok(Month::March),
        "апрель" => Ok(Month::April),
        "май" => Ok(Month::May),
        "июнь" => Ok(Month::June),
        "июль" => Ok(Month::July),
        "август" => Ok(Month::August),
        "сентябрь" => Ok(Month::September),
        "октябрь" => Ok(Month::October),
        "ноябрь" => Ok(Month::November),
        "декабрь" => Ok(Month::December),
        _ => Err("Месяц не распознан".to_owned()),
    }
}

fn month_to_word(month: Month) -> Result<String, String> {
    match month {
        Month::January => Ok("января".to_owned()),
        Month::February => Ok("февраля".to_owned()),
        Month::March => Ok("марта".to_owned()),
        Month::April => Ok("апреля".to_owned()),
        Month::May => Ok("мая".to_owned()),
        Month::June => Ok("июня".to_owned()),
        Month::July => Ok("июля".to_owned()),
        Month::August => Ok("августа".to_owned()),
        Month::September => Ok("сентября".to_owned()),
        Month::October => Ok("октября".to_owned()),
        Month::November => Ok("ноября".to_owned()),
        Month::December => Ok("декабря".to_owned())
    }
}
