-- Drop the table if it exists
IF EXISTS (SELECT * FROM sys.tables WHERE name = 'Employees')
BEGIN
    DROP TABLE Employees;
END

-- Create a sample Employees table
CREATE TABLE Employees (
    EmployeeID INT PRIMARY KEY IDENTITY(1,1),
    FirstName VARCHAR(50),
    LastName VARCHAR(50),
    Department VARCHAR(50),
    Salary DECIMAL(10,2)
);

-- Insert sample records
INSERT INTO Employees (FirstName, LastName, Department, Salary)
VALUES
    ('John', 'Doe', 'IT', 75000.00),
    ('Jane', 'Smith', 'HR', 65000.00),
    ('Mike', 'Johnson', 'Sales', 80000.00),
    ('Sarah', 'Williams', 'Marketing', 70000.00),
    ('Robert', 'Brown', 'IT', 72000.00);
