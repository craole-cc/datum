#import "../../documentation/template.typ": *

#show: setup-document.with(
  title: "Roadmap: From Data Enthusiast to Data Engineer & Architect",
  author: brand.author,
)

#title-page(
  title: "Data Engineering Roadmap",
  subtitle: "From Enthusiast to Professional",
  icon: icons.roadmap,
  version: "1.0",
)

= Table of Contents

#styled-toc((
  ("Vision & Objectives", <vision>),
  ("Learning Phases", <phases>),
  ("Certification & Industry Readiness", <certification>),
  ("Portfolio & Projects", <portfolio>),
  ("Accountability & Growth Habits", <accountability>),
  ("Resources & References", <resources>),
  ("Progress Tracking", <tracking>),
))

#pagebreak()

= Vision & Objectives <vision>

#info-box(
  title: "Mission Statement",
  icon: icons.learning,
  color: brand.colors.primary,
  [
    Transition into a data engineering role, progress toward data architect, and build a portfolio that proves your readiness for industry jobs.
  ],
)

*Key Outcomes:*
- Master core data engineering skills: databases, pipelines, analytics, visualization, cloud, and architecture essentials
- Earn recognized, practical certifications that signal job readiness
- Develop a robust, modular, public portfolio using #tech-badge("python") and #tech-badge("rust")
- Build professional habits for self-directed growth, accountability, and continuous learning

= Learning Phases <phases>

== Phase 1: Fundamentals & SQL Mastery _(Weeks 1–6)_

#timeline-entry(
  date: "Week 1-2",
  title: "Database Foundations",
  description: "PostgreSQL setup, basic querying, data types",
  status: "todo",
)

#timeline-entry(
  date: "Week 3-4",
  title: "Advanced SQL",
  description: "Joins, subqueries, window functions, CTEs",
  status: "todo",
)

#timeline-entry(
  date: "Week 5-6",
  title: "Data Modeling",
  description: "Schema design, normalization, performance optimization",
  status: "todo",
)

*Focus Areas:*
- #tech-badge("sql") Deepen SQL (PostgreSQL focus, some MongoDB)
- #icons.database *Core Skills:* Data modeling, querying, normalization, schema design
- #icons.folder *Portfolio:* Populate #file-path("python/sqlmastery/") with progressive exercises
- #icons.done *Milestone:* Achieve fluency in reading/writing complex SQL

== Phase 2: Programming for Data Engineering _(Weeks 7–12)_

#timeline-entry(
  date: "Week 7-9",
  title: "Python Data Pipelines",
  description: "Pandas, ETL logic, basic workflow orchestration",
  status: "todo",
)

#timeline-entry(
  date: "Week 10-12",
  title: "Rust for Data",
  description: "Polars, Arrow, sqlx integration, workspace setup",
  status: "todo",
)

*Focus Areas:*
- #tech-badge("python") Pipelines with Pandas, ETL logic, basic Airflow/Prefect exploration
- #tech-badge("rust") Begin Rust ETL/analytics with Polars, Arrow, sqlx
- #icons.git *Version Control:* Solidify git/jj fluency, professional workflows

== Phase 3: Data Engineering in Practice _(Weeks 13–20)_

#info-box(title: "Real-World Projects", icon: icons.project, color: brand.colors.accent, [
  Build actual data pipelines with real datasets. Focus on end-to-end workflows from ingestion to reporting.
])

*Focus Areas:*
- #icons.pipeline *Projects:* Real data pipelines—ingestion, cleaning, transformation, aggregation, reporting
- #tech-badge("docker") *DevOps:* Containerization, deployment scripts, cloud intro (AWS/GCP free tiers)
- #icons.analytics *Portfolio Expansion:* NBA stats, weather data, open datasets—each in distinct subfolder
- #icons.done *Testing & Quality:* Automated tests, linting, documentation in both stacks

== Phase 4: Advanced Topics & Job Readiness _(Weeks 21–30+)_

#timeline-entry(
  date: "Week 21-24",
  title: "Big Data & Distributed Systems",
  description: "Spark basics, PySpark, distributed pipelines",
  status: "todo",
)

#timeline-entry(
  date: "Week 25-27",
  title: "Modern Data Architecture",
  description: "Data warehousing, DBT, ELT patterns, orchestration",
  status: "todo",
)

#timeline-entry(
  date: "Week 28-30",
  title: "Interview Preparation",
  description: "System design, take-home projects, portfolio polish",
  status: "todo",
)

*Focus Areas:*
- #icons.cloud *Big Data:* Spark basics, distributed pipelines (PySpark or Rust/Polars clustering)
- #icons.database *Architectural Patterns:* Data warehousing, modern data stack (DBT, ELT), orchestration
- #icons.learning *System Design:* Architectural decision-making, interview challenges
- #icons.external *Interview Prep:* Take-home tasks, code reviews, resume, LinkedIn polish
- #icons.done *Capstone:* Showcase project demonstrating end-to-end workflow and architectural skills

#pagebreak()

= Certification & Industry Readiness <certification>

#styled-table(
  columns: 3,
  [*Area*],
  [*Recommended Certification(s)*],
  [*Notes*],
  [#tech-badge("sql") Data Engineering],
  [Google Data Engineer, Datacamp SQL, Microsoft DP-900],
  [Free/affordable; check scholarships],
  [#tech-badge("python") Analytics],
  [Datacamp, Coursera, edX microcredentials],
  [Focus on hands-on courses],
  [#tech-badge("cloud") Platforms],
  [AWS Cloud Practitioner, GCP Associate],
  [Start with free tiers/prep materials],
  [#tech-badge("rust") Optional],
  [Open-source project endorsement],
  [Build credibility with strong projects],
)

#info-box(title: "Certification Strategy", icon: icons.done, color: brand.colors.secondary, [
  - Research scholarship/free voucher opportunities frequently
  - Publicly post certificates and course completions in profile/README
  - Focus on hands-on, practical certifications over theoretical ones
])

= Portfolio & Projects <portfolio>

#info-box(title: "Portfolio Structure", icon: icons.folder, color: brand.colors.accent, [
  Each project demonstrates a discrete skill, tool, or concept. Start small and grow systematically.
])

*Repository Structure:*
- #file-path("shared/") — Datasets (public), utilities for repeatable use
- #file-path("python/") — Monorepo: #file-path("sqlmastery/"), analytics projects, pipeline demos
- #file-path("rust/") — Cargo workspace: ETL/analytics projects leveraging Rust-native stack

*Documentation Standards:*
Every project has a README stating objectives, approach, results, and learnings—link back to main roadmap index.

*Project Priorities:*
- #icons.pipeline Data pipeline construction (build, test, run)
- #icons.analytics Real analytics: NBA stats, weather, open government data
- #icons.viz Notebook-driven reports/analysis (where appropriate)

= Accountability & Growth Habits <accountability>

#timeline-entry(
  date: "Weekly",
  title: "Progress Check-ins",
  description: "Log progress and blockers, adjust timelines as needed",
  status: "doing",
)

#timeline-entry(
  date: "Ongoing",
  title: "Issue Tracking",
  description: "Use GitHub's issues/boards for task management",
  status: "doing",
)

*Growth Practices:*
- #icons.learning *Reflection:* Maintain learning journal (#file-path("LEARNING.md"))
- #icons.external *Networking:* Share wins on LinkedIn, blogs, community forums
- #icons.done *Milestone Celebrations:* Acknowledge every achievement, no matter how small

= Resources & References <resources>

#styled-table(
  columns: 2,
  [*Category*],
  [*Resources*],
  [#tech-badge("sql") Databases],
  [Mode SQL tutorials, Postgres docs, MongoDB University],
  [#tech-badge("python") Programming],
  [Real Python, pandas docs, DataCamp],
  [#tech-badge("rust") Systems],
  [Official Rust book, Polars docs, community forums],
  [#icons.cloud Certifications],
  [Google Cloud, Microsoft Learn, AWS Training, Coursera/edX],
  [#icons.learning Interview Prep],
  [LeetCode SQL, system design blogs, project galleries],
)

= Progress Tracking <tracking>

Keep a dated log of achievements. Update regularly and celebrate milestones!

#styled-table(
  columns: 3,
  [*Date*],
  [*Phase/Project*],
  [*Achievement/Notes*],
  [YYYY-MM-DD],
  [SQLMastery started],
  [Set up project, solved 5 exercises],
  [YYYY-MM-DD],
  [Project-2],
  [Pipeline ingests and transforms data],
  [#status-badge("todo", "TBD")],
  [Future milestone],
  [Track your progress here],
)

#align(center)[
  #v(2em)
  #text(size: 14pt, style: "italic", fill: brand.colors.primary)[
    #brand.tagline
  ]
]
