// components/erks/src/context.rs

use crate::prelude::internal::*;
use std::any::Any;
use std::error::Error as StdError;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::{Path, PathBuf};

/// Extension trait with just two methods
pub trait Context<T> {
  /// Attach a plain string context
  fn context<C>(self, ctx: C) -> Result<T>
  where
    C: Into<String>;

  /// Attach both a path and a string context
  fn context_with_path<P, C>(self, path: P, ctx: C) -> Result<T>
  where
    P: Into<PathBuf>,
    C: Into<String>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
  E: StdError + Send + Sync + Any + 'static,
{
  fn context<C>(self, ctx: C) -> Result<T>
  where
    C: Into<String>,
  {
    match self {
      Ok(v) => Ok(v),
      Err(err) => {
        let msg = ctx.into();
        if let Some(ioe) = (&err as &dyn Any).downcast_ref::<IOError>() {
          // no path supplied
          Err(map_io_error_with_context(ioe, &msg))
        } else {
          Err(Error::Context {
            source: Some(Box::new(err)),
            context: msg,
          })
        }
      }
    }
  }

  fn context_with_path<P, C>(self, path: P, ctx: C) -> Result<T>
  where
    P: Into<PathBuf>,
    C: Into<String>,
  {
    match self {
      Ok(v) => Ok(v),
      Err(err) => {
        let path = path.into();
        let msg = ctx.into();
        if let Some(ioe) = (&err as &dyn Any).downcast_ref::<IOError>() {
          Err(map_io_error_with_path(ioe, path, &msg))
        } else {
          Err(Error::Context {
            source: Some(Box::new(err)),
            context: msg,
          })
        }
      }
    }
  }
}

// —— internal mappers —— //

fn map_io_error_with_context(ioe: &IOError, context: &str) -> Error {
  let path = extract_path_from_context(context)
    .unwrap_or_else(|| PathBuf::from("<unknown>"));
  let source = IOError::new(ioe.kind(), format!("{ioe}"));
  dispatch_io(source, path, context)
}

fn map_io_error_with_path(
  ioe: &IOError,
  path: PathBuf,
  context: &str,
) -> Error {
  let source = IOError::new(ioe.kind(), format!("{ioe}"));
  dispatch_io(source, path, context)
}

fn dispatch_io(source: IOError, path: PathBuf, context: &str) -> Error {
  match source.kind() {
    IOErrorKind::NotFound => Error::PathNotFound {
      source,
      path,
      context: context.to_string(),
    },
    IOErrorKind::PermissionDenied => Error::PathPermissionDenied {
      source,
      path,
      context: context.to_string(),
    },
    IOErrorKind::AlreadyExists => Error::PathAlreadyExists {
      source,
      path,
      context: context.to_string(),
    },
    _ => {
      let lc = context.to_lowercase();
      if is_read_operation(&lc) {
        Error::FileRead {
          source,
          path,
          context: context.to_string(),
        }
      } else if is_write_operation(&lc) {
        Error::FileWrite {
          source,
          path,
          context: context.to_string(),
        }
      } else if is_create_operation(&lc) {
        if is_directory_operation(&lc) {
          Error::DirCreate {
            source,
            path,
            context: context.to_string(),
          }
        } else {
          Error::FileCreate {
            source,
            path,
            context: context.to_string(),
          }
        }
      } else if is_delete_operation(&lc) {
        if is_directory_operation(&lc) {
          Error::DirDelete {
            source,
            path,
            context: context.to_string(),
          }
        } else {
          Error::FileDelete {
            source,
            path,
            context: context.to_string(),
          }
        }
      } else {
        Error::Context {
          source: Some(Box::new(source)),
          context: context.to_string(),
        }
      }
    }
  }
}

fn extract_path_from_context(context: &str) -> Option<PathBuf> {
  if let Some(start) = context.find('"') {
    if let Some(end) = context[start + 1..].find('"') {
      return Some(PathBuf::from(&context[start + 1..start + 1 + end]));
    }
  }
  if let Some(start) = context.find('\'') {
    if let Some(end) = context[start + 1..].find('\'') {
      return Some(PathBuf::from(&context[start + 1..start + 1 + end]));
    }
  }
  for token in context.split_whitespace() {
    if token.starts_with('/') && token.len() > 1 {
      return Some(PathBuf::from(token));
    }
    if token.len() >= 3 && token.chars().nth(1) == Some(':') {
      return Some(PathBuf::from(token));
    }
  }
  None
}

fn is_read_operation(ctx: &str) -> bool {
  ctx.contains("read")
    || ctx.contains("open")
    || ctx.contains("load")
    || ctx.contains("parse")
}
fn is_write_operation(ctx: &str) -> bool {
  ctx.contains("write") || ctx.contains("save") || ctx.contains("store")
}
fn is_create_operation(ctx: &str) -> bool {
  ctx.contains("create") || ctx.contains("make") || ctx.contains("new")
}
fn is_delete_operation(ctx: &str) -> bool {
  ctx.contains("delete") || ctx.contains("remove") || ctx.contains("rm")
}
fn is_directory_operation(ctx: &str) -> bool {
  ctx.contains("dir") || ctx.contains("folder") || ctx.contains("directory")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::{Error as IOError, ErrorKind as IOErrorKind};
  use std::path::PathBuf;

  #[test]
  fn generic_context() {
    let r: StdResult<(), IOError> =
      Err(IOError::new(IOErrorKind::NotFound, "oops"));
    let e = r.context("hi").unwrap_err();
    match e {
      Error::Context { context, .. } => assert_eq!(context, "hi"),
      _ => panic!(),
    }
  }

  #[test]
  fn path_context() {
    let r: StdResult<(), IOError> =
      Err(IOError::new(IOErrorKind::NotFound, "oops"));
    let p = PathBuf::from("/foo");
    let e = r.context_with_path(&p, "hello").unwrap_err();
    match e {
      Error::PathNotFound { path, context, .. } => {
        assert_eq!(path, p);
        assert_eq!(context, "hello");
      }
      _ => panic!(),
    }
  }
}
