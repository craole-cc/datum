use super::*;

async fn convert_null_strings_to_nulls(
  df: DataFrame,
  null_values: &[String],
) -> TheResult<DataFrame> {
  let schema = df.schema();
  let mut select_exprs = Vec::new();

  for field in schema.fields() {
    let column_name = field.name();
    let mut expr = col(column_name);

    // For each null value, replace it with actual null
    for null_val in null_values {
      if !null_val.is_empty() {
        expr = when(col(column_name).eq(lit(null_val)), lit_null())
          .otherwise(expr)?;
      }
    }

    select_exprs.push(expr.alias(column_name));
  }

  let result = df
    .select(select_exprs)?
    .collect()
    .await
    .into_diagnostic()
    .wrap_err("Failed to convert null strings to actual nulls")?;

  // Convert back to DataFrame
  let ctx = SessionContext::new();
  let df = ctx.read_batch(result)?;

  Ok(df)
}

// Helper function to create null literal
fn lit_null() -> Expr {
  lit(ScalarValue::Utf8(None))
}
