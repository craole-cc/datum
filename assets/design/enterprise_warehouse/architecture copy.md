# Enterprise Data Warehouse Architecture

## 1. High Level Data Architecture

```plantuml
@startuml HighLevelArchitecture

skinparam {
  BackgroundColor      #FAFAFA
  RoundCorner          15
  DefaultTextAlignment center
  componentStyle       uml2
  DefaultFontSize      11
  TitleFontSize        16
  PackageFontSize      14
}

skinparam cloud {
  BackgroundColor #E8F5E8
  BorderColor     #4CAF50
  FontColor       #2E7D32
}

skinparam database {
  BackgroundColor #FFF3E0
  BorderColor     #FF8F00
  FontColor       #E65100
}

skinparam interface {
  BackgroundColor #F3E5F5
  BorderColor     #9C27B0
  FontColor       #6A1B9A
}

skinparam component {
  BackgroundColor #FFFFFF
  BorderColor     #CCCCCC
  FontColor       #333333
}

skinparam package {
  BackgroundColor #F8F9FA
  BorderColor     #6C757D
  FontColor       #495057
}

left to right direction
title <size:18><b>🏗️ High Level Data Architecture</b></size>

cloud "📁 Data Sources" as Sources {
  folder "🏢 ERP System" as ERP {
   component "📄 <b>File Types</b>\n────────────\n📊 CSV Files\n\n📋 <b>Data Content</b>\n────────────\n👥 Customer Reference\n📦 Inventory Reference\n🗺️ Location Reference" as ERPComp
  }
  folder "🤝 CRM System" as CRM {
   component "📄 <b>File Types</b>\n────────────\n📊 CSV Files\n\n📋 <b>Data Content</b>\n────────────\n👤 Customer Information\n🛍️ Product Information\n💰 Sales Details" as CRMComp
  }
}

package "🏭 Data Warehouse [SQL Server]" as Warehouse {
  database "🥉 <b>Bronze Layer</b>\n<i>[Raw Data Extraction]</i>" as Bronze {
   component "🔧 <b>Technical Details</b>\n─────────────────\n📋 <b>Storage:</b> Tables\n\n⚙️ <b>Load Strategy:</b>\n   • 🔄 Batch Processing\n   • 📥 Full Load\n   • 🗑️ Truncate & Insert\n\n🔄 <b>Transformations:</b> None\n📊 <b>Data Modeling:</b> None" as BronzeComp
  }
  
  database "🥈 <b>Silver Layer</b>\n<i>[Data Transformation]</i>" as Silver {
   component "🔧 <b>Technical Details</b>\n─────────────────\n📋 <b>Storage:</b> Tables\n\n⚙️ <b>Load Strategy:</b>\n   • 🔄 Batch Processing\n   • 📥 Full Load\n   • 🗑️ Truncate & Insert\n\n🔄 <b>Transformations:</b>\n   • ⚖️ Standardization\n   • 📏 Normalization\n   • ➕ Column Derivation\n   • 🧹 Cleansing & Enrichment\n\n📊 <b>Data Modeling:</b> None" as SilverComp
  }
  
  database "🥇 <b>Gold Layer</b>\n<i>[Business Ready Data]</i>" as Gold {
   component "🔧 <b>Technical Details</b>\n─────────────────\n📋 <b>Storage:</b> Views\n\n⚙️ <b>Load Strategy:</b> None\n\n🔄 <b>Transformations:</b>\n   • 🔗 Data Integration\n   • 📈 Aggregation\n   • 🧠 Business Logic\n\n📊 <b>Data Modeling:</b>\n   • ⭐ Star Schema\n   • 📋 Flat Tables\n   • 📊 Aggregated Tables" as GoldComp
  }
}

rectangle "📊 Data Consumption" as Consumption {
  interface "📈 <b>BI & Reporting</b>\n────────────\n🔍 Analytics\n📊 Dashboards\n📋 Reports" as BI
  interface "🗃️ <b>SQL Queries</b>\n────────────\n💻 Ad-hoc Analysis\n🔍 Data Exploration\n⚡ Direct Access" as SQL
  interface "🤖 <b>Machine Learning</b>\n────────────\n🧠 AI Models\n🔮 Predictions\n📈 Advanced Analytics" as ML
}

Sources -[#4CAF50,thickness=3]-> Bronze
Bronze -[#FF8F00,thickness=3]-> Silver 
Silver -[#FFC107,thickness=3]-> Gold 
Gold -[#9C27B0,thickness=3]-> BI
Gold -[#9C27B0,thickness=3]-> SQL
Gold -[#9C27B0,thickness=3]-> ML

@enduml
```

## 2. ETL Process Flow

```plantuml
@startuml ETLProcess

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 10
  DefaultTextAlignment center
}

skinparam rectangle {
  BackgroundColor #E8F6F3
  BorderColor #1ABC9C
  FontColor #148F77
}

skinparam process {
  BackgroundColor #FDF2E9
  BorderColor #E67E22
  FontColor #A0522D
}

skinparam storage {
  BackgroundColor #EBF5FB
  BorderColor #3498DB
  FontColor #1B4F72
}

title ETL Process Flow

rectangle "Extract" as Extract {
  process "Data Sources" as Sources
  process "Connection Pooling" as Pool
  process "Data Validation" as Validate
}

rectangle "Transform" as Transform {
  process "Data Cleaning" as Clean
  process "Data Enrichment" as Enrich
  process "Business Rules" as Rules
  process "Data Quality Checks" as Quality
}

rectangle "Load" as Load {
  process "Bulk Insert" as Bulk
  process "Incremental Load" as Incremental
  process "Error Handling" as Error
}

storage "Target DWH" as Target

Sources --> Pool
Pool --> Validate
Validate --> Clean
Clean --> Enrich
Enrich --> Rules
Rules --> Quality
Quality --> Bulk
Quality --> Incremental
Bulk --> Target
Incremental --> Target
Error --> Target

note right of Bulk
  Bulk Insert for initial loads
  and large data volumes
end note

note right of Incremental
  Incremental loads for
  daily/hourly updates
end note

@enduml
```

## 3. Data Warehouse Architecture (Detailed)

```plantuml
@startuml DataWarehouseArchitecture

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 8
  DefaultTextAlignment center
}

skinparam package {
  BackgroundColor #F8F9FA
  BorderColor #6C757D
  FontColor #495057
}

skinparam database {
  BackgroundColor #E3F2FD
  BorderColor #1976D2
  FontColor #0D47A1
}

skinparam component {
  BackgroundColor #FFF3E0
  BorderColor #F57C00
  FontColor #E65100
}

title Data Warehouse Architecture

package "Source Systems" {
  database "CRM Database" as CRM_DB
  database "ERP Database" as ERP_DB
}

package "Staging Area" {
  database "Raw Data Store" as Staging {
    component "CRM Tables" as CRM_Stage
    component "ERP Tables" as ERP_Stage
  }
}


package "Warehouse" {
  database "🥉 Bronze Layer\n(Raw Zone)" as Bronze {
    component "Raw Data\n---\nUnprocessed source data,\npreserves original format" as Bronze_Raw
    component "Full Load\n---\nComplete data snapshots,\nhistorical versioning" as Bronze_Load
  }
  database "🥈 Silver Layer\n(Refined Zone)" as Silver {
    component "Cleaned Data\n---\nValidated, standardized,\nand deduplicated data" as Silver_Clean
    component "Enriched Data\n---\nDerived fields, lookups,\nand business rules applied" as Silver_Enrich
  }
  database "🥇 Gold Layer\n(Curated Zone)" as Gold {
    component "Business Models\n---\nAggregated metrics,\nKPIs, and analytics views" as Gold_Model
    component "Star Schema\n---\nDimensional models for\nreporting and analysis" as Gold_Star
  }
}

' package "Data Warehouse Layers" {
'   database "Bronze Layer" as Bronze {
'     component "Raw Historical Data" as Bronze_Raw
'     component "Data Lineage" as Bronze_Lineage
'   }
  
'   database "Silver Layer" as Silver {
'     component "Cleaned Data" as Silver_Clean
'     component "Standardized Schema" as Silver_Schema
'     component "Data Quality Metrics" as Silver_Quality
'   }
  
'   database "Gold Layer" as Gold {
'     component "Dimension Tables" as Gold_Dim
'     component "Fact Tables" as Gold_Fact
'     component "Aggregated Views" as Gold_Agg
'   }
' }

package "Data Access Layer" {
  component "BI Tools" as BI
  component "Reporting Services" as Reports
  component "Ad-hoc Queries" as Queries
  component "ML Pipelines" as ML
}

CRM_DB --> CRM_Stage
ERP_DB --> ERP_Stage

CRM_Stage --> Bronze_Raw
ERP_Stage --> Bronze_Raw

Bronze_Raw --> Silver_Clean
Silver_Clean --> Gold_Dim
Silver_Clean --> Gold_Fact
Gold_Dim --> Gold_Agg
Gold_Fact --> Gold_Agg

Gold_Agg --> BI
Gold_Agg --> Reports
Gold_Fact --> Queries
Gold_Dim --> ML

@enduml
```

## 4. Data Modeling - Star Schema

```plantuml
@startuml StarSchemaModel

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 8
  DefaultTextAlignment center
}

skinparam class {
  BackgroundColor #FFF8E1
  BorderColor #FF8F00
  FontColor #E65100
}

title Star Schema Data Model

class "Fact_Sales" as FactSales {
  + sales_id : INT
  + customer_key : INT <<FK>>
  + product_key : INT <<FK>>
  + date_key : INT <<FK>>
  + store_key : INT <<FK>>
  --
  + quantity_sold : DECIMAL
  + unit_price : DECIMAL
  + total_amount : DECIMAL
  + discount_amount : DECIMAL
  + profit_margin : DECIMAL
}

class "Dim_Customer" as DimCustomer {
  + customer_key : INT <<PK>>
  --
  + customer_id : VARCHAR
  + first_name : VARCHAR
  + last_name : VARCHAR
  + email : VARCHAR
  + phone : VARCHAR
  + address : VARCHAR
  + city : VARCHAR
  + state : VARCHAR
  + country : VARCHAR
  + customer_segment : VARCHAR
  + registration_date : DATE
}

class "Dim_Product" as DimProduct {
  + product_key : INT <<PK>>
  --
  + product_id : VARCHAR
  + product_name : VARCHAR
  + category : VARCHAR
  + subcategory : VARCHAR
  + brand : VARCHAR
  + supplier : VARCHAR
  + cost_price : DECIMAL
  + list_price : DECIMAL
  + product_status : VARCHAR
}

class "Dim_Date" as DimDate {
  + date_key : INT <<PK>>
  --
  + full_date : DATE
  + day_of_week : VARCHAR
  + day_of_month : INT
  + month : VARCHAR
  + quarter : VARCHAR
  + year : INT
  + is_weekend : BOOLEAN
  + is_holiday : BOOLEAN
  + fiscal_year : INT
  + fiscal_quarter : VARCHAR
}

class "Dim_Store" as DimStore {
  + store_key : INT <<PK>>
  --
  + store_id : VARCHAR
  + store_name : VARCHAR
  + store_type : VARCHAR
  + address : VARCHAR
  + city : VARCHAR
  + state : VARCHAR
  + country : VARCHAR
  + region : VARCHAR
  + manager : VARCHAR
  + opening_date : DATE
}

FactSales "1" -- "*" DimCustomer : customer_key
FactSales "1" -- "*" DimProduct : product_key  
FactSales "1" -- "*" DimDate : date_key
FactSales "1" -- "*" DimStore : store_key

@enduml
```

## 5. Data Integration Process

```plantuml
@startuml DataIntegrationProcess

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 10
  DefaultTextAlignment center
}

skinparam activity {
  BackgroundColor #E8F6F3
  BorderColor #1ABC9C
  FontColor #148F77
}

skinparam decision {
  BackgroundColor #FADBD8
  BorderColor #E74C3C
  FontColor #C0392B
}

skinparam storage {
  BackgroundColor #EBF5FB
  BorderColor #3498DB
  FontColor #1B4F72
}

title Data Integration Process

start

:Source System 1;
:Source System 2;
:Source System N;

fork
  :Extract from CRM;
fork again
  :Extract from ERP;
fork again
  :Extract from APIs;
end fork

:Data Validation;

if (Data Quality Check) then (Pass)
  :Schema Mapping;
  :Data Transformation;
  :Business Rule Application;
  
  if (Integration Method?) then (Full Load)
    :Truncate and Load;
  else (Incremental)
    :Identify Changes;
    :Merge/Upsert;
  endif
  
  :Load to Target;
  :Update Metadata;
  :Data Lineage Tracking;
  
else (Fail)
  :Log Error;
  :Send Alert;
  :Quarantine Data;
  stop
endif

:Success Notification;
stop

@enduml
```

## 6. Separation of Concerns (SOC) Architecture

```plantuml
@startuml SeparationOfConcerns

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 12
  DefaultTextAlignment center
}

skinparam package {
  BackgroundColor #F8F9FA
  BorderColor #6C757D
  FontColor #495057
}

skinparam component {
  BackgroundColor #E3F2FD
  BorderColor #1976D2
  FontColor #0D47A1
}

skinparam interface {
  BackgroundColor #FFF3E0
  BorderColor #F57C00
  FontColor #E65100
}

title Separation of Concerns - Data Warehouse Architecture

package "Data Ingestion Layer" {
  component "Data Extractors" as Extractors
  component "Data Validators" as Validators
  component "Connection Managers" as Connections
  interface "Ingestion API" as IngestionAPI
}

package "Data Processing Layer" {
  component "Data Transformers" as Transformers
  component "Business Rule Engine" as Rules
  component "Data Quality Engine" as Quality
  interface "Processing API" as ProcessingAPI
}

package "Data Storage Layer" {
  component "Bronze Storage" as Bronze
  component "Silver Storage" as Silver
  component "Gold Storage" as Gold
  interface "Storage API" as StorageAPI
}

package "Data Access Layer" {
  component "Query Engine" as QueryEngine
  component "Report Generator" as Reports
  component "API Gateway" as Gateway
  interface "Access API" as AccessAPI
}

package "Data Governance Layer" {
  component "Data Catalog" as Catalog
  component "Security Manager" as Security
  component "Audit Logger" as Audit
  interface "Governance API" as GovernanceAPI
}

package "Orchestration Layer" {
  component "Workflow Manager" as Workflow
  component "Job Scheduler" as Scheduler
  component "Monitoring" as Monitor
  interface "Orchestration API" as OrchestrationAPI
}

Extractors --> IngestionAPI
Validators --> IngestionAPI
Connections --> IngestionAPI

Transformers --> ProcessingAPI
Rules --> ProcessingAPI
Quality --> ProcessingAPI

Bronze --> StorageAPI
Silver --> StorageAPI
Gold --> StorageAPI

QueryEngine --> AccessAPI
Reports --> AccessAPI
Gateway --> AccessAPI

Catalog --> GovernanceAPI
Security --> GovernanceAPI
Audit --> GovernanceAPI

Workflow --> OrchestrationAPI
Scheduler --> OrchestrationAPI
Monitor --> OrchestrationAPI

IngestionAPI --> ProcessingAPI
ProcessingAPI --> StorageAPI
StorageAPI --> AccessAPI

GovernanceAPI --> StorageAPI
GovernanceAPI --> AccessAPI
OrchestrationAPI --> IngestionAPI
OrchestrationAPI --> ProcessingAPI

@enduml
```

## 7. Data Catalog Architecture

```plantuml
@startuml DataCatalogArchitecture

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 10
  DefaultTextAlignment center
}

skinparam database {
  BackgroundColor #E8F5E8
  BorderColor #4CAF50
  FontColor #2E7D32
}

skinparam component {
  BackgroundColor #FFF3E0
  BorderColor #FF8F00
  FontColor #E65100
}

skinparam rectangle {
  BackgroundColor #E3F2FD
  BorderColor #1976D2
  FontColor #0D47A1
}

title Data Catalog Architecture

database "Metadata Repository" as MetadataRepo {
  component "Technical Metadata" as TechMeta
  component "Business Metadata" as BizMeta
  component "Operational Metadata" as OpMeta
  component "Data Lineage" as Lineage
}

rectangle "Data Discovery" as Discovery {
  component "Search Engine" as Search
  component "Browse Interface" as Browse
  component "Recommendation Engine" as Recommend
}

rectangle "Data Profiling" as Profiling {
  component "Schema Discovery" as Schema
  component "Data Quality Metrics" as Metrics
  component "Statistical Analysis" as Stats
}

rectangle "Data Governance" as Governance {
  component "Data Classification" as Classification
  component "Access Control" as Access
  component "Compliance Tracking" as Compliance
}

rectangle "User Interface" as UI {
  component "Web Portal" as Portal
  component "API Gateway" as API
  component "Mobile App" as Mobile
}

rectangle "Data Sources" as Sources {
  component "Data Warehouse" as DWH
  component "Data Lakes" as Lakes
  component "Operational Systems" as OpSys
}

Sources --> Profiling : "Auto-Discovery"
Profiling --> MetadataRepo : "Store Metadata"
MetadataRepo --> Discovery : "Search & Browse"
Discovery --> UI : "User Interface"
Governance --> MetadataRepo : "Policy Enforcement"
UI --> API : "External Access"

@enduml
```

## 8. Bulk Insert vs Normal Insert Process

```plantuml
@startuml BulkInsertComparison

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 8
  DefaultTextAlignment center
}

skinparam rectangle {
  BackgroundColor #E8F6F3
  BorderColor #1ABC9C
  FontColor #148F77
}

skinparam activity {
  BackgroundColor #FDF2E9
  BorderColor #E67E22
  FontColor #A0522D
}

title Bulk Insert vs Normal Insert Process Comparison

rectangle "Normal Insert Process" as NormalInsert {
  activity "Read Record 1" as Read1
  activity "Validate Record 1" as Val1
  activity "Insert Record 1" as Ins1
  activity "Commit Transaction 1" as Com1
  activity "Read Record 2" as Read2
  activity "Validate Record 2" as Val2
  activity "Insert Record 2" as Ins2
  activity "Commit Transaction 2" as Com2
  activity "..." as Dots1
  activity "Read Record N" as ReadN
  activity "Validate Record N" as ValN
  activity "Insert Record N" as InsN
  activity "Commit Transaction N" as ComN
}

rectangle "Bulk Insert Process" as BulkInsert {
  activity "Read All Records" as ReadAll
  activity "Validate All Records" as ValAll
  activity "Prepare Bulk Statement" as Prepare
  activity "Execute Bulk Insert" as Execute
  activity "Single Commit" as SingleCommit
  activity "Error Handling" as ErrorHandle
}

Read1 --> Val1
Val1 --> Ins1
Ins1 --> Com1
Com1 --> Read2
Read2 --> Val2
Val2 --> Ins2
Ins2 --> Com2
Com2 --> Dots1
Dots1 --> ReadN
ReadN --> ValN
ValN --> InsN
InsN --> ComN

ReadAll --> ValAll
ValAll --> Prepare
Prepare --> Execute
Execute --> SingleCommit
SingleCommit --> ErrorHandle

note right of NormalInsert
  **Pros:**
  - Better error handling per record
  - Can process records individually
  - Less memory usage
  
  **Cons:**
  - Slower performance
  - More network round trips
  - Higher transaction overhead
end note

note right of BulkInsert
  **Pros:**
  - Much faster performance
  - Fewer network round trips
  - Lower transaction overhead
  - Better for large datasets
  
  **Cons:**
  - Higher memory usage
  - All-or-nothing approach
  - More complex error handling
end note

@enduml
```

## 9. Dimensions vs Measures

```plantuml
@startuml DimensionsMeasures

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 10
  DefaultTextAlignment center
}

skinparam rectangle {
  BackgroundColor #E8F6F3
  BorderColor #1ABC9C
  FontColor #148F77
}

skinparam class {
  BackgroundColor #FFF3E0
  BorderColor #FF8F00
  FontColor #E65100
}

title Dimensions vs Measures in Data Analytics

rectangle "Dimensions" as Dimensions {
  class "Categorical Data" as Cat {
    + Customer Name
    + Product Category
    + Region
    + Date
    + Store Location
    + Sales Rep
  }
  
  note right of Cat
    **Characteristics:**
    - Qualitative data
    - Used for grouping/filtering
    - Provide context
    - Cannot be aggregated
    - Text or discrete values
  end note
}

rectangle "Measures" as Measures {
  class "Quantitative Data" as Quant {
    + Sales Amount
    + Quantity Sold
    + Profit Margin
    + Discount Amount
    + Cost
    + Revenue
  }
  
  note right of Quant
    **Characteristics:**
    - Quantitative data
    - Can be aggregated (SUM, AVG, etc.)
    - Numeric values
    - Used for calculations
    - Key performance indicators
  end note
}

rectangle "Analysis Examples" as Examples {
  class "Example 1" as Ex1 {
    **Dimensions:** Product Category, Region
    **Measures:** Sales Amount, Quantity
    **Question:** What are the total sales by product category in each region?
  }
  
  class "Example 2" as Ex2 {
    **Dimensions:** Customer Segment, Time Period
    **Measures:** Revenue, Profit Margin
    **Question:** How does profit margin vary by customer segment over time?
  }
}

Dimensions --> Examples : "Provide Context"
Measures --> Examples : "Provide Values"

@enduml
```

## 10. GIT Workflow

```plantuml
@startuml GitWorkflow

skinparam {
  BackgroundColor #FFFFFF
  RoundCorner 8
  DefaultTextAlignment center
}

skinparam rectangle {
  BackgroundColor #E8F6F3
  BorderColor #1ABC9C
  FontColor #148F77
}

skinparam database {
  BackgroundColor #FDF2E9
  BorderColor #E67E22
  FontColor #A0522D
}

skinparam component {
  BackgroundColor #E3F2FD
  BorderColor #1976D2
  FontColor #0D47A1
}

title GIT Workflow for Data Warehouse Development

rectangle "Development Environment" as Dev {
  component "Feature Branch" as Feature
  component "Local Development" as Local
  component "Unit Tests" as UnitTest
}

rectangle "Testing Environment" as Test {
  component "Integration Branch" as Integration
  component "Data Validation" as Validation
  component "Performance Testing" as PerfTest
}

rectangle "Production Environment" as Prod {
  component "Main Branch" as Main
  component "Production Deployment" as Deploy
  component "Monitoring" as Monitor
}

database "Repository Structure" as Repo {
  component "SQL Scripts" as SQL
  component "ETL Pipelines" as ETL
  component "Data Models" as Models
  component "Configuration Files" as Config
  component "Documentation" as Docs
}

Feature --> Integration : "Pull Request"
Integration --> Main : "Merge to Main"
Local --> UnitTest : "Run Tests"
UnitTest --> Feature : "Commit Changes"
Integration --> Validation : "Validate Data"
Validation --> PerfTest : "Performance Check"
Main --> Deploy : "Automated Deployment"
Deploy --> Monitor : "Production Monitoring"

Repo --> Dev : "Checkout Code"
Repo --> Test : "CI/CD Pipeline"
Repo --> Prod : "Release"

note bottom of Repo
  **Repository Contents:**
  - /sql/ddl/ - Table definitions
  - /sql/dml/ - Data manipulation scripts
  - /etl/ - ETL pipeline code
  - /models/ - Data model definitions
  - /config/ - Environment configurations
  - /docs/ - Project documentation
  - /tests/ - Unit and integration tests
end note

@enduml
```

## Usage Instructions

1. **Copy any diagram** from above and paste it into a PlantUML editor
2. **Customize the diagrams** by modifying:
   - Colors and styling (skinparam sections)
   - Component names and descriptions
   - Relationships and connections
   - Notes and annotations

3. **Popular PlantUML editors:**
   - Online: plantuml.com/plantuml
   - VS Code: PlantUML extension
   - IntelliJ: PlantUML integration plugin
   - Standalone: PlantUML jar file

4. **Export formats available:**
   - PNG (images)
   - SVG (scalable vector graphics)
   - PDF (documents)
   - LaTeX (for academic papers)

Each diagram represents a different aspect of your SQL Data Warehouse project and can be used for documentation, presentations, or architectural discussions.