USE BaraaWarehouse;
GO

EXEC bronze.load_bronze ;
GO

CREATE OR ALTER PROCEDURE bronze.load_bronze
AS
BEGIN
DECLARE
@stage VARCHAR(100);
  DECLARE @startTime DATETIME2, @endTime DATETIME2, @elapsedMs INT;
  BEGIN TRANSACTION

  BEGIN TRY
    PRINT '========== [bronze.load_bronze] Data load started ==========';

    -- CRM TABLES
    PRINT '----- [CRM Data Load] -----';

    -- bronze.crm_cust_info
    SET @stage = 'bronze.crm_cust_info';
    PRINT 'Truncating table: bronze.crm_cust_info ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.crm_cust_info;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.crm_cust_info ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.crm_cust_info
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\cust_info.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    -- bronze.crm_prd_info
    SET @stage = 'bronze.crm_prd_info';
    PRINT 'Truncating table: bronze.crm_prd_info ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.crm_prd_info;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.crm_prd_info ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.crm_prd_info
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\prd_info.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    -- bronze.crm_sales_details
    SET @stage = 'bronze.crm_sales_details';
    PRINT 'Truncating table: bronze.crm_sales_details ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.crm_sales_details;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.crm_sales_details ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.crm_sales_details
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\crm\sales_details.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    -- ERP TABLES
    PRINT '----- [ERP Data Load] -----';

    -- bronze.erp_cust_az12
    SET @stage = 'bronze.erp_cust_az12';
    PRINT 'Truncating table: bronze.erp_cust_az12 ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.erp_cust_az12;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.erp_cust_az12 ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.erp_cust_az12
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\CUST_AZ12.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    -- bronze.erp_loc_a101
    SET @stage = 'bronze.erp_loc_a101';
    PRINT 'Truncating table: bronze.erp_loc_a101 ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.erp_loc_a101;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.erp_loc_a101 ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.erp_loc_a101
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\LOC_A101.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    -- bronze.erp_px_cat_g1v2
    SET @stage = 'bronze.erp_px_cat_g1v2';
    PRINT 'Truncating table: bronze.erp_px_cat_g1v2 ...';
    SET @startTime = SYSDATETIME();
    TRUNCATE TABLE bronze.erp_px_cat_g1v2;
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Truncate completed in ', @elapsedMs, ' ms.');

    PRINT 'Inserting data into: bronze.erp_px_cat_g1v2 ...';
    SET @startTime = SYSDATETIME();
    BULK INSERT bronze.erp_px_cat_g1v2
    FROM 'D:\Projects\GitHub\CC\datum\data\sources\baraa_warehouse\erp\PX_CAT_G1V2.csv'
    WITH (
      FORMAT = 'CSV',
      FIRSTROW = 2,
      FIELDTERMINATOR = ',',
      TABLOCK,
      BATCHSIZE = 10000
    );
    SET @endTime = SYSDATETIME();
    SET @elapsedMs = DATEDIFF(MILLISECOND, @startTime, @endTime);
    PRINT CONCAT('Insert completed in ', @elapsedMs, ' ms. ✔');

    COMMIT TRANSACTION;
    PRINT '========== [bronze.load_bronze] Data load completed successfully ==========';

  END TRY

  BEGIN CATCH
    ROLLBACK TRANSACTION;

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

    PRINT CONCAT('*** ERROR in stage: ', @stage, ' ***');
    PRINT ERROR_MESSAGE();
    ;THROW; -- This re-raises the original error and terminates batch
  END CATCH;
END
