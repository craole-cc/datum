SELECT
    TOP(1000)[cst_id]
    , [cst_key]
    , [cst_firstname]
    , [cst_lastname]
    , [cst_marital_status]
    , [cst_gndr]
    , [cst_create_date]
FROM [BaraaWarehouse].[bronze].[crm_cust_info]
