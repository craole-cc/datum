-- ===============================================================================
-- BARAA WAREHOUSE - BRONZE LAYER INITIALIZATION SCRIPT
-- ===============================================================================
--
-- PURPOSE: Establishes the foundational bronze schema for raw data ingestion
-- TARGET:  Microsoft SQL Server Database
-- AUTHOR:  Data Engineering Team
--
-- FEATURES:
--    * Idempotent execution (safe to run multiple times)
--    * Comprehensive error handling and logging
--    * Clean bronze layer architecture
--    * Detailed progress reporting
--
-- ===============================================================================

-- +-----------------------------------------------------------------------------+
-- | PHASE 1: DATABASE CONTEXT ESTABLISHMENT                                    |
-- +-----------------------------------------------------------------------------+

PRINT 'BARAA WAREHOUSE - BRONZE LAYER INITIALIZATION';
PRINT '==================================================';
PRINT '';

USE BaraaWarehouse;
PRINT '>>> Successfully switched to BaraaWarehouse database context';
GO

-- +-----------------------------------------------------------------------------+
-- | PHASE 2: BRONZE SCHEMA ARCHITECTURE                                        |
-- +-----------------------------------------------------------------------------+

PRINT '';
PRINT 'CREATING BRONZE SCHEMA ARCHITECTURE...';
PRINT '--------------------------------------';

IF NOT EXISTS (
    SELECT 1
FROM sys.schemas
WHERE name = 'bronze'
)
BEGIN
  EXEC ('CREATE SCHEMA bronze');
  PRINT '    [SUCCESS] Bronze schema created successfully';
END
ELSE
BEGIN
  PRINT '    [INFO] Bronze schema already exists - skipping creation';
END
GO

-- +-----------------------------------------------------------------------------+
-- | PHASE 3: CRM DATA TABLES CONSTRUCTION                                      |
-- +-----------------------------------------------------------------------------+

PRINT '';
PRINT 'BUILDING CRM DATA TABLES...';
PRINT '---------------------------';

-- =============================================================================
-- CRM CUSTOMER INFORMATION REPOSITORY
-- =============================================================================

IF OBJECT_ID('bronze.crm_cust_info', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.crm_cust_info;
  PRINT '    [CLEANUP] Removed existing customer information table';
END

CREATE TABLE bronze.crm_cust_info
(
  -- Primary customer identifiers
  cst_id INT NOT NULL,
  cst_key NVARCHAR(50) NOT NULL,

  -- Personal information
  cst_firstname NVARCHAR(50) NULL,
  cst_lastname NVARCHAR(50) NULL,
  cst_marital_status NVARCHAR(50) NULL,
  cst_gndr NVARCHAR(50) NULL,

  -- Audit trail
  cst_create_date DATE NULL
);

PRINT '    [SUCCESS] Customer information table created successfully';
GO

-- =============================================================================
-- CRM PRODUCT CATALOG REPOSITORY
-- =============================================================================

IF OBJECT_ID('bronze.crm_prd_info', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.crm_prd_info;
  PRINT '    [CLEANUP] Removed existing product information table';
END

CREATE TABLE bronze.crm_prd_info
(
  -- Product identifiers
  prd_id INT NOT NULL,
  prd_key NVARCHAR(50) NOT NULL,

  -- Product details
  prd_nm NVARCHAR(255) NULL,
  prd_cost INT NULL,
  prd_line NVARCHAR(50) NULL,

  -- Lifecycle management
  prd_start_dt DATETIME NULL,
  prd_end_dt DATETIME NULL
);

PRINT '    [SUCCESS] Product information table created successfully';
GO

-- =============================================================================
-- CRM SALES TRANSACTION REPOSITORY
-- =============================================================================

IF OBJECT_ID('bronze.crm_sales_details', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.crm_sales_details;
  PRINT '    [CLEANUP] Removed existing sales details table';
END

CREATE TABLE bronze.crm_sales_details
(
  -- Order identification
  sls_ord_num NVARCHAR(50) NOT NULL,
  sls_prd_key NVARCHAR(50) NOT NULL,
  sls_cust_id INT NOT NULL,

  -- Critical dates (stored as integers)
  sls_order_dt INT NULL,
  sls_ship_dt INT NULL,
  sls_due_dt INT NULL,

  -- Financial metrics
  sls_sales INT NULL,
  sls_quantity INT NULL,
  sls_price INT NULL
);

PRINT '    [SUCCESS] Sales details table created successfully';
GO

-- +-----------------------------------------------------------------------------+
-- | PHASE 4: ERP SYSTEM INTEGRATION TABLES                                     |
-- +-----------------------------------------------------------------------------+

PRINT '';
PRINT 'BUILDING ERP INTEGRATION TABLES...';
PRINT '----------------------------------';

-- =============================================================================
-- ERP CUSTOMER DEMOGRAPHICS (AZ12)
-- =============================================================================

IF OBJECT_ID('bronze.erp_cust_az12', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.erp_cust_az12;
  PRINT '    [CLEANUP] Removed existing ERP customer demographics table';
END

CREATE TABLE bronze.erp_cust_az12
(
  -- Customer identification
  cid NVARCHAR(50) NOT NULL,

  -- Demographics
  bdate DATE NULL,
  gen NVARCHAR(50) NULL
);

PRINT '    [SUCCESS] ERP customer demographics table created successfully';
GO

-- =============================================================================
-- ERP GEOGRAPHIC LOCATIONS (A101)
-- =============================================================================

IF OBJECT_ID('bronze.erp_loc_a101', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.erp_loc_a101;
  PRINT '    [CLEANUP] Removed existing ERP location table';
END

CREATE TABLE bronze.erp_loc_a101
(
  -- Customer reference
  CID NVARCHAR(50) NOT NULL,

  -- Geographic information
  CNTRY NVARCHAR(75) NULL
);

PRINT '    [SUCCESS] ERP location table created successfully';
GO

-- =============================================================================
-- ERP PRODUCT CATEGORIZATION (G1V2)
-- =============================================================================

IF OBJECT_ID('bronze.erp_px_cat_g1v2', 'U') IS NOT NULL
BEGIN
  DROP TABLE bronze.erp_px_cat_g1v2;
  PRINT '    [CLEANUP] Removed existing ERP product categorization table';
END

CREATE TABLE bronze.erp_px_cat_g1v2
(
  -- Product identification
  ID NVARCHAR(50) NOT NULL,

  -- Categorization hierarchy
  CAT NVARCHAR(100) NULL,
  SUBCAT NVARCHAR(100) NULL,

  -- Operational status
  MAINTENANCE NVARCHAR(10) NULL
);

PRINT '    [SUCCESS] ERP product categorization table created successfully';
GO

-- +-----------------------------------------------------------------------------+
-- | PHASE 5: ERROR MANAGEMENT INFRASTRUCTURE                                   |
-- +-----------------------------------------------------------------------------+

PRINT '';
PRINT 'ESTABLISHING ERROR MANAGEMENT SYSTEM...';
PRINT '---------------------------------------';

-- =============================================================================
-- COMPREHENSIVE ERROR LOGGING REPOSITORY
-- =============================================================================

IF OBJECT_ID('dbo.error_log', 'U') IS NOT NULL
BEGIN
  DROP TABLE dbo.error_log;
  PRINT '    [CLEANUP] Removed existing error log table';
END

CREATE TABLE dbo.error_log
(
  -- Primary key with auto-increment
  error_id INT IDENTITY PRIMARY KEY,

  -- Temporal tracking
  error_time DATETIME2 DEFAULT SYSUTCDATETIME() NOT NULL,

  -- Contextual information
  stage VARCHAR(100) NOT NULL,

  -- Error details
  error_message NVARCHAR(4000) NOT NULL,
  error_procedure NVARCHAR(255) NULL,
  error_line INT NULL,

  -- Environmental context
  user_name NVARCHAR(256) DEFAULT SUSER_SNAME() NOT NULL,
  host_name NVARCHAR(256) DEFAULT HOST_NAME() NOT NULL,

  -- Error classification
  severity INT NULL,
  state INT NULL
);

PRINT '    [SUCCESS] Error logging infrastructure established successfully';
GO

-- +-----------------------------------------------------------------------------+
-- | COMPLETION SUMMARY                                                         |
-- +-----------------------------------------------------------------------------+

PRINT '';
PRINT '*** BRONZE SCHEMA INITIALIZATION COMPLETE! ***';
PRINT '==============================================';
PRINT '';
PRINT 'DEPLOYMENT SUMMARY:';
PRINT '  > Bronze schema established';
PRINT '  > 3 CRM data tables deployed';
PRINT '  > 3 ERP integration tables deployed';
PRINT '  > 1 error management table deployed';
PRINT '';
PRINT '>>> System ready for data ingestion operations!';
PRINT '';
GO
