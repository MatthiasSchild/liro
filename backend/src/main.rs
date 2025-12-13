pub mod err;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;

use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(paths(
    handlers::contacts::create_customer,
    handlers::contacts::create_supplier,
    handlers::contacts::delete_customer,
    handlers::contacts::delete_supplier,
    handlers::contacts::get_customer,
    handlers::contacts::get_supplier,
    handlers::contacts::list_customers,
    handlers::contacts::list_suppliers,
    handlers::info::get,
    handlers::ledger_accounts::create,
    handlers::ledger_accounts::delete,
    handlers::ledger_accounts::get,
    handlers::ledger_accounts::list,
    handlers::ledger_transactions::create,
    handlers::ledger_transactions::delete,
    handlers::ledger_transactions::get,
    handlers::ledger_transactions::list,
    handlers::products::create,
    handlers::products::delete,
    handlers::products::get,
    handlers::products::list,
    handlers::purchase_invoice_lines::create,
    handlers::purchase_invoice_lines::delete,
    handlers::purchase_invoice_lines::get,
    handlers::purchase_invoice_lines::list,
    handlers::purchase_invoices::create,
    handlers::purchase_invoices::delete,
    handlers::purchase_invoices::get,
    handlers::purchase_invoices::list,
    handlers::purchase_order_lines::create,
    handlers::purchase_order_lines::delete,
    handlers::purchase_order_lines::get,
    handlers::purchase_order_lines::list,
    handlers::purchase_orders::create,
    handlers::purchase_orders::delete,
    handlers::purchase_orders::get,
    handlers::purchase_orders::list,
    handlers::sale_invoice_lines::create,
    handlers::sale_invoice_lines::delete,
    handlers::sale_invoice_lines::get,
    handlers::sale_invoice_lines::list,
    handlers::sale_invoices::create,
    handlers::sale_invoices::delete,
    handlers::sale_invoices::get,
    handlers::sale_invoices::list,
    handlers::sale_order_lines::create,
    handlers::sale_order_lines::delete,
    handlers::sale_order_lines::get,
    handlers::sale_order_lines::list,
    handlers::sale_orders::create,
    handlers::sale_orders::delete,
    handlers::sale_orders::get,
    handlers::sale_orders::list,
    handlers::stock_moves::create,
    handlers::stock_moves::delete,
    handlers::stock_moves::get,
    handlers::stock_moves::list,
    handlers::stocks::create_contact,
    handlers::stocks::create_own,
    handlers::stocks::delete,
    handlers::stocks::get,
    handlers::stocks::list_contract,
    handlers::stocks::list_own,
    handlers::taxes::create,
    handlers::taxes::delete,
    handlers::taxes::get,
    handlers::taxes::list,
    handlers::users::create,
    handlers::users::delete,
    handlers::users::get,
    handlers::users::list,
    handlers::variants::create,
    handlers::variants::delete,
    handlers::variants::get,
    handlers::variants::list,
))]
struct ApiDoc;

async fn connect_database() -> DatabaseConnection {
    // Connect to database
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(std::env::VarError::NotPresent) => {
            tracing::error!(
                "DATABASE_URL environment variable not set. Please set it in your environment or .env file."
            );
            std::process::exit(1);
        }
        Err(e) => {
            tracing::error!("Error reading DATABASE_URL environment variable: {}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Connecting to database: {}", db_url);
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt.sqlx_logging_level(log::LevelFilter::Debug);
    let db = match Database::connect(db_opt).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Run migrations
    tracing::info!("Running database migrations...");
    match Migrator::up(&db, None).await {
        Ok(_) => tracing::info!("Database migrations completed successfully"),
        Err(e) => {
            tracing::error!("Database migration failed: {}", e);
            std::process::exit(1);
        }
    }

    db
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let app_state = {
        let db = connect_database().await;
        AppState::new(db)
    };

    tracing::info!("Start web server");
    HttpServer::new(move || {
        let swagger = utoipa_swagger_ui::SwaggerUi::new("/docs/{_:.*}")
            .url("/openapi.json", ApiDoc::openapi());

        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(swagger)
            .service(handlers::docs::redirect)
            .service(handlers::contacts::create_customer)
            .service(handlers::contacts::create_supplier)
            .service(handlers::contacts::delete_customer)
            .service(handlers::contacts::delete_supplier)
            .service(handlers::contacts::get_customer)
            .service(handlers::contacts::get_supplier)
            .service(handlers::contacts::list_customers)
            .service(handlers::contacts::list_suppliers)
            .service(handlers::info::get)
            .service(handlers::ledger_accounts::create)
            .service(handlers::ledger_accounts::delete)
            .service(handlers::ledger_accounts::get)
            .service(handlers::ledger_accounts::list)
            .service(handlers::ledger_transactions::create)
            .service(handlers::ledger_transactions::delete)
            .service(handlers::ledger_transactions::get)
            .service(handlers::ledger_transactions::list)
            .service(handlers::products::create)
            .service(handlers::products::delete)
            .service(handlers::products::get)
            .service(handlers::products::list)
            .service(handlers::purchase_invoice_lines::create)
            .service(handlers::purchase_invoice_lines::delete)
            .service(handlers::purchase_invoice_lines::get)
            .service(handlers::purchase_invoice_lines::list)
            .service(handlers::purchase_invoices::create)
            .service(handlers::purchase_invoices::delete)
            .service(handlers::purchase_invoices::get)
            .service(handlers::purchase_invoices::list)
            .service(handlers::purchase_order_lines::create)
            .service(handlers::purchase_order_lines::delete)
            .service(handlers::purchase_order_lines::get)
            .service(handlers::purchase_order_lines::list)
            .service(handlers::purchase_orders::create)
            .service(handlers::purchase_orders::delete)
            .service(handlers::purchase_orders::get)
            .service(handlers::purchase_orders::list)
            .service(handlers::sale_invoice_lines::create)
            .service(handlers::sale_invoice_lines::delete)
            .service(handlers::sale_invoice_lines::get)
            .service(handlers::sale_invoice_lines::list)
            .service(handlers::sale_invoices::create)
            .service(handlers::sale_invoices::delete)
            .service(handlers::sale_invoices::get)
            .service(handlers::sale_invoices::list)
            .service(handlers::sale_order_lines::create)
            .service(handlers::sale_order_lines::delete)
            .service(handlers::sale_order_lines::get)
            .service(handlers::sale_order_lines::list)
            .service(handlers::sale_orders::create)
            .service(handlers::sale_orders::delete)
            .service(handlers::sale_orders::get)
            .service(handlers::sale_orders::list)
            .service(handlers::stock_moves::create)
            .service(handlers::stock_moves::delete)
            .service(handlers::stock_moves::get)
            .service(handlers::stock_moves::list)
            .service(handlers::stocks::create_contact)
            .service(handlers::stocks::create_own)
            .service(handlers::stocks::delete)
            .service(handlers::stocks::get)
            .service(handlers::stocks::list_contract)
            .service(handlers::stocks::list_own)
            .service(handlers::taxes::create)
            .service(handlers::taxes::delete)
            .service(handlers::taxes::get)
            .service(handlers::taxes::list)
            .service(handlers::users::create)
            .service(handlers::users::delete)
            .service(handlers::users::get)
            .service(handlers::users::list)
            .service(handlers::variants::create)
            .service(handlers::variants::delete)
            .service(handlers::variants::get)
            .service(handlers::variants::list)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
