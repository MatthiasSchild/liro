use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum("sale_order_status")
                    .values(vec!["draft", "quotation", "order", "finished"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum("sale_invoice_status")
                    .values(vec!["draft", "proforma", "invoice", "finished"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum("purchase_order_status")
                    .values(vec!["draft", "finished"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum("purchase_invoice_status")
                    .values(vec!["draft", "finished"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Email))
                    .col(string(Users::PassHash))
                    .col(string(Users::FirstName))
                    .col(string(Users::LastName))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LedgerAccounts::Table)
                    .if_not_exists()
                    .col(pk_auto(LedgerAccounts::Id))
                    .col(string(LedgerAccounts::AccountType))
                    .col(string(LedgerAccounts::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LedgerTransactions::Table)
                    .if_not_exists()
                    .col(pk_auto(LedgerTransactions::Id))
                    .col(date(LedgerTransactions::Date))
                    .col(integer(LedgerTransactions::Amount))
                    .col(integer(LedgerTransactions::DebitAccountId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_ledger_transactions_debit_account_id")
                            .from("ledger_transactions", "debit_account_id")
                            .to("ledger_accounts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(LedgerTransactions::CreditAccountId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_ledger_transactions_credit_account_id")
                            .from("ledger_transactions", "credit_account_id")
                            .to("ledger_accounts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    .col(pk_auto(Contacts::Id))
                    .col(string(Contacts::Name))
                    .col(boolean(Contacts::IsCustomer))
                    .col(boolean(Contacts::IsSupplier))
                    .col(string(Contacts::Street1))
                    .col(string(Contacts::Street2))
                    .col(string(Contacts::PostalCode))
                    .col(string(Contacts::City))
                    .col(string(Contacts::Country))
                    .col(integer_null(Contacts::CustomerAccountId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_contacts_customer_account_id")
                            .from("contacts", "customer_account_id")
                            .to("ledger_accounts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(Contacts::SupplierAccountId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_contacts_supplier_account_id")
                            .from("contacts", "supplier_account_id")
                            .to("ledger_accounts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(pk_auto(Products::Id))
                    .col(string(Products::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Variants::Table)
                    .if_not_exists()
                    .col(pk_auto(Variants::Id))
                    .col(string(Variants::Name))
                    .col(integer(Variants::SalePrice))
                    .col(integer(Variants::PurchasePrice))
                    .col(integer(Variants::ProductId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_variants_product_id")
                            .from("variants", "product_id")
                            .to("products", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Stocks::Table)
                    .if_not_exists()
                    .col(pk_auto(Stocks::Id))
                    .col(string(Stocks::Name))
                    .col(boolean(Stocks::IsOwn))
                    .col(integer_null(Stocks::OwnerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_stocks_owner_id")
                            .from("stocks", "owner_id")
                            .to("contacts", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StockMoves::Table)
                    .if_not_exists()
                    .col(pk_auto(StockMoves::Id))
                    .col(double(StockMoves::Quantity))
                    .col(integer(StockMoves::VariantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_stock_moves_variant_id")
                            .from("stock_moves", "variant_id")
                            .to("variants", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(integer(StockMoves::SourceStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_stock_moves_source_stock_id")
                            .from("stock_moves", "source_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(integer(StockMoves::TargetStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_stock_moves_target_stock_id")
                            .from("stock_moves", "target_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Taxes::Table)
                    .if_not_exists()
                    .col(pk_auto(Taxes::Id))
                    .col(string(Taxes::Name))
                    .col(string(Taxes::NameShort))
                    .col(double(Taxes::Rate))
                    .col(integer(Taxes::AccountId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_taxes_account_id")
                            .from("taxes", "account_id")
                            .to("ledger_accounts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SaleOrders::Table)
                    .if_not_exists()
                    .col(pk_auto(SaleOrders::Id))
                    .col(string(SaleOrders::Reference))
                    .col(date(SaleOrders::Date))
                    .col(custom(SaleOrders::Status, "sale_order_status"))
                    .col(integer(SaleOrders::CustomerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_orders_customer_id")
                            .from("sale_orders", "customer_id")
                            .to("contacts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(SaleOrders::SourceStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_orders_source_stock_id")
                            .from("sale_orders", "source_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(SaleOrders::TargetStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_orders_target_stock_id")
                            .from("sale_orders", "target_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SaleOrderLines::Table)
                    .if_not_exists()
                    .col(pk_auto(SaleOrderLines::Id))
                    .col(string(SaleOrderLines::Description))
                    .col(double(SaleOrderLines::Quantity))
                    .col(integer(SaleOrderLines::Price))
                    .col(integer(SaleOrderLines::Tax))
                    .col(integer(SaleOrderLines::OrderId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_order_lines_order_id")
                            .from("sale_order_lines", "order_id")
                            .to("sale_orders", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleOrderLines::VariantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_order_lines_variant_id")
                            .from("sale_order_lines", "variant_id")
                            .to("variants", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleOrderLines::MoveId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_order_lines_move_id")
                            .from("sale_order_lines", "move_id")
                            .to("stock_moves", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SaleInvoices::Table)
                    .if_not_exists()
                    .col(pk_auto(SaleInvoices::Id))
                    .col(string(SaleInvoices::Reference))
                    .col(date(SaleInvoices::Date))
                    .col(custom(SaleInvoices::Status, "sale_invoice_status"))
                    .col(integer(SaleInvoices::OrderId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoices_order_id")
                            .from("sale_invoices", "order_id")
                            .to("sale_orders", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(SaleInvoices::CustomerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoices_customer_id")
                            .from("sale_invoices", "customer_id")
                            .to("contacts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SaleInvoiceLines::Table)
                    .if_not_exists()
                    .col(pk_auto(SaleInvoiceLines::Id))
                    .col(string(SaleInvoiceLines::Description))
                    .col(double(SaleInvoiceLines::Quantity))
                    .col(integer(SaleInvoiceLines::Price))
                    .col(integer(SaleInvoiceLines::Tax))
                    .col(integer(SaleInvoiceLines::InvoiceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoice_lines_invoice_id")
                            .from("sale_invoice_lines", "invoice_id")
                            .to("sale_invoices", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleInvoiceLines::VariantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoice_lines_variant_id")
                            .from("sale_invoice_lines", "variant_id")
                            .to("variants", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleInvoiceLines::TransactionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoice_lines_transaction_id")
                            .from("sale_invoice_lines", "transaction_id")
                            .to("ledger_transactions", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleInvoiceLines::TaxId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoice_lines_tax_id")
                            .from("sale_invoice_lines", "tax_id")
                            .to("taxes", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(SaleInvoiceLines::TaxTransactionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_sale_invoice_lines_tax_transaction_id")
                            .from("sale_invoice_lines", "tax_transaction_id")
                            .to("ledger_transactions", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrders::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseOrders::Id))
                    .col(string(PurchaseOrders::Reference))
                    .col(date(PurchaseOrders::Date))
                    .col(custom(PurchaseOrders::Status, "purchase_order_status"))
                    .col(integer(PurchaseOrders::SupplierId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_orders_supplier_id")
                            .from("purchase_orders", "supplier_id")
                            .to("contacts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(PurchaseOrders::SourceStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_orders_source_stock_id")
                            .from("purchase_orders", "source_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(PurchaseOrders::TargetStockId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_orders_target_stock_id")
                            .from("purchase_orders", "target_stock_id")
                            .to("stocks", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrderLines::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseOrderLines::Id))
                    .col(string(PurchaseOrderLines::Description))
                    .col(double(PurchaseOrderLines::Quantity))
                    .col(integer(PurchaseOrderLines::Price))
                    .col(integer(PurchaseOrderLines::Tax))
                    .col(integer(PurchaseOrderLines::OrderId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_order_lines_order_id")
                            .from("purchase_order_lines", "order_id")
                            .to("purchase_orders", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseOrderLines::VariantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_order_lines_variant_id")
                            .from("purchase_order_lines", "variant_id")
                            .to("variants", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseOrderLines::MoveId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_order_lines_move_id")
                            .from("purchase_order_lines", "move_id")
                            .to("stock_moves", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PurchaseInvoices::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseInvoices::Id))
                    .col(string(PurchaseInvoices::Reference))
                    .col(date(PurchaseInvoices::Date))
                    .col(custom(PurchaseInvoices::Status, "purchase_invoice_status"))
                    .col(integer(PurchaseInvoices::OrderId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoices_order_id")
                            .from("purchase_invoices", "order_id")
                            .to("purchase_orders", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer(PurchaseInvoices::SupplierId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoices_supplier_id")
                            .from("purchase_invoices", "supplier_id")
                            .to("contacts", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PurchaseInvoiceLines::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseInvoiceLines::Id))
                    .col(string(PurchaseInvoiceLines::Description))
                    .col(double(PurchaseInvoiceLines::Quantity))
                    .col(integer(PurchaseInvoiceLines::Price))
                    .col(integer(PurchaseInvoiceLines::Tax))
                    .col(integer(PurchaseInvoiceLines::InvoiceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoice_lines_invoice_id")
                            .from("purchase_invoice_lines", "invoice_id")
                            .to("purchase_invoices", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseInvoiceLines::VariantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoice_lines_variant_id")
                            .from("purchase_invoice_lines", "variant_id")
                            .to("variants", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseInvoiceLines::TransactionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoice_lines_transaction_id")
                            .from("purchase_invoice_lines", "transaction_id")
                            .to("ledger_transactions", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseInvoiceLines::TaxId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoice_lines_tax_id")
                            .from("purchase_invoice_lines", "tax_id")
                            .to("taxes", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(integer_null(PurchaseInvoiceLines::TaxTransactionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("rel_purchase_invoice_lines_tax_transaction_id")
                            .from("purchase_invoice_lines", "tax_transaction_id")
                            .to("ledger_transactions", "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseInvoiceLines::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(PurchaseInvoices::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(PurchaseOrderLines::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(PurchaseOrders::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(SaleInvoiceLines::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(SaleInvoices::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(SaleOrderLines::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(SaleOrders::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Taxes::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(StockMoves::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Variants::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Stocks::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(LedgerTransactions::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(LedgerAccounts::Table).to_owned())
            .await
            .ok();

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
            .ok();

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    PassHash,
    FirstName,
    LastName,
}

#[derive(DeriveIden)]
enum LedgerAccounts {
    Table,
    Id,
    AccountType,
    Name,
}

#[derive(DeriveIden)]
enum LedgerTransactions {
    Table,
    Id,
    Date,
    Amount,
    DebitAccountId,
    CreditAccountId,
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    Name,
    IsCustomer,
    IsSupplier,
    Street1,
    Street2,
    PostalCode,
    City,
    Country,
    CustomerAccountId,
    SupplierAccountId,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Variants {
    Table,
    Id,
    Name,
    SalePrice,
    PurchasePrice,
    ProductId,
}

#[derive(DeriveIden)]
enum Stocks {
    Table,
    Id,
    Name,
    IsOwn,
    OwnerId,
}

#[derive(DeriveIden)]
enum StockMoves {
    Table,
    Id,
    Quantity,
    VariantId,
    SourceStockId,
    TargetStockId,
}

#[derive(DeriveIden)]
enum Taxes {
    Table,
    Id,
    Name,
    NameShort,
    Rate,
    AccountId,
}

#[derive(DeriveIden)]
enum SaleOrders {
    Table,
    Id,
    Reference,
    Date,
    Status,
    CustomerId,
    SourceStockId,
    TargetStockId,
}

#[derive(DeriveIden)]
enum SaleOrderLines {
    Table,
    Id,
    Description,
    Quantity,
    Price,
    Tax,
    OrderId,
    VariantId,
    MoveId,
}

#[derive(DeriveIden)]
enum SaleInvoices {
    Table,
    Id,
    Reference,
    Date,
    Status,
    OrderId,
    CustomerId,
}

#[derive(DeriveIden)]
enum SaleInvoiceLines {
    Table,
    Id,
    Description,
    Quantity,
    Price,
    Tax,
    InvoiceId,
    VariantId,
    TransactionId,
    TaxId,
    TaxTransactionId,
}

#[derive(DeriveIden)]
enum PurchaseOrders {
    Table,
    Id,
    Reference,
    Date,
    Status,
    SupplierId,
    SourceStockId,
    TargetStockId,
}

#[derive(DeriveIden)]
enum PurchaseOrderLines {
    Table,
    Id,
    Description,
    Quantity,
    Price,
    Tax,
    OrderId,
    VariantId,
    MoveId,
}

#[derive(DeriveIden)]
enum PurchaseInvoices {
    Table,
    Id,
    Reference,
    Date,
    Status,
    OrderId,
    SupplierId,
}

#[derive(DeriveIden)]
enum PurchaseInvoiceLines {
    Table,
    Id,
    Description,
    Quantity,
    Price,
    Tax,
    InvoiceId,
    VariantId,
    TransactionId,
    TaxId,
    TaxTransactionId,
}
