--==================================================================
--> BARAA WAREHOUSE - BRONZE LAYER - DATA LOADING PROCEDURE
--==================================================================
-- PURPOSE: Loads raw CSV data into bronze schema tables with comprehensive
--          error handling, performance monitoring, and detailed logging
-- TARGET:  Microsoft SQL Server Database
-- AUTHOR:  Craig Cole
-- FEATURES:
--    * High-performance bulk loading with optimized batch sizes
--    * Comprehensive transaction management and rollback capability
--    * Detailed timing and performance metrics
--    * Robust error handling with centralized logging
--    * Clean, professional output formatting
--    * Data quality tolerance for bronze layer ingestion
--    * Error file cleanup to prevent file existence conflicts
--
--==================================================================

--+----------------------------------------------------------------+
--> PHASE 1: DATABASE CONTEXT ESTABLISHMENT
--+----------------------------------------------------------------+

PRINT 'BARAA WAREHOUSE - BRONZE LAYER INITIALIZATION' ;
PRINT '======================================================' ;
PRINT '' ;
USE BaraaWarehouse ;
PRINT '>>> Successfully switched to BaraaWarehouse database context.' ;
GO

--+----------------------------------------------------------------+
--> PHASE 2: DEPLOYMENT DEFINITION
--+----------------------------------------------------------------+
CREATE OR ALTER PROCEDURE bronze.load_bronze
AS
BEGIN

--==================================================================
-- VARIABLE DECLARATIONS AND INITIALIZATION
--==================================================================

DECLARE
@stage VARCHAR(100);
  DECLARE @startTime DATETIME2, @endTime DATETIME2, @elapsedMs INT;
  DECLARE @totalStartTime DATETIME2 = SYSDATETIME();
  DECLARE @rowsAffected INT;
  DECLARE @totalRowsLoaded INT = 0;
  DECLARE @errorCount INT = 0;
  DECLARE @sql NVARCHAR(MAX);
  DECLARE @timestamp VARCHAR(20) = FORMAT(SYSDATETIME(), 'yyyyMMdd_HHmmss');

  -- Begin main transaction
  BEGIN TRANSACTION LoadBronzeData;

  BEGIN TRY
        --==================================================================
        --> PROCEDURE INITIALIZATION
        --==================================================================

        PRINT '';
        PRINT '*** BRONZE LAYER DATA LOADING INITIATED ***';
        PRINT '===========================================';
        PRINT CONCAT('    Execution started at: ', FORMAT(@totalStartTime, 'yyyy-MM-dd HH:mm:ss.fff'));
        PRINT '';

        --==================================================================
        --> ERROR FILE CLEANUP (PREVENT FILE EXISTS ERRORS)
        --==================================================================

        PRINT 'INITIALIZATION: Cleaning up previous error files...';
        PRINT '';

        -- Clean up error files using xp_cmdshell (requires appropriate permissions)
        EXEC xp_cmdshell 'DEL /Q "D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\*_errors.log" 2>NUL', NO_OUTPUT;
        EXEC xp_cmdshell 'DEL /Q "D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\*.Error.Txt" 2>NUL', NO_OUTPUT;

--+----------------------------------------------------------------+
--> PHASE 3: DATA INGESTION
--+----------------------------------------------------------------+
        --==================================================================
        --> CUSTOMER DATA INGESTION
        --==================================================================

        PRINT 'PHASE 3.1: CUSTOMER DATA INGESTION';
        PRINT '-----------------------------------';
        PRINT '';

        -- ---------------------------------------------------------------------
        -- CRM Customer Information Loading
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.crm_cust_info';
        PRINT '    Processing: CRM Customer Information';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.crm_cust_info;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.crm_cust_info
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\cust_info.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,        -- Allow up to 1000 errors for bronze layer
            KEEPNULLS,               -- Preserve NULL values in source
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\crm_cust_info_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        -- ---------------------------------------------------------------------
        -- CRM Product Information Loading
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.crm_prd_info';
        PRINT '    Processing: CRM Product Information';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.crm_prd_info;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.crm_prd_info
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\prd_info.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,
            KEEPNULLS,
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\crm_prd_info_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        -- ---------------------------------------------------------------------
        -- CRM Sales Details Loading
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.crm_sales_details';
        PRINT '    Processing: CRM Sales Transaction Details';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.crm_sales_details;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.crm_sales_details
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\sales_details.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,
            KEEPNULLS,
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\crm_sales_details_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        --==================================================================
        -- PHASE 2: ERP SYSTEM DATA INGESTION
        --==================================================================

        PRINT 'PHASE 2: ERP SYSTEM DATA INGESTION';
        PRINT '----------------------------------';
        PRINT '';

        -- ---------------------------------------------------------------------
        -- ERP Customer Demographics Loading (AZ12)
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.erp_cust_az12';
        PRINT '    Processing: ERP Customer Demographics (AZ12)';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.erp_cust_az12;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.erp_cust_az12
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\CUST_AZ12.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,
            KEEPNULLS,
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\erp_cust_az12_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        -- ---------------------------------------------------------------------
        -- ERP Geographic Locations Loading (A101)
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.erp_loc_a101';
        PRINT '    Processing: ERP Geographic Locations (A101)';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.erp_loc_a101;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.erp_loc_a101
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\LOC_A101.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,
            KEEPNULLS,
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\erp_loc_a101_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        -- ---------------------------------------------------------------------
        -- ERP Product Categorization Loading (G1V2)
        -- ---------------------------------------------------------------------

        SET @stage = 'bronze.erp_px_cat_g1v2';
        PRINT '    Processing: ERP Product Categorization (G1V2)';
        PRINT '    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~';

        PRINT '        [PREP] Truncating existing data...';
        SET @startTime = SYSDATETIME();
        TRUNCATE TABLE bronze.erp_px_cat_g1v2;
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        PRINT CONCAT('        [DONE] Table cleared in ', @elapsedMs, ' ms');

        PRINT '        [LOAD] Bulk inserting from CSV source...';
        SET @startTime = SYSDATETIME();
        BULK INSERT bronze.erp_px_cat_g1v2
        FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\PX_CAT_G1V2.csv'
        WITH (
            FORMAT = 'CSV',
            FIRSTROW = 2,
            FIELDTERMINATOR = ',',
            ROWTERMINATOR = '\n',
            TABLOCK,
            BATCHSIZE = 10000,
            MAXERRORS = 1000,
            KEEPNULLS,
            ERRORFILE = 'D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\erp_px_cat_g1v2_errors.log'
        );
        SET @endTime = SYSDATETIME();
        SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
        SET @rowsAffected = @@ROWCOUNT;
        SET @totalRowsLoaded += @rowsAffected;
        PRINT CONCAT('        [SUCCESS] ', FORMAT(@rowsAffected, 'N0'), ' records loaded in ', @elapsedMs, ' ms');
        PRINT '';

        --==================================================================
        -- TRANSACTION COMMIT AND COMPLETION SUMMARY
        --==================================================================

        COMMIT TRANSACTION LoadBronzeData;

        DECLARE @totalEndTime DATETIME2 = SYSDATETIME();
        DECLARE @totalElapsedMs INT = DATEDIFF(MILLISECOND, @totalStartTime, @totalEndTime);

        PRINT '*** BRONZE LAYER DATA LOADING COMPLETED SUCCESSFULLY ***';
        PRINT '========================================================';
        PRINT '';
        PRINT 'EXECUTION SUMMARY:';
        PRINT CONCAT('  > Total records loaded: ', FORMAT(@totalRowsLoaded, 'N0'));
        PRINT CONCAT('  > Total execution time: ', @totalElapsedMs, ' ms (',
                     FORMAT(@totalElapsedMs / 1000.0, 'N2'), ' seconds)');
        PRINT CONCAT('  > Average throughput: ',
                     FORMAT(@totalRowsLoaded / CASE WHEN @totalElapsedMs = 0 THEN 1 ELSE @totalElapsedMs END * 1000.0, 'N0'),
                     ' records/second');
        PRINT CONCAT('  > Completion timestamp: ', FORMAT(@totalEndTime, 'yyyy-MM-dd HH:mm:ss.fff'));
        PRINT '';
        PRINT 'NOTE: Any data quality issues have been logged to error files.';
        PRINT 'Check the logs directory for details on rejected records.';
        PRINT '';
        PRINT '>>> Bronze layer ready for silver transformation processes';
        PRINT '';

    END TRY
    BEGIN CATCH
        --==================================================================
        -- COMPREHENSIVE ERROR HANDLING
        --==================================================================

        ROLLBACK TRANSACTION LoadBronzeData;

        -- Log error to centralized error table
        IF OBJECT_ID('dbo.error_log', 'U') IS NOT NULL
        BEGIN
    INSERT INTO dbo.error_log
      (
      stage,
      error_message,
      error_procedure,
      error_line,
      severity,
      state
      )
    VALUES
      (
        @stage,
        ERROR_MESSAGE(),
        ERROR_PROCEDURE(),
        ERROR_LINE(),
        ERROR_SEVERITY(),
        ERROR_STATE()
            );
  END

        -- Display formatted error information
        PRINT '';
        PRINT '*** BRONZE DATA LOADING FAILED ***';
        PRINT '==================================';
        PRINT CONCAT('  > Failed at stage: ', ISNULL(@stage, 'Unknown'));
        PRINT CONCAT('  > Error message: ', ERROR_MESSAGE());
        PRINT CONCAT('  > Error procedure: ', ISNULL(ERROR_PROCEDURE(), 'N/A'));
        PRINT CONCAT('  > Error line: ', ERROR_LINE());
        PRINT CONCAT('  > Error severity: ', ERROR_SEVERITY());
        PRINT CONCAT('  > Error state: ', ERROR_STATE());
        PRINT '';
        PRINT '>>> Transaction has been rolled back - no data changes applied';
        PRINT '';

        -- Re-throw the error to maintain error propagation
        THROW;
    END CATCH;
END
GO

-- +-----------------------------------------------------------------------------+
-- | ALTERNATIVE APPROACH: TIMESTAMPED ERROR FILES                             |
-- +-----------------------------------------------------------------------------+
-- If xp_cmdshell is not available, use this alternative version that creates
-- timestamped error files to avoid conflicts:

/*
CREATE OR ALTER PROCEDURE bronze.load_bronze_timestamped
AS
BEGIN
    -- Same structure as above but with timestamped error files
    DECLARE @timestamp VARCHAR(20) = FORMAT(SYSDATETIME(), 'yyyyMMdd_HHmmss');

    -- Example for first table:
    BULK INSERT bronze.crm_cust_info
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\cust_info.csv'
    WITH (
        FORMAT = 'CSV',
        FIRSTROW = 2,
        FIELDTERMINATOR = ',',
        ROWTERMINATOR = '\n',
        TABLOCK,
        BATCHSIZE = 10000,
        MAXERRORS = 1000,
        KEEPNULLS,
        ERRORFILE = CONCAT('D:\Projects\GitHub\CC\datum\logs\baraa_warehouse\crm_cust_info_errors_', @timestamp, '.log')
    );
    -- Repeat pattern for all tables...
END
*/

-- +-----------------------------------------------------------------------------+
-- | PROCEDURE EXECUTION                                                        |
-- +-----------------------------------------------------------------------------+

PRINT 'EXECUTING BRONZE DATA LOADING PROCEDURE...';
PRINT '==========================================';

EXEC bronze.load_bronze;
GO
