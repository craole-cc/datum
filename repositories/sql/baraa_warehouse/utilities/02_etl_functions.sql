-- ===============================================================================
-- ETL UTILITY PROCEDURES - DEPLOY ONCE, USE EVERYWHERE
-- ===============================================================================
-- FILE: 01_etl_utilities.sql
-- PURPOSE: Core reusable procedures for all ETL processes
-- DEPLOY: Run this ONCE in your data warehouse database
-- VERSION: 1.0
-- ===============================================================================

USE { DATABASE_NAME } ;
GO

-- Create utilities schema if it doesn't exist
IF NOT EXISTS (SELECT 1
FROM sys.schemas
WHERE name = 'utilities')
EXEC ('CREATE SCHEMA utilities') ;
GO

-- =============================================================================
-- UTILITY: Process Header & Footer Management
-- =============================================================================
IF OBJECT_ID ('utilities.usp_ProcessHeader',
'P') IS NOT NULL DROP PROCEDURE utilities.usp_ProcessHeader ;
GO

CREATE PROCEDURE utilities.usp_ProcessHeader
  @ProcessName NVARCHAR(100),
  @Stage NVARCHAR(20),
  @Version NVARCHAR(20),
  @Description NVARCHAR(500) = NULL,
  @BatchId UNIQUEIDENTIFIER OUTPUT,
  @StartTime DATETIME2 OUTPUT
AS
BEGIN
  SET @BatchId = NEWID();
  SET @StartTime = SYSDATETIME();

  PRINT REPLICATE('═', 80);
  PRINT UPPER(@ProcessName + ' - ' + @Stage + ' LAYER PROCESSING');
  PRINT 'VERSION: ' + @Version + ' | BATCH: ' + CAST(@BatchId AS NVARCHAR(36));
  IF @Description IS NOT NULL PRINT 'PURPOSE: ' + @Description;
  PRINT 'STARTED: ' + FORMAT(@StartTime, 'yyyy-MM-dd HH:mm:ss.fff');
  PRINT REPLICATE('═', 80);
  PRINT '';
END;
GO

IF OBJECT_ID('utilities.usp_ProcessFooter', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_ProcessFooter;
GO

CREATE PROCEDURE utilities.usp_ProcessFooter
  @ProcessName NVARCHAR(100),
  @Stage NVARCHAR(20),
  @StartTime DATETIME2,
  @ErrorOccurred BIT = 0,
  @RowsProcessed INT = NULL,
  @NextStage NVARCHAR(100) = 'next processing stage'
AS
BEGIN
  DECLARE @EndTime DATETIME2 = SYSDATETIME();
  DECLARE @Duration BIGINT = DATEDIFF(MILLISECOND, @StartTime, @EndTime);
  DECLARE @DurationText NVARCHAR(50) =
        CASE
            WHEN @Duration >= 3600000 THEN CAST(@Duration/3600000 AS NVARCHAR(10)) + 'h ' + CAST((@Duration%3600000)/60000 AS NVARCHAR(10)) + 'm'
            WHEN @Duration >= 60000 THEN CAST(@Duration/60000 AS NVARCHAR(10)) + 'm ' + CAST((@Duration%60000)/1000 AS NVARCHAR(10)) + 's'
            WHEN @Duration >= 1000 THEN CAST(@Duration/1000 AS NVARCHAR(10)) + 's'
            ELSE CAST(@Duration AS NVARCHAR(10)) + 'ms'
        END;

  PRINT REPLICATE('═', 80);
  PRINT '*** ' + @ProcessName + ' - ' + @Stage + ' PROCESSING ' + CASE WHEN @ErrorOccurred = 0 THEN 'COMPLETE!' ELSE 'FAILED!' END + ' ***';
  PRINT REPLICATE('═', 80);
  PRINT '🏁 EXECUTION SUMMARY:';
  PRINT '   ├─ Duration: ' + @DurationText;
  IF @RowsProcessed IS NOT NULL PRINT '   ├─ Rows Processed: ' + FORMAT(@RowsProcessed, 'N0');
  PRINT '   ├─ Status: ' + CASE WHEN @ErrorOccurred = 0 THEN 'SUCCESS ✅' ELSE 'FAILED ❌' END;
  PRINT '   └─ Completed: ' + FORMAT(@EndTime, 'yyyy-MM-dd HH:mm:ss.fff');
  PRINT '';
  IF @ErrorOccurred = 0
        PRINT '🚀 Ready for ' + @NextStage + '!';
    ELSE
        PRINT '💥 Check error_log table for details';
  PRINT REPLICATE('═', 80);
END;
GO

-- =============================================================================
-- UTILITY: Step Management
-- =============================================================================
IF OBJECT_ID('utilities.usp_LogStep', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_LogStep;
GO

CREATE PROCEDURE utilities.usp_LogStep
  @StepNumber INT,
  @TotalSteps INT,
  @StepName NVARCHAR(100),
  @Status NVARCHAR(20) = 'EXECUTING',
  @StartTime DATETIME2 OUTPUT
AS
BEGIN
  SET @StartTime = SYSDATETIME();
  PRINT '┌─ STEP ' + FORMAT(@StepNumber, '00') + '/' + FORMAT(@TotalSteps, '00') + ': ' + @StepName;
  PRINT '├─ TIMESTAMP: ' + FORMAT(@StartTime, 'yyyy-MM-dd HH:mm:ss.fff');
  PRINT '└─ STATUS: ' + @Status;
  PRINT '';
END;
GO

IF OBJECT_ID('utilities.usp_CompleteStep', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_CompleteStep;
GO

CREATE PROCEDURE utilities.usp_CompleteStep
  @StartTime DATETIME2,
  @Message NVARCHAR(500) = 'Step completed successfully',
  @RowCount INT = NULL,
  @Status NVARCHAR(10) = '✓'
AS
BEGIN
  DECLARE @Duration BIGINT = DATEDIFF(MILLISECOND, @StartTime, SYSDATETIME());
  DECLARE @DurationText NVARCHAR(50) =
        CASE
            WHEN @Duration >= 60000 THEN CAST(@Duration/60000 AS NVARCHAR(10)) + 'm ' + CAST((@Duration%60000)/1000 AS NVARCHAR(10)) + 's'
            WHEN @Duration >= 1000 THEN CAST(@Duration/1000 AS NVARCHAR(10)) + 's'
            ELSE CAST(@Duration AS NVARCHAR(10)) + 'ms'
        END;

  PRINT '    ' + @Status + ' ' + @Message;
  IF @RowCount IS NOT NULL PRINT '    📊 Rows affected: ' + FORMAT(@RowCount, 'N0');
  PRINT '    ⏱ Duration: ' + @DurationText;
  PRINT '';
END;
GO

-- =============================================================================
-- UTILITY: Dynamic Object Management
-- =============================================================================
IF OBJECT_ID('utilities.usp_CreateTable', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_CreateTable;
GO

CREATE PROCEDURE utilities.usp_CreateTable
  @SchemaName NVARCHAR(50),
  @TableName NVARCHAR(100),
  @ColumnDefinitions NVARCHAR(MAX),
  @DropIfExists BIT = 1,
  @AddAuditColumns BIT = 1,
  @TableType NVARCHAR(20) = 'TABLE'
-- 'TABLE', 'VIEW', 'TEMP'
AS
BEGIN
  DECLARE @FullTableName NVARCHAR(150) = @SchemaName + '.' + @TableName;
  DECLARE @SQL NVARCHAR(MAX);
  DECLARE @StartTime DATETIME2 = SYSDATETIME();

  -- Drop existing object
  IF @DropIfExists = 1
    BEGIN
    IF @TableType = 'VIEW' AND OBJECT_ID(@FullTableName, 'V') IS NOT NULL
        BEGIN
      SET @SQL = 'DROP VIEW ' + @FullTableName;
      EXEC sp_executesql @SQL;
      PRINT '    🧹 Removed existing view: ' + @FullTableName;
    END
        ELSE IF OBJECT_ID(@FullTableName, 'U') IS NOT NULL
        BEGIN
      SET @SQL = 'DROP TABLE ' + @FullTableName;
      EXEC sp_executesql @SQL;
      PRINT '    🧹 Removed existing table: ' + @FullTableName;
    END
  END

  -- Build CREATE statement
  IF @TableType = 'VIEW'
    BEGIN
    SET @SQL = 'CREATE VIEW ' + @FullTableName + ' AS ' + @ColumnDefinitions;
  END
    ELSE
    BEGIN
    SET @SQL = 'CREATE TABLE ' + @FullTableName + ' (' + @ColumnDefinitions;

    -- Add standard audit columns for tables
    IF @AddAuditColumns = 1 AND @TableType = 'TABLE'
        BEGIN
      SET @SQL = @SQL + ',
            created_datetime DATETIME2 DEFAULT SYSDATETIME() NOT NULL,
            modified_datetime DATETIME2 DEFAULT SYSDATETIME() NOT NULL,
            created_by NVARCHAR(256) DEFAULT SUSER_SNAME() NOT NULL,
            process_batch_id UNIQUEIDENTIFIER DEFAULT NEWID() NOT NULL';
    END

    SET @SQL = @SQL + ');';
  END

  -- Execute creation
  EXEC sp_executesql @SQL;

  EXEC utilities.usp_CompleteStep @StartTime, @Message = @FullTableName
  + ' created successfully';
END;
GO

-- =============================================================================
-- UTILITY: Data Quality & Validation
-- =============================================================================
IF OBJECT_ID('utilities.usp_DataQualityCheck', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_DataQualityCheck;
GO

CREATE PROCEDURE utilities.usp_DataQualityCheck
  @SchemaName NVARCHAR(50),
  @TableName NVARCHAR(100),
  @CheckType NVARCHAR(50),
  -- 'COUNT', 'NULLS', 'DUPLICATES', 'FRESHNESS'
  @ColumnName NVARCHAR(100) = NULL,
  @ExpectedValue INT = NULL,
  @Result INT OUTPUT
AS
BEGIN
  DECLARE @FullTableName NVARCHAR(150) = @SchemaName + '.' + @TableName;
  DECLARE @SQL NVARCHAR(MAX);
  DECLARE @Status NVARCHAR(10) = '🔍';

  IF @CheckType = 'COUNT'
    BEGIN
    SET @SQL = 'SELECT @Result = COUNT(*) FROM ' + @FullTableName;
    EXEC sp_executesql @SQL, N'@Result INT OUTPUT', @Result OUTPUT;
    PRINT '    ' + @Status + ' Total records: ' + FORMAT(@Result, 'N0');

    IF @ExpectedValue IS NOT NULL
        BEGIN
      SET @Status = CASE WHEN @Result = @ExpectedValue THEN '✓' ELSE '⚠' END;
      PRINT '    ' + @Status + ' Expected vs Actual: ' + FORMAT(@ExpectedValue, 'N0') + ' vs ' + FORMAT(@Result, 'N0');
    END
  END
    ELSE IF @CheckType = 'NULLS' AND @ColumnName IS NOT NULL
    BEGIN
    SET @SQL = 'SELECT @Result = COUNT(*) FROM ' + @FullTableName + ' WHERE ' + @ColumnName + ' IS NULL';
    EXEC sp_executesql @SQL, N'@Result INT OUTPUT', @Result OUTPUT;
    SET @Status = CASE WHEN @Result = 0 THEN '✓' ELSE '⚠' END;
    PRINT '    ' + @Status + ' Null values in ' + @ColumnName + ': ' + CAST(@Result AS NVARCHAR(10));
  END
    ELSE IF @CheckType = 'DUPLICATES' AND @ColumnName IS NOT NULL
    BEGIN
    SET @SQL = 'SELECT @Result = COUNT(*) - COUNT(DISTINCT ' + @ColumnName + ') FROM ' + @FullTableName;
    EXEC sp_executesql @SQL, N'@Result INT OUTPUT', @Result OUTPUT;
    SET @Status = CASE WHEN @Result = 0 THEN '✓' ELSE '⚠' END;
    PRINT '    ' + @Status + ' Duplicate values in ' + @ColumnName + ': ' + CAST(@Result AS NVARCHAR(10));
  END
    ELSE IF @CheckType = 'FRESHNESS' AND @ColumnName IS NOT NULL
    BEGIN
    SET @SQL = 'SELECT @Result = DATEDIFF(HOUR, MAX(' + @ColumnName + '), GETDATE()) FROM ' + @FullTableName;
    EXEC sp_executesql @SQL, N'@Result INT OUTPUT', @Result OUTPUT;
    SET @Status = CASE WHEN @Result <= 24 THEN '✓' ELSE '⚠' END;
    PRINT '    ' + @Status + ' Data freshness: ' + CAST(@Result AS NVARCHAR(10)) + ' hours old';
  END
END;
GO

-- =============================================================================
-- UTILITY: Error Management
-- =============================================================================
IF OBJECT_ID('utilities.usp_LogError', 'P') IS NOT NULL DROP PROCEDURE utilities.usp_LogError;
GO

CREATE PROCEDURE utilities.usp_LogError
  @Stage NVARCHAR(100),
  @ProcessName NVARCHAR(100) = NULL,
  @BatchId UNIQUEIDENTIFIER = NULL
AS
BEGIN
  -- Ensure error log table exists
  IF OBJECT_ID('dbo.error_log', 'U') IS NULL
    BEGIN
    CREATE TABLE dbo.error_log
    (
      error_id INT IDENTITY PRIMARY KEY,
      error_time DATETIME2 DEFAULT SYSDATETIME() NOT NULL,
      stage NVARCHAR(100) NOT NULL,
      process_name NVARCHAR(100) NULL,
      error_message NVARCHAR(4000) NOT NULL,
      error_procedure NVARCHAR(255) NULL,
      error_line INT NULL,
      user_name NVARCHAR(256) DEFAULT SUSER_SNAME() NOT NULL,
      host_name NVARCHAR(256) DEFAULT HOST_NAME() NOT NULL,
      severity INT NULL,
      state INT NULL,
      batch_id UNIQUEIDENTIFIER NULL
    );
  END

  INSERT INTO dbo.error_log
    (stage, process_name, error_message, error_procedure, error_line, severity, state, batch_id)
  VALUES
    (
      @Stage,
      @ProcessName,
      ERROR_MESSAGE(),
      ISNULL(ERROR_PROCEDURE(), 'Dynamic SQL'),
      ERROR_LINE(),
      ERROR_SEVERITY(),
      ERROR_STATE(),
      @BatchId
    );

  DECLARE @ErrorId INT = SCOPE_IDENTITY();
  PRINT '    ❌ ERROR logged (ID: ' + CAST(@ErrorId AS NVARCHAR(10)) + ')';
  PRINT '    💬 ' + ERROR_MESSAGE();
  PRINT '    📍 Line: ' + CAST(ISNULL(ERROR_LINE(), 0) AS NVARCHAR(10));
END;
GO

PRINT '✅ ETL Utilities deployed successfully!';
GO